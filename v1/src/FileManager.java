// FileManager.java

import java.io.File;
import java.io.IOException;

public class FileManager {
    private FileUtils fileUtils;

    public FileManager() {
        fileUtils = new FileUtils();
    }

    public void downloadFile(String url, String filename, FileProgressListener listener) {
        fileUtils.downloadFile(url, filename, listener);
    }

    public void uploadFile(String filename, String url) {
        fileUtils.uploadFile(filename, url);
    }

    public List<String> listFiles() {
        return fileUtils.listFiles();
    }

    public void deleteFile(String filename) {
        fileUtils.deleteFile(filename);
    }

    public void createDirectory(String directoryName) {
        fileUtils.createDirectory(directoryName);
    }

    public String selectFile() {
        return fileUtils.selectFile();
    }
}
