/*
Copyright © 2022 NAME HERE <EMAIL ADDRESS>

*/
package main

import (
	"fmt"

	"github.com/Hugobyte/aurras/workflow/composer/cmd"
)

var Blue = Color("\033[1;34m%s\033[0m")

func Color(colorString string) func(...interface{}) string {
	sprint := func(args ...interface{}) string {
		return fmt.Sprintf(colorString,
			fmt.Sprint(args...))
	}
	return sprint
}

const banner = `  
  _________  __  ______  ____  ___________ 
 / ___/ __ \/  |/  / _ \/ __ \/ __/ __/ _ \
/ /__/ /_/ / /|_/ / ___/ /_/ /\ \/ _// , _/
\___/\____/_/  /_/_/   \____/___/___/_/|_| 
					
			HugoByte AI Labs
`

func main() {
	fmt.Println(Blue(banner))
	cmd.Execute()
}
