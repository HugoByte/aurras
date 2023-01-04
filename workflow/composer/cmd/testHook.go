package cmd

import (
	"fmt"
	"io/ioutil"
	"os"
	"os/exec"

	"github.com/spf13/cobra"
)

var testConfig string

// testHookCmd represents the testHook command
var testHookCmd = &cobra.Command{
	Use:   "test",
	Short: "To Test Workflow Hooks",
	Long:  ``,
	Run: func(cmd *cobra.Command, args []string) {
		TestHooks(ProviderPath, testConfig)
	},
}

func init() {
	rootCmd.AddCommand(testHookCmd)
	testHookCmd.Flags().StringVarP(&testConfig, "config", "c", "", "Config file to test workflow hooks")
	testHookCmd.MarkFlagRequired("testconfig")
}

func TestHooks(providerPath, configPath string) error {
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

	err = runTest()
	if err != nil {
		return err
	}

	err = removeFile(tempDir)
	if err != nil {
		return err
	}

	return nil
}

func runTest() error {

	fmt.Println("Running Test ...")
	cmd := exec.Command("python3", "-Wignore", "-m", "unittest")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	err := cmd.Run()
	if err != nil {
		return fmt.Errorf("failed to execute unittest :%w", err)
	}
	fmt.Println("Test Completed")
	return nil
}
