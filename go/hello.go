package main

import (
	"database/sql"
	"fmt"

	_ "github.com/go-sql-driver/mysql"
)

func sum(s []int, c chan int) {

}

func main() {
	dsn := fmt.Sprintf("root@tcp(172.16.111.11:33000)/test")
	db, e := sql.Open("mysql", dsn)
	if e != nil {
		fmt.Printf("%v", e)
	}
	_, e = db.Exec("set @@tidb_replica_read = \"leader\"")
	if e != nil {
		fmt.Printf("e != nil")
	}

	fmt.Println("prepare..")
	prepare(db)

	fmt.Println("select..")
	for i := 0; i < 1000000; i++ {
		db.Exec("select * from x")
	}
}

func prepare(db *sql.DB) {
	for i := 0; i < 10000; i++ {
		db.Exec(fmt.Sprintf("insert into x (a) values (%v)", i))
	}
}
