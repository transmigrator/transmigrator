import java.io.File;
import java.io.IOException;
import java.io.OutputStream;
import java.util.Enumeration;
import java.util.Vector;

import javax.microedition.io.Connector;
import javax.microedition.io.FileConnection;
import javax.swing.JFileChooser;
import javax.swing.filechooser.FileFilter;

public class FileManager {
    private String downloadDir;

    public FileManager(String downloadDir) {
        this.downloadDir = downloadDir;
    }

    public boolean isJ2ME() {
        try {
            Class.forName("javax.microedition.io.Connector");
            return true;
        } catch (ClassNotFoundException e) {
            // Log the error or handle it in some way
            System.out.println("Error loading javax.microedition.io.Connector: " + e.getMessage());
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
                // Create a file chooser to select a file
                FileChooser fileChooser = new FileChooser();
                fileChooser.setFiles(fileVector);
                int selectedIndex = fileChooser.showFileChooser();
                if (selectedIndex != -1) {
                    String selectedFile = (String) fileVector.elementAt(selectedIndex);
                    fc.close();
                    return selectedFile;
                } else {
                    fc.close();
                    return null;
                }
            } else {
                fc.close();
                return null;
            }
        } catch (Exception e) {
            // Log the error or handle it in some way
            System.out.println("Error selecting file for upload: " + e.getMessage());
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
        } catch (Exception e) {
            // Log the error or handle it in some way
            System.out.println("Error downloading file: " + e.getMessage());
        }
    }

    private void downloadFileJVM(String fileName, String fileContent) {
        try {
            File file = new File(downloadDir + fileName);
            if (!file.exists()) {
                file.createNewFile();
            }
            java.io.FileWriter fw = new java.io.FileWriter(file);
            fw.write(fileContent);
            fw.close();
        } catch (IOException e) {
            // Log the error or handle it in some way
            System.out.println("Error downloading file: " + e.getMessage());
        } catch (SecurityException e) {
            // Log the error or handle it in some way
            System.out.println("Security error downloading file: " + e.getMessage());
        }
    }

    public String getDownloadDir() {
        return downloadDir;
    }

    public void setDownloadDir(String downloadDir) {
        this.downloadDir = downloadDir;
    }
}
