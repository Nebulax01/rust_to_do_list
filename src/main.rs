use std::io;

struct Task {
    id: usize,
    name: String,
    status: bool,
}

fn show_options() {
    println!("1. View current tasks");
    println!("2. Add a new task");
    println!("3. Remove a task");
    println!("4. Mark Task as done");
    println!("5. Exit");
}

fn print_task(task: &Task) {
    println!(
        "id: {}, name: {}, status: {}",
        task.id,
        task.name,
        if task.status { "Done" } else { "Pending" }
    );
}

fn view_current_tasks(list: &Vec<Task>) {
    for task in list {
        print_task(task);
    }
}

fn add_new_task(list: &mut Vec<Task>) {

    println!("Input task description");
    let mut task_name = String::new();

    io::stdin()
        .read_line(&mut task_name)
        .expect("Failed to read line");

    let task = Task {
        id: list.len(),
        name: task_name.trim().to_string(),
        status: false,
    };

    list.push(task);
}

fn remove_task(list: &mut Vec<Task>, task_id: usize) {
    if task_id < list.len() {
        list.remove(task_id);
    } else {
        println!("Task with index {} not found!", task_id);
    }
}

fn mark_done_task(list: &mut Vec<Task>, task_id: usize) {
    match list.get_mut(task_id) {
        Some(task) => {
            task.status = true;
            println!("Task marked done");
        },
        None => println!("Task with index {} not found!", task_id),
    }
}

fn input_and_validate_task_id() -> usize {
    loop {
        let mut task_id = String::new();

        io::stdin()
            .read_line(&mut task_id)
            .expect("Failed to read line");

        match task_id.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please type a number!"),
        }
    }
}

fn main() {
    let mut list: Vec<Task> = Vec::new();

    loop {
        show_options();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => view_current_tasks(&list),
            "2" => add_new_task(&mut list),
            "3" => {
                println!("Input task ID that you wish to be removed");
                let task_remove_id = input_and_validate_task_id();
                remove_task(&mut list, task_remove_id);
            }
            "4" => {
                let mark_done_task_id = input_and_validate_task_id();
                mark_done_task(&mut list, mark_done_task_id);
            }
            "5" => break,
            _ => {
                println!("pick a valid option from the list!");
                continue;
            }
        }
    }
}