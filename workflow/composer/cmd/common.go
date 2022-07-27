package cmd

import (
	"fmt"
	"io"
	"io/ioutil"
	"os"
	"path"
)

func Dir(src string, dst string) error {
	var err error
	var fds []os.FileInfo
	var srcinfo os.FileInfo

	if srcinfo, err = os.Stat(src); err != nil {
		return fmt.Errorf("error while getting stats of file :%w", err)
	}

	if err = os.MkdirAll(dst, srcinfo.Mode()); err != nil {
		return fmt.Errorf("error while creating directory :%w", err)
	}

	if fds, err = ioutil.ReadDir(src); err != nil {
		return fmt.Errorf("error while reading directory :%w", err)
	}
	for _, fd := range fds {
		srcfp := path.Join(src, fd.Name())
		dstfp := path.Join(dst, fd.Name())

		if fd.IsDir() {
			if err = Dir(srcfp, dstfp); err != nil {
				return fmt.Errorf("error while creating directory :%w", err)
			}
		} else {
			if err = File(srcfp, dstfp); err != nil {
				return fmt.Errorf("error while creating file :%w", err)
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
		return fmt.Errorf("error in opening file :%w", err)
	}
	defer srcfd.Close()

	if dstfd, err = os.Create(dst); err != nil {
		return fmt.Errorf("error in creating file :%w", err)
	}
	defer dstfd.Close()

	if _, err = io.Copy(dstfd, srcfd); err != nil {
		return fmt.Errorf("error in copying file :%w", err)
	}
	if srcinfo, err = os.Stat(src); err != nil {
		return fmt.Errorf("error while getting stats of file :%w", err)
	}
	return os.Chmod(dst, srcinfo.Mode())
}
func removeFile(path string) error {
	if err := os.RemoveAll(path); err != nil {
		return err
	}
	return nil
}
