use crate::next_page::NextPage;

pub struct Page<Item, Modeled> {
    pub next: NextPage<Modeled>,
    pub records: Vec<Item>,
}
