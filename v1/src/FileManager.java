import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.io.OutputStream;
import javax.microedition.io.Connector;
import javax.microedition.io.FileConnection;

public class FileManager {
    private String downloadDir;

    public FileManager() {
        if (isJ2ME()) {
            downloadDir = "file:///root1/";
        } else {
            downloadDir = System.getProperty("java.io.tmpdir");
            if (downloadDir == null) {
                downloadDir = System.getProperty("user.home");
            }
        }
    }

    public String getDownloadDir() {
        if (downloadDir == null) {
            throw new IllegalStateException("Download directory is not set");
        }
        return downloadDir;
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

    public String selectFileForUpload() {
        if (isJ2ME()) {
            return selectFileForUploadJ2ME();
        } else {
            return selectFileForUploadJVM();
        }
    }

    public void downloadFile(String fileName, String fileContent) {
        if (isJ2ME()) {
            downloadFileJ2ME(fileName, fileContent);
        } else {
            downloadFileJVM(fileName, fileContent);
        }
    }

    private boolean isJ2ME() {
        return System.getProperty("microedition.configuration") != null;
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
            // Handle exception
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
            // Handle exception
        }
    }
}
    private boolean isJ2ME() {
        return System.getProperty("microedition.configuration") != null;
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
            // Handle exception
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
            // Handle exception
        }
    }
}
