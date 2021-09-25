-- Your SQL goes here
-- diesel setup --database-url='mysql://root:@localhost:3306'

CREATE TABLE account
(
    account_id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    pw VARCHAR(10000) NOT NULL
);

CREATE TABLE image_master
(
    image_id INT AUTO_INCREMENT PRIMARY KEY,
    account_id INT,
    image_base64 LONGTEXT,
    delete_flag TINYINT,
    FOREIGN KEY (account_id) REFERENCES account (account_id)
);