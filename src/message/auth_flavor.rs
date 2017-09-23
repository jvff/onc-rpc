#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AuthFlavor {
    None,
    System,
    Short,
    DiffieHelman,
}
