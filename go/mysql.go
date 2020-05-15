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
	defer db.Close()

	rows, err := db.Query("show tables")
	if err != nil {
		fmt.Print(err)
	}
	defer rows.Close()
	for rows.Next() {
		var v string
		if err = rows.Scan(&v); err != nil {
			fmt.Print(err)
			return
		}
		fmt.Print(v)
	}
}
