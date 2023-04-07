# bookshop-rs

A simple book store API in need of input validation/sanitization.

This is a part of the University of Wyoming's Secure Software Design Course (Spring 2023). This is the base repository to be forked and updated for various assignments. Alternative language versions are available in:

- [Go](https://github.com/andey-robins/bookshop-go)
- [Javascript](https://github.com/andey-robins/bookshop-js)

## Versioning

`bookshop-rs` is built with:

- cargo 1.70.0-nightly (15d090969 2023-03-21)
- rust edition 2021

## Usage

Start the api using `cargo run`

I recommend using [`httpie`](https://httpie.io) for testing of HTTP endpoints on the terminal. Tutorials are available elsewhere online, and you're free to use whatever tools you deem appropriate for testing your code.

For usage, here is an example: `echo '{"title": "Dune", "author": "Frank Herbert"}' | http GET localhost:8080/books/price` to return the price
## Analysis of Existing Code
There will not be any analysis of the input validation (such as inputting letters for a price) since that is already a known issue by the second part of the assignment.
However, the idea of `Price` alone in the `books` table allowing string input demonstrates how this could be an issue.
The `Price` in the `books` table does not allow strings, yet it is formatted as one when the `create_book` function is called.
Other issues are present from these inputs, but this is just an example of how input validation would be needed, and the issue is already known.

First, in terms of security, there is no logging implemented by default in this software, although there is logging enabled for only the Rocket Crate.
To ensure that the software use can increase Auditability, logging to a file should be implemented.

For the `handlers/books.rs` file, there is a function called `create_book` that allows a user to add a book to a database.
This then makes a database execution which takes the pure values and inputs it into the database.
This could lead to issues with potential SQL injection attacks.
This can be fixed by utilizing the sanitization method for adding to the database that was demonstrated in `deaddrop-rs`.
Similarly, we can also potentially invoke SQL injections for the `get_book_id` and `get_book_price` functions in `db/books.rs`.
Both of these functions can be fixed with the input sanitization in a similar way.
However, the potential to get None from requests is handled by checking the options for the fields for the book.
However, in `handlers/books.rs`, an additional check for if there is None could be implemented when calling `get_book_id` to ensure that an id was returned.

Similar to the `db/books.rs`, both `db/customers.rs` and `purchaseOrders.rs` do not utilize input sanitization when calling the database.
As a result, more SQL injection attacks could be made for any request used in either file.
Additionally, the `get_customer_id` and `get_purchase_order_id` functions do not have their outputs checked in the files in the `/handlers/` folder.

Overall, every request to the database does not have any input sanitization, which allows any potential step to be influenced by a SQL injection attack.
Thus, the sanitization methods used in `deaddrop-rs` should be utilized to mitigate these errors.

The `update_address` function in `customers.rs` could also have securiy issues by not allowing for any authentication.
Any user could input any customer id to change the address.
As a result, any attacker could change a victim's address to the attacker's.
This would allow the attacker to potentially steal an order from a user if they are not aware of their address being changed.
Thus, a verification before an order can be implemented to display the customer's address which would need to be verified before shipping the order.


A potential issue with adding to the books database is that duplicate title/author pairs could be implemented.
There is no checking of duplicates when `create_book` requests the database to be updated with a new book, and that could lead to issues with getting the wrong price in the future.

Finally, there is a raw html request string in `handlers/orders.rs`'s `get_status` function.
This can be modified with the formatted inputs to allow for XSS in the request, which is a vulnerability for the security of the DOM.
This should either not allow for this raw html string to be in the function, or there should be some checks on the inputs to the formatted string before the html string is made.

As for functions that don't work quite as intended, `get_balance` returns a customer with the balance.
This is uninformitive, since the function implies that we only get the balance value, but more is returned.
Some change such as `display_customer_balance` would be more informative, while the return of that function should be changed.
This issue has issues similar in the `get_price` and `get_shipped` functions which are slightly misinformative.