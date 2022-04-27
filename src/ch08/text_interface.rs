use std::collections::HashMap;

type Company = HashMap<String, Vec<String>>;

fn add_employee(company: &mut Company, cmd: &str) {
    let mut tokens = cmd.split_whitespace().peekable();
    let cmd = tokens.next().unwrap();
    if cmd != "Add" {
        return;
    }
    let mut name = Vec::new();
    let to = loop {
        if tokens.peek().is_none() {
            break false;
        }
        if tokens.peek().unwrap().ne(&"to") {
            name.push(tokens.next().unwrap());
            continue
        } else {
            tokens.next();
            break true;
        }
    };
    if to == false {
        return;
    }
    let department = tokens.next().unwrap();
    let people = company.entry(department.to_string()).or_insert(Vec::new());
    (*people).push(name.join(" "));
}

fn people<'a>(company: &'a Company, department: &str) -> &'a Vec<String> {
    company.get(&department.to_string()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_employee() {
        let mut company = Company::new();
        add_employee(&mut company, "Add Bran to Core");
        add_employee(&mut company, "Add Lisa to Core");
        add_employee(&mut company, "Add Jim to Sales");
        let core_people = people(&company, "Core");
        let sales_people = people(&company, "Sales");
        assert_eq!(core_people, &vec![String::from("Bran"), String::from("Lisa")]);
        assert_eq!(sales_people, &vec![String::from("Jim")]);
    }
}


