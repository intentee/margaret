use crate::next_page::NextPage;

pub struct Page<Item, Modeled, Ordering> {
    pub next: NextPage<Modeled, Ordering>,
    pub records: Vec<Item>,
}
