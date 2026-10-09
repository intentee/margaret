use margaret::framework::active_record::key::Key;
use margaret_active_record_tests::models::article_translation::ArticleTranslation;

fn equivalent<Compared: Eq>(left: &Compared, right: &Compared) -> bool {
    left == right
}

fn main() {
    drop(equivalent::<Key<ArticleTranslation>>);
}
