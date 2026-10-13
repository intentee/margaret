use crate::next_page::NextPage;

pub struct Page<Item, Modeled, Ordering, Toward> {
    pub next: NextPage<Modeled, Ordering, Toward>,
    pub records: Vec<Item>,
}
