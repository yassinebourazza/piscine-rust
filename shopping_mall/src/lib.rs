pub mod mall;
pub use mall::*;
use std::collections::HashMap;

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    let mut best_name = String::new();
    let mut best_store = None;
    let mut max_size = 0;

    for floor in mall.floors.values() {
        for (store_name, store) in &floor.stores {
            if store.square_meters > max_size {
                max_size = store.square_meters;
                best_name = store_name.clone();
                best_store = Some(store.clone());
            }
        }
    }

    (best_name, best_store.unwrap())
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&String, &Employee)> {
    let mut max_salary = 0.0;
    let mut result = Vec::new();

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for employee in store.employees.values() {
                if employee.salary > max_salary {
                    max_salary = employee.salary
                }
            }
        }
    }

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for (name, employee) in &store.employees {
                if employee.salary == max_salary {
                    result.push((name, employee));
                }
            }
        }
    }

    result

}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut nb_person = 0;

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            nb_person += store.employees.len();
        }
    }

    nb_person += mall.guards.len();

    nb_person
}

pub fn check_for_securities(mall: &mut Mall, guards: HashMap<String, Guard>) {
    let mut n_guard = mall.guards.len();
    let mut total_area: u64 = 0;
    for (_, store) in &mall.floors {
        total_area += store.size_limit;
    }
    let number_quards: u64 = total_area / 200;

    for (guard_name, info) in guards {
        mall.hire_guard(guard_name.clone(), info);
        n_guard += 1;
        if n_guard == number_quards.try_into().unwrap() {
            break;
        }
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for employee in store.employees.values_mut() {
                let working_hours = employee.working_hours.1 - employee.working_hours.0;
                let ten_percente = employee.salary / 10.0;

                if working_hours >= 10 {
                    employee.raise(ten_percente);
                } else {
                    employee.cut(ten_percente);
                }
            }
        }
    }
}