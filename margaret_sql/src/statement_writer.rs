use margaret_sql_identifier::qualified_table::qualified_table;
use margaret_sql_identifier::quote_identifier::quote_identifier;

use crate::array_parameter::ArrayParameter;
use crate::assignment::Assignment;
use crate::assignments::Assignments;
use crate::comparison::Comparison;
use crate::condition::Condition;
use crate::conflict_action::ConflictAction;
use crate::conflict_filter::ConflictFilter;
use crate::delete::Delete;
use crate::direction::Direction;
use crate::exists_probe::ExistsProbe;
use crate::expression::Expression;
use crate::from_item::FromItem;
use crate::insert::Insert;
use crate::insert_source::InsertSource;
use crate::insert_value::InsertValue;
use crate::join::Join;
use crate::order_term::OrderTerm;
use crate::returning::Returning;
use crate::select::Select;
use crate::select_filter::SelectFilter;
use crate::select_limit::SelectLimit;
use crate::sql_parameter::SqlParameter;
use crate::statement::Statement;
use crate::table_alias::TableAlias;
use crate::table_source::TableSource;
use crate::unnest_source::UnnestSource;
use crate::update::Update;

fn comparison_operator(comparison: Comparison) -> &'static str {
    match comparison {
        Comparison::Equal => "=",
        Comparison::Greater => ">",
        Comparison::GreaterOrEqual => ">=",
        Comparison::Less => "<",
        Comparison::LessOrEqual => "<=",
    }
}

fn direction_keyword(direction: Direction) -> &'static str {
    match direction {
        Direction::Ascending => "ASC",
        Direction::Descending => "DESC",
    }
}

fn alias_identifier(TableAlias { position }: TableAlias) -> String {
    quote_identifier(&format!("t{position}"))
}

fn positioned_identifier(prefix: &str, position: usize) -> String {
    quote_identifier(&format!("{prefix}{position}"))
}

pub(crate) struct StatementWriter {
    parameters: Vec<SqlParameter>,
    text: String,
}

impl StatementWriter {
    pub(crate) fn new() -> Self {
        Self {
            parameters: Vec::new(),
            text: String::new(),
        }
    }

    pub(crate) fn delete(
        mut self,
        Delete {
            condition,
            returning,
            target,
        }: &Delete,
    ) -> Statement {
        self.text.push_str("DELETE FROM ");
        self.table_source(target);
        self.text.push_str(" WHERE ");
        self.condition(condition);
        self.returning(returning);
        self.finish()
    }

    pub(crate) fn insert(
        mut self,
        Insert {
            conflict,
            returning,
            source,
            target,
            values,
        }: &Insert,
    ) -> Statement {
        self.text.push_str("INSERT INTO ");
        self.table_source(target);
        self.insert_source(values, source);
        self.conflict_action(conflict);
        self.returning(returning);
        self.finish()
    }

    pub(crate) fn select(mut self, select: &Select) -> Statement {
        self.select_body(select);
        self.finish()
    }

    pub(crate) fn update(
        mut self,
        Update {
            assignments,
            condition,
            returning,
            target,
        }: &Update,
    ) -> Statement {
        self.text.push_str("UPDATE ");
        self.table_source(target);
        self.text.push_str(" SET ");
        self.assignments(assignments);
        self.text.push_str(" WHERE ");
        self.condition(condition);
        self.returning(returning);
        self.finish()
    }

    fn array_parameter(
        &mut self,
        ArrayParameter {
            element_type,
            values,
        }: &ArrayParameter,
    ) {
        self.parameter(values);
        self.text.push_str("::");
        self.text.push_str(&element_type.render());
        self.text.push_str("[]");
    }

    fn assignment(&mut self, Assignment { columns, values }: &Assignment) {
        if let [column] = columns.as_slice() {
            self.text.push_str(&quote_identifier(column));
            self.text.push_str(" = ");
            self.expression_list(values);
        } else {
            self.column_list(columns);
            self.text.push_str(" = ");
            self.expression_row(values);
        }
    }

    fn assignments(&mut self, Assignments { first, rest }: &Assignments) {
        self.assignment(first);

        for assignment in rest {
            self.text.push_str(", ");
            self.assignment(assignment);
        }
    }

    fn column_list(&mut self, columns: &[&'static str]) {
        self.text.push('(');

        for (position, column) in columns.iter().enumerate() {
            if position > 0 {
                self.text.push_str(", ");
            }

            self.text.push_str(&quote_identifier(column));
        }

        self.text.push(')');
    }

    fn condition(&mut self, condition: &Condition) {
        match condition {
            Condition::And { left, right } => {
                self.text.push('(');
                self.condition(left);
                self.text.push_str(" AND ");
                self.condition(right);
                self.text.push(')');
            }
            Condition::Compare {
                comparison,
                left,
                right,
            } => {
                self.expression(left);
                self.text.push(' ');
                self.text.push_str(comparison_operator(*comparison));
                self.text.push(' ');
                self.expression(right);
            }
            Condition::Exists(probe) => self.exists_probe(probe),
            Condition::IsNull(expressions) => {
                self.expression_row(expressions);
                self.text.push_str(" IS NULL");
            }
            Condition::Not(negated) => {
                self.text.push_str("NOT (");
                self.condition(negated);
                self.text.push(')');
            }
            Condition::RowCompare {
                comparison,
                left,
                right,
            } => {
                self.expression_row(left);
                self.text.push(' ');
                self.text.push_str(comparison_operator(*comparison));
                self.text.push(' ');
                self.expression_row(right);
            }
        }
    }

    fn conflict_action(&mut self, conflict: &ConflictAction) {
        match conflict {
            ConflictAction::Ignore { target } => {
                self.text.push_str(" ON CONFLICT ");
                self.column_list(target);
                self.text.push_str(" DO NOTHING");
            }
            ConflictAction::Raise => {}
            ConflictAction::Update {
                assignments,
                filter,
                target,
            } => {
                self.text.push_str(" ON CONFLICT ");
                self.column_list(target);
                self.text.push_str(" DO UPDATE SET ");
                self.assignments(assignments);

                if let ConflictFilter::When(condition) = filter {
                    self.text.push_str(" WHERE ");
                    self.condition(condition);
                }
            }
        }
    }

    fn exists_probe(&mut self, ExistsProbe { condition, source }: &ExistsProbe) {
        self.text.push_str("EXISTS (SELECT 1 FROM ");
        self.table_source(source);
        self.text.push_str(" WHERE ");
        self.condition(condition);
        self.text.push(')');
    }

    fn expression(&mut self, expression: &Expression) {
        match expression {
            Expression::Column { alias, column } => {
                self.text.push_str(&alias_identifier(*alias));
                self.text.push('.');
                self.text.push_str(&quote_identifier(column));
            }
            Expression::Excluded { column } => {
                self.text.push_str("EXCLUDED.");
                self.text.push_str(&quote_identifier(column));
            }
            Expression::Greatest { first, second } => {
                self.text.push_str("GREATEST(");
                self.expression(first);
                self.text.push_str(", ");
                self.expression(second);
                self.text.push(')');
            }
            Expression::Parameter(parameter) => self.parameter(parameter),
            Expression::SubqueryColumn { alias, position } => {
                self.text.push_str(&alias_identifier(*alias));
                self.text.push('.');
                self.text.push_str(&positioned_identifier("c", *position));
            }
            Expression::UnnestColumn { alias, position } => {
                self.text.push_str(&alias_identifier(*alias));
                self.text.push('.');
                self.text.push_str(&positioned_identifier("k", *position));
            }
        }
    }

    fn expression_list(&mut self, expressions: &[Expression]) {
        for (position, expression) in expressions.iter().enumerate() {
            if position > 0 {
                self.text.push_str(", ");
            }

            self.expression(expression);
        }
    }

    fn expression_row(&mut self, expressions: &[Expression]) {
        self.text.push('(');
        self.expression_list(expressions);
        self.text.push(')');
    }

    fn finish(self) -> Statement {
        Statement {
            parameters: self.parameters,
            text: self.text,
        }
    }

    fn insert_columns(&mut self, values: &[InsertValue]) {
        self.text.push_str(" (");

        for (position, InsertValue { column, .. }) in values.iter().enumerate() {
            if position > 0 {
                self.text.push_str(", ");
            }

            self.text.push_str(&quote_identifier(column));
        }

        self.text.push(')');
    }

    fn insert_expressions(&mut self, values: &[InsertValue]) {
        for (position, InsertValue { value, .. }) in values.iter().enumerate() {
            if position > 0 {
                self.text.push_str(", ");
            }

            self.expression(value);
        }
    }

    fn insert_source(&mut self, values: &[InsertValue], source: &InsertSource) {
        match source {
            InsertSource::Guarded(condition) => {
                if !values.is_empty() {
                    self.insert_columns(values);
                }

                self.text.push_str(" SELECT");

                if !values.is_empty() {
                    self.text.push(' ');
                    self.insert_expressions(values);
                }

                self.text.push_str(" WHERE ");
                self.condition(condition);
            }
            InsertSource::Values if values.is_empty() => {
                self.text.push_str(" DEFAULT VALUES");
            }
            InsertSource::Values => {
                self.insert_columns(values);
                self.text.push_str(" VALUES (");
                self.insert_expressions(values);
                self.text.push(')');
            }
        }
    }

    fn join(&mut self, join: &Join) {
        match join {
            Join::Inner { on, source } => {
                self.text.push_str(" JOIN ");
                self.table_source(source);
                self.text.push_str(" ON ");
                self.condition(on);
            }
            Join::Lateral { alias, select } => {
                self.text.push_str(" CROSS JOIN LATERAL (");
                self.select_body(select);
                self.text.push_str(") AS ");
                self.text.push_str(&alias_identifier(*alias));
            }
            Join::Left { on, source } => {
                self.text.push_str(" LEFT JOIN ");
                self.table_source(source);
                self.text.push_str(" ON ");
                self.condition(on);
            }
        }
    }

    fn order_term(
        &mut self,
        OrderTerm {
            direction,
            expression,
        }: &OrderTerm,
    ) {
        self.expression(expression);
        self.text.push(' ');
        self.text.push_str(direction_keyword(*direction));
    }

    fn parameter(&mut self, parameter: &SqlParameter) {
        self.parameters.push(parameter.clone());
        self.text.push('$');
        self.text.push_str(&self.parameters.len().to_string());
    }

    fn returning(&mut self, returning: &Returning) {
        if let Returning::Columns(columns) = returning {
            self.text.push_str(" RETURNING ");
            self.expression_list(columns);
        }
    }

    fn select_body(
        &mut self,
        Select {
            columns,
            filter,
            from,
            joins,
            limit,
            ordering,
        }: &Select,
    ) {
        self.text.push_str("SELECT ");

        for (position, column) in columns.iter().enumerate() {
            if position > 0 {
                self.text.push_str(", ");
            }

            self.expression(column);
            self.text.push_str(" AS ");
            self.text.push_str(&positioned_identifier("c", position));
        }

        self.text.push_str(" FROM ");
        self.source_item(from);

        for join in joins {
            self.join(join);
        }

        if let SelectFilter::Matching(condition) = filter {
            self.text.push_str(" WHERE ");
            self.condition(condition);
        }

        for (position, term) in ordering.iter().enumerate() {
            self.text
                .push_str(if position == 0 { " ORDER BY " } else { ", " });
            self.order_term(term);
        }

        if let SelectLimit::Rows(rows) = limit {
            self.text.push_str(" LIMIT ");
            self.text.push_str(&rows.to_string());
        }
    }

    fn source_item(&mut self, from: &FromItem) {
        match from {
            FromItem::Table(source) => self.table_source(source),
            FromItem::Unnest(unnest) => self.unnest_source(unnest),
        }
    }

    fn table_source(
        &mut self,
        TableSource {
            alias,
            namespace,
            table,
        }: &TableSource,
    ) {
        self.text.push_str(&qualified_table(*namespace, table));
        self.text.push_str(" AS ");
        self.text.push_str(&alias_identifier(*alias));
    }

    fn unnest_source(&mut self, UnnestSource { alias, arrays }: &UnnestSource) {
        self.text.push_str("unnest(");

        for (position, array) in arrays.iter().enumerate() {
            if position > 0 {
                self.text.push_str(", ");
            }

            self.array_parameter(array);
        }

        self.text.push_str(") AS ");
        self.text.push_str(&alias_identifier(*alias));
        self.text.push('(');

        for position in 0..arrays.len() {
            if position > 0 {
                self.text.push_str(", ");
            }

            self.text.push_str(&positioned_identifier("k", position));
        }

        self.text.push(')');
    }
}
