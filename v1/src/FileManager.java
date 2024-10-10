import javax.microedition.io.Connector;
import javax.microedition.io.FileConnection;
import java.io.File;
import java.io.IOException;

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
        return downloadDir;
    }

    public void setDownloadDir(String downloadDir) {
        this.downloadDir = downloadDir;
    }

    public String selectFileForUpload() {
        return FileChooser.selectFile();
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
