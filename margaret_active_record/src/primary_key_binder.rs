use std::marker::PhantomData;
use std::str::FromStr;

use async_trait::async_trait;

use margaret_database::database::Database;
use margaret_route_parameter_binding::http_route_parameter_binder::HttpRouteParameterBinder;
use margaret_route_parameter_binding::route_parameter_binding_outcome::RouteParameterBindingOutcome;

use crate::continuation::Continuation;
use crate::encodable::Encodable;
use crate::loadable::Loadable;
use crate::lookup::Lookup;
use crate::model::Model;
use crate::narrowed::Narrowed;
use crate::primary_key_clause::primary_key_clause;
use crate::record::Record;
use crate::unguarded::Unguarded;
use crate::unique::Unique;

fn binding_outcome<Loaded>(lookup: Lookup<Loaded>) -> RouteParameterBindingOutcome<Loaded> {
    match lookup {
        Lookup::Found(loaded) => RouteParameterBindingOutcome::Bound(loaded),
        Lookup::Missing => RouteParameterBindingOutcome::NotFound,
    }
}

pub struct PrimaryKeyBinder<'database, Loaded> {
    database: &'database Database,
    loaded: PhantomData<fn() -> Loaded>,
}

impl<'database, Loaded> PrimaryKeyBinder<'database, Loaded> {
    #[must_use]
    pub fn new(database: &'database Database) -> Self {
        Self {
            database,
            loaded: PhantomData,
        }
    }
}

#[async_trait]
impl<Loaded> HttpRouteParameterBinder for PrimaryKeyBinder<'_, Loaded>
where
    Loaded: Loadable,
    Loaded::Root: Model,
    <Loaded::Root as Record>::PrimaryKey: Encodable + FromStr,
    <<Loaded::Root as Record>::PrimaryKey as FromStr>::Err: Send,
{
    type Model = Loaded;

    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<Loaded>> {
        match value.parse::<<Loaded::Root as Record>::PrimaryKey>() {
            Ok(primary_key) => {
                Ok(binding_outcome(
                    Unique::<Loaded::Root, Unguarded>::continued(Narrowed::new(
                        primary_key_clause::<Loaded::Root>(primary_key),
                    ))
                    .loaded(self.database)
                    .await?,
                ))
            }
            Err(_malformed) => Ok(RouteParameterBindingOutcome::NotFound),
        }
    }
}
