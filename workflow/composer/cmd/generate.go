package cmd

import (
	"fmt"
	"io/ioutil"
	"os"
	"os/exec"

	"github.com/spf13/cobra"
)

var config string
var wasmBinaryPath string

const ProviderPath = "../providers"

// generateCmd represents the generate command
var generateCmd = &cobra.Command{
	Use:   "generate",
	Short: "Generates which uses hooks and configuration file to generate wasm binary out of it.",
	Long:  ``,
	Run: func(cmd *cobra.Command, args []string) {

		GenerateWasm(ProviderPath, config)
	},
}

func init() {
	rootCmd.AddCommand(generateCmd)

	generateCmd.Flags().StringVarP(&config, "config", "c", "", "Config file to generate wasm binary")
	generateCmd.Flags().StringVarP(&wasmBinaryPath, "out", "o", "", "Output path ")
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

	err = copyTarget(cwd)
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
	outPath, err := os.UserHomeDir()

	if err != nil {
		return "", fmt.Errorf("error in getting home dir :%w", err)
	}

	err = os.Chdir(fmt.Sprintf("%s/output", outPath))
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

func copyTarget(cwd string) error {

	var err error
	err = os.Chdir("target/")
	if err != nil {
		fmt.Println(err)
		return fmt.Errorf("error in chaning dir :%w", err)
	}

	cmd := exec.Command("cp", "wasm32-wasi/release/workflow.wasm", cwd)
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
