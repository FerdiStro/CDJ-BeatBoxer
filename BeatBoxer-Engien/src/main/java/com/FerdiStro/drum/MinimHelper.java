package com.FerdiStro.drum;

import java.io.File;
import java.io.FileInputStream;
import java.io.InputStream;

public class MinimHelper {

    public String sketchPath(String fileName) {

        if (new File(fileName).isAbsolute()) {
            return fileName;
        }

        return fileName;
    }

    public InputStream createInput(String fileName) {
        try {
            String fullPath = sketchPath(fileName);
            File file = new File(fullPath);

            if (!file.exists()) {
                System.err.println("CRITICAL: file not exist: " + fullPath);
                return null;
            }

            return new FileInputStream(file);
        } catch (Exception e) {
            System.err.println("Error loading file: " + fileName);
            e.printStackTrace();
            return null;
        }
    }
}