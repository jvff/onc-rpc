#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AuthFlavor {
    Null,
    Unix,
    Short,
    Des,
}
