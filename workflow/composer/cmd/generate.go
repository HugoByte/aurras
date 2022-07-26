/*
Copyright © 2022 NAME HERE <EMAIL ADDRESS>

*/
package cmd

import (
	"fmt"
	"io"
	"io/ioutil"
	"os"
	"os/exec"
	"path"

	"github.com/spf13/cobra"
)

var providerPath string
var config string

// generateCmd represents the generate command
var generateCmd = &cobra.Command{
	Use:   "generate",
	Short: "Geneates which uses hooks and configuration file to generate wasm binary out of it.",
	Long:  ``,
	Run: func(cmd *cobra.Command, args []string) {
		GenerateWasm(providerPath, config)
	},
}

func init() {
	rootCmd.AddCommand(generateCmd)

	generateCmd.Flags().StringVarP(&providerPath, "provider", "p", "", "Hook path which is required to parse Config file")
	generateCmd.Flags().StringVarP(&config, "config", "c", "", "Config file to generate wasm binary")

	generateCmd.MarkFlagRequired("provider")
	generateCmd.MarkFlagRequired("config")

}

func GenerateWasm(providerPath, configPath string) error {
	dir, _ := os.Getwd()
	i, err := ioutil.TempDir(dir, "tmp")
	fmt.Println(i, err)

	err = Dir(providerPath, i)
	fmt.Println(err)
	r := fmt.Sprintf("%s/config.yaml", i)
	err = File(configPath, r)

	fmt.Println(err)
	err = os.Chdir(i)
	fmt.Println(err)
	cmd := exec.Command("tackle", "config.yaml")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	err = cmd.Run()

	fmt.Println(err)

	os.RemoveAll(i)

	return nil
}

func Dir(src string, dst string) error {
	var err error
	var fds []os.FileInfo
	var srcinfo os.FileInfo

	if srcinfo, err = os.Stat(src); err != nil {
		return err
	}

	if err = os.MkdirAll(dst, srcinfo.Mode()); err != nil {
		return err
	}

	if fds, err = ioutil.ReadDir(src); err != nil {
		return err
	}
	for _, fd := range fds {
		srcfp := path.Join(src, fd.Name())
		dstfp := path.Join(dst, fd.Name())

		if fd.IsDir() {
			if err = Dir(srcfp, dstfp); err != nil {
				fmt.Println(err)
			}
		} else {
			if err = File(srcfp, dstfp); err != nil {
				fmt.Println(err)
			}
		}
	}
	return nil
}

func File(src, dst string) error {
	var err error
	var srcfd *os.File
	var dstfd *os.File
	var srcinfo os.FileInfo

	if srcfd, err = os.Open(src); err != nil {
		return err
	}
	defer srcfd.Close()

	if dstfd, err = os.Create(dst); err != nil {
		return err
	}
	defer dstfd.Close()

	if _, err = io.Copy(dstfd, srcfd); err != nil {
		return err
	}
	if srcinfo, err = os.Stat(src); err != nil {
		return err
	}
	return os.Chmod(dst, srcinfo.Mode())
}
