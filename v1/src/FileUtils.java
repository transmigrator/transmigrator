import java.io.File;
import java.io.IOException;
import javax.microedition.io.Connection;
import javax.microedition.io.Connector;
import javax.microedition.io.file.FileConnection;

public class FileUtils {
    private boolean isJ2ME;

    public FileUtils() {
        isJ2ME = System.getProperty("microedition.platform") != null || System.getProperty("java.vm.name").contains("J2ME");
    }

    public void downloadFile(String url, String filename, FileProgressListener listener) {
        if (isJ2ME) {
            // Use JSR-75 FileConnection API to download file
            FileConnection fc = (FileConnection) Connector.open("file:///" + filename, Connector.READ_WRITE);
            // ...
        } else {
            // Use java.io package to download file
            File file = new File(filename);
            java.io.FileOutputStream fos = new java.io.FileOutputStream(file);
            java.io.InputStream is = new java.net.URL(url).openStream();
            byte[] buffer = new byte[1024];
            int bytesRead;
            while ((bytesRead = is.read(buffer)) != -1) {
                fos.write(buffer, 0, bytesRead);
            }
            fos.close();
            is.close();
        }
    }

    public void uploadFile(String filename, String url) {
        if (isJ2ME) {
            // Use JSR-75 FileConnection API to upload file
            FileConnection fc = (FileConnection) Connector.open("file:///" + filename, Connector.READ_WRITE);
            // ...
        } else {
            // Use java.io package to upload file
            File file = new File(filename);
            java.io.FileInputStream fis = new java.io.FileInputStream(file);
            java.io.OutputStream os = new java.net.URL(url).openConnection().getOutputStream();
            byte[] buffer = new byte[1024];
            int bytesRead;
            while ((bytesRead = fis.read(buffer)) != -1) {
                os.write(buffer, 0, bytesRead);
            }
            fis.close();
            os.close();
        }
    }

    public List<String> listFiles() {
        if (isJ2ME) {
            // Use JSR-75 FileConnection API to list files
            FileConnection fc = (FileConnection) Connector.open("file:///", Connector.READ_WRITE);
            // ...
        } else {
            // Use java.io package to list files
            File dir = new File(".");
            String[] files = dir.list();
            List<String> fileList = new ArrayList<String>();
            for (String file : files) {
                fileList.add(file);
            }
            return fileList;
        }
    }

    public void deleteFile(String filename) {
        if (isJ2ME) {
            // Use JSR-75 FileConnection API to delete file
            FileConnection fc = (FileConnection) Connector.open("file:///" + filename, Connector.WRITE);
            // ...
        } else {
            // Use java.io package to delete file
            File file = new File(filename);
            file.delete();
        }
    }

    public void createDirectory(String directoryName) {
        if (isJ2ME) {
            // Use JSR-75 FileConnection API to create directory
            FileConnection fc = (FileConnection) Connector.open("file:///" + directoryName, Connector.WRITE);
            // ...
        } else {
            // Use java.io package to create directory
            File dir = new File(directoryName);
            dir.mkdir();
        }
    }

    public String selectFile() {
        if (isJ2ME) {
            // Use JSR-75 FileConnection API to select file
            FileConnection fc = (FileConnection) Connector.open("file:///", Connector.READ_WRITE);
            // ...
        } else {
            // Use java.io package to select file
            // This is a bit tricky, as there's no direct equivalent to J2ME's FileConnection API
            // You might want to use a library like JFileChooser or implement your own file chooser
            // For now, let's just return a hardcoded file name
            return "selected_file.txt";
        }
    }
}