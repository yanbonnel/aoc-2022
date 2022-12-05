

pub fn day2() {
    let mut score_step1 = 0;
    let mut score_step2 = 0;
    for line in include_str!("./day2.txt")
        .split("\n") {

        let mut fields = line.split(" ");
        let op = fields.next().unwrap();
        let me = fields.next().unwrap();

        score_step1 = score_step1 + shap_score(me) + win_score(op, me);

        let me_step2 = match (op, me) {
            ("A", "X") => "Z",
            ("A", "Y") => "X",
            ("A", "Z") => "Y",
            ("B", "X") => "X",
            ("B", "Y") => "Y",
            ("B", "Z") => "Z",
            ("C", "X") => "Y",
            ("C", "Y") => "Z",
            ("C", "Z") => "X",
            _ => unimplemented!()
        };

        score_step2 = score_step2 + shap_score(me_step2) + win_score(op, me_step2);


    }

    println!("Step 1 : {}", score_step1);
    println!("Step 2 : {}", score_step2);



}

fn win_score(op: &str, me: &str) -> i32 {
    match (op, me) {
        ("A", "X") => 3,
        ("A", "Y") => 6,
        ("A", "Z") => 0,
        ("B", "X") => 0,
        ("B", "Y") => 3,
        ("B", "Z") => 6,
        ("C", "X") => 6,
        ("C", "Y") => 0,
        ("C", "Z") => 3,
        _ => unimplemented!()
    }
}

fn shap_score(me: &str) -> i32 {
    match me {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => unimplemented!()
    }
}
