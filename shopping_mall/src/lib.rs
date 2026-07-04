pub mod mall;
pub use crate::mall::{Mall, Guard, Store , Employee , Floor};
use std::collections::HashMap;

pub fn biggest_store(m: &Mall) -> (String, Store) {
    let mut count = 0;
    let mut res = None;
    let mut exp_name = String::new();
    for (_n,floor) in m.floors.iter() {
        for (name,store) in floor.stores.iter() {
            if count < store.square_meters {
                res = Some(store);
                exp_name = name;
                count = store.square_meters;
            }
        }
    }
    (exp_name,res.unwrap().clone())
}

pub fn highest_paid_employee(m: &Mall) -> Vec<(String,Employee)> {
    let mut vec = Vec::new();
    let mut res = None;
    let mut count: f64 = 0.0;
    let mut exp_name = String::new();

    for (_n,floor) in m.floors.iter() {
        for (_n1,store) in floor.stores.iter() {
            for (name,emp) in store.employees.iter() {
                if count < emp.salary {
                    exp_name = name.clone();
                    res = Some(emp);
                    count = emp.salary;
                }
            }
        }
    }
    vec.push((exp_name,res.unwrap().clone()));
    return vec

}

pub fn nbr_of_employees(m: &Mall) -> usize {
    let mut count = 0;
    for (_n,floor) in m.floors.iter() {
        for (_n1,store) in floor.stores.iter() {
            for (_name,_emp) in store.employees.iter() {
                count += 1
            }
        }
    }

    for (_n,guard) in m.guards.iter() {
        count += 1
    }
    count
}

pub fn check_for_securities(m: &mut Mall, new_guard: HashMap<String, Guard>) {
    let mut mallsize = 0;
    for (_n,floor) in m.floors.iter() {
        mallsize += floor.size_limit as usize;
    }

    mallsize = (mallsize / 200) as usize;

    let curguard = m.guards.len();
    let mut count = 0;
    for (name,guard) in new_guard {
        count += 1;

        if mallsize < count + curguard {
        break
    }
        m.hire_guard(name, guard)
    }
    
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for employee in store.employees.values_mut() {
                let (start, end) = employee.working_hours;
                let hours_worked = if end >= start {
                    end - start
                } else {
                    end + 24 - start
                };
                if hours_worked > 10 {
                    employee.salary = (employee.salary * 11.0) / 10.0;
                } else if hours_worked < 10 {
                    employee.salary = (employee.salary * 9.0) / 10.0;
                }
            }
        }
    }
}
