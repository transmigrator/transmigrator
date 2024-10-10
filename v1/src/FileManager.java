// FileManager.java

import javax.microedition.io.Connector;
import javax.microedition.io.FileConnection;
import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.io.OutputStream;
import java.util.Enumeration;
import java.util.Vector;
import javax.swing.JFileChooser;

public class FileManager {
    private String downloadDir;

    public FileManager() {
        this.downloadDir = null;
    }

    public boolean isJ2ME() {
        try {
            Class.forName("javax.microedition.io.Connector");
            return true;
        } catch (ClassNotFoundException e) {
            return false;
        }
    }

    public String selectFileForUpload() {
        if (isJ2ME()) {
            return selectFileForUploadJ2ME();
        } else {
            return selectFileForUploadJVM();
        }
    }

    private String selectFileForUploadJ2ME() {
        try {
            FileConnection fc = (FileConnection) Connector.open("file:///root1/");
            if (fc.isDirectory()) {
                Enumeration fileEnum = fc.list();
                Vector fileVector = new Vector();
                while (fileEnum.hasMoreElements()) {
                    String file = (String) fileEnum.nextElement();
                    fileVector.addElement(file);
                }
                // Allow the user to select a file from the list
                // For simplicity, return the first file for now
                String selectedFile = (String) fileVector.elementAt(0);
                fc.close();
                return selectedFile;
            } else {
                fc.close();
                return null;
            }
        } catch (Exception e) {
            return null;
        }
    }

    private String selectFileForUploadJVM() {
        JFileChooser fileChooser = new JFileChooser();
        int returnValue = fileChooser.showOpenDialog(null);
        if (returnValue == JFileChooser.APPROVE_OPTION) {
            File selectedFile = fileChooser.getSelectedFile();
            return selectedFile.getAbsolutePath();
        } else {
            return null;
        }
    }

    public void downloadFile(String fileName, String fileContent) {
        if (isJ2ME()) {
            downloadFileJ2ME(fileName, fileContent);
        } else {
            downloadFileJVM(fileName, fileContent);
        }
    }

    private void downloadFileJ2ME(String fileName, String fileContent) {
        try {
            FileConnection fc = (FileConnection) Connector.open(downloadDir + fileName);
            if (!fc.exists()) {
                fc.create();
            }
            OutputStream os = fc.openOutputStream();
            os.write(fileContent.getBytes());
            os.close();
            fc.close();
        } catch (IOException e) {
            // Handle exception, e.g., log the error or display an error message
            System.out.println("Error downloading file: " + e.getMessage());
        } catch (SecurityException e) {
            // Handle exception, e.g., log the error or display an error message
            System.out.println("Security error downloading file: " + e.getMessage());
        }
    }

    private void downloadFileJVM(String fileName, String fileContent) {
        try {
            File file = new File(downloadDir + fileName);
            if (!file.exists()) {
                file.createNewFile();
            }
            FileWriter fw = new FileWriter(file);
            fw.write(fileContent);
            fw.close();
        } catch (IOException e) {
            // Handle exception, e.g., log the error or display an error message
            System.out.println("Error downloading file: " + e.getMessage());
        } catch (SecurityException e) {
            // Handle exception, e.g., log the error or display an error message
            System.out.println("Security error downloading file: " + e.getMessage());
        }
    }

    public void setDownloadDir(String downloadDir) {
        this.downloadDir = downloadDir;
        if (!isJ2ME()) {
            File dir = new File(downloadDir);
            if (!dir.exists()) {
                dir.mkdirs();
            }
        }
    }

    public String getDownloadDir() {
        if (downloadDir == null) {
            // Return a default directory or handle this situation in a more user-friendly way
            return System.getProperty("java.io.tmpdir");
        }
        return downloadDir;
    }
}
