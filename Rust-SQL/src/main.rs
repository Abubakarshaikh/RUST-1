#[macro_use]
extern crate mysql;
// ...

use mysql as my;

#[derive(Debug, PartialEq, Eq)]
struct Student {
    // sid: i32,
    name: Option<String>,
    email: Option<String>,
    age: Option<String>,
}

fn main() {
    // See docs on the `OptsBuilder`'s methods for the list of options available via URL.
    let pool = my::Pool::new("mysql://root:tahiri1234@localhost:3307/rust").unwrap();
    


//     // Let's create payment table.
//     // Unwrap just to make sure no error happened.
//     pool.prep_exec(r"CREATE TEMPORARY TABLE payment (
//                          customer_id int not null,
//                          amount int not null,
//                          account_name text
//                      )", ()).unwrap();

    let students = vec![
        Student { name: Some("2".into()), email: Some("emai".into()) ,age:Some("23".into()) }];
//         Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
//         Payment { customer_id: 5, amount: 6, account_name: None },
//         Payment { customer_id: 7, amount: 8, account_name: None },
//         Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
//     ];

    // Let's insert payments to the database
    // We will use into_iter() because we do not need to map Stmt to anything else.
    // Also we assume that no error happened in `prepare`.
    for mut stmt in pool.prepare(r"INSERT INTO tblstudent
                                       ( name, email,age)
                                   VALUES
                                       ( :name, :email ,:age)").into_iter() {
        for p in students.iter() {
            // `execute` takes ownership of `params` so we pass account name by reference.
            // Unwrap each result just to make sure no errors happened.
            stmt.execute(params!{
                // "sid" => p.sid,
                "name" => &p.name,
                "email" => &p.email,
                "age" => &p.age,
            }).unwrap();
        }
    }

//     // Let's select payments from database
//     let selected_payments: Vec<Payment> =
//     pool.prep_exec("SELECT customer_id, amount, account_name from payment", ())
//     .map(|result| { // In this closure we will map `QueryResult` to `Vec<Payment>`
//         // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
//         // will map each `MyResult` to contained `row` (no proper error handling)
//         // and second call to `map` will map each `row` to `Payment`
//         result.map(|x| x.unwrap()).map(|row| {
//             // ⚠️ Note that from_row will panic if you don't follow your schema
//             let (customer_id, amount, account_name) = my::from_row(row);
//             Payment {
//                 customer_id: customer_id,
//                 amount: amount,
//                 account_name: account_name,
//             }
//         }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
//     }).unwrap(); // Unwrap `Vec<Payment>`

//     // Now make sure that `payments` equals to `selected_payments`.
//     // Mysql gives no guaranties on order of returned rows without `ORDER BY`
//     // so assume we are lukky.
//     assert_eq!(payments, selected_payments);
//     println!("Yay!");
// }
}