package main

import (
	"database/sql"
	"fmt"

	_ "github.com/go-sql-driver/mysql"
)

var (
	ip   string = "172.16.4.137"
	port int    = 33000
	db   string = "test"
)

func main() {
	fmt.Println("Opening...")
	db, err := sql.Open("mysql", fmt.Sprintf("root@tcp(%s:%d)/%s", ip, port, db))
	if err != nil {
		fmt.Print(err)
	}
	_, err = db.Exec("show tables")
	if err != nil {
		fmt.Print(err)
	}
	defer db.Close()
}
