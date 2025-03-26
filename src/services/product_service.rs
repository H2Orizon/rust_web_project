pub fn get_all_item() -> Vec<String>{
    let items = vec!["Ноутбук", "Смартфон", "Навушники"];
    return items.iter().map(|&item| item.to_string()).collect();
}