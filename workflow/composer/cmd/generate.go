/*
Copyright © 2022 NAME HERE <EMAIL ADDRESS>

*/
package cmd

import (
	"fmt"
	"io/ioutil"
	"os"
	"os/exec"

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
	var err error
	//get current working directory
	cwd, err := os.Getwd()
	if err != nil {
		return fmt.Errorf("error unable to get current working dir :%w", err)
	}
	tempDir, err := ioutil.TempDir(cwd, "tmp")
	if err != nil {
		return fmt.Errorf("error unable to create temp dir :%w", err)
	}

	err = Dir(providerPath, tempDir)
	if err != nil {
		return err
	}

	dstConfig := fmt.Sprintf("%s/config.yaml", tempDir)
	err = File(configPath, dstConfig)
	if err != nil {
		return err
	}

	// change current working directory
	err = os.Chdir(tempDir)
	if err != nil {
		return fmt.Errorf("error in chaning dir :%w", err)
	}

	err = runTackle()
	if err != nil {
		return err
	}

	outputDir, err := addWasmTarget(tempDir)
	if err != nil {
		return err
	}

	err = cargoBuild(outputDir, tempDir)
	if err != nil {
		return err
	}

	err = copyTarget()
	if err != nil {
		return err
	}

	err = clean(outputDir, tempDir)
	if err != nil {
		return err
	}

	return nil
}

func runTackle() error {

	cmd := exec.Command("tackle", "config.yaml")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	err := cmd.Run()
	if err != nil {
		return fmt.Errorf("failed to execute command tackle :%w", err)
	}
	return nil
}

func addWasmTarget(tempDir string) (string, error) {
	var err error
	err = os.Chdir("../../output")
	if err != nil {
		return "", fmt.Errorf("error in chaning dir :%w", err)
	}

	outputDir, err := os.Getwd()

	if err != nil {
		clean(outputDir, tempDir)
		return "", fmt.Errorf("error unable to get current working dir :%w", err)
	}

	cmd := exec.Command("rustup", "target", "add", "wasm32-wasi")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	err = cmd.Run()
	if err != nil {
		clean(outputDir, tempDir)
		return "", fmt.Errorf("failed to add target  :%w", err)
	}

	return outputDir, nil
}

func cargoBuild(outputDir, tempDir string) error {
	cmd := exec.Command("cargo", "build", "-q", "--release", "--target", "wasm32-wasi")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	err := cmd.Run()
	if err != nil {
		clean(outputDir, tempDir)
		return fmt.Errorf("failed to cargo build :%w", err)
	}

	return nil
}

func copyTarget() error {
	var err error
	err = os.Chdir("../../../aurras/target")
	if err != nil {
		return fmt.Errorf("error in chaning dir :%w", err)
	}

	cmd := exec.Command("cp", "wasm32-wasi/release/workflow.wasm", "../workflow/")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	err = cmd.Run()
	if err != nil {
		return fmt.Errorf("failed to copy :%w", err)
	}
	return nil

}

func clean(outputDir, tempDir string) error {
	cmd := exec.Command("cargo", "clean")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	err := cmd.Run()
	if err != nil {
		return fmt.Errorf("failed to clean :%w", err)
	}
	err = removeFile(outputDir)
	if err != nil {
		return fmt.Errorf("failed to remove :%w", err)
	}
	err = removeFile(tempDir)
	if err != nil {
		return fmt.Errorf("failed to remove :%w", err)
	}
	return nil
}
