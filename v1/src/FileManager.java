// FileManager.java

import java.io.File;
import java.io.IOException;

public class FileManager {
    private FileUtils fileUtils;

    public FileManager() {
        String environment = System.getProperty("java.vm.name");
        if (environment.contains("J2ME")) {
            fileUtils = new J2MEFileUtils();
        } else {
            fileUtils = new JVMFileUtils();
        }
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

// FileUtils.java

public interface FileUtils {
    void downloadFile(String url, String filename, FileProgressListener listener);
    void uploadFile(String filename, String url);
    List<String> listFiles();
    void deleteFile(String filename);
    void createDirectory(String directoryName);
    String selectFile();
}

// J2MEFileUtils.java

import javax.microedition.io.Connection;
import javax.microedition.io.Connector;
import javax.microedition.io.file.FileConnection;

public class J2MEFileUtils implements FileUtils {
    // Implementation of FileUtils for Java ME environment
}

// JVMFileUtils.java

import java.io.File;
import java.io.IOException;

public class JVMFileUtils implements FileUtils {
    // Implementation of FileUtils for JVM environment
}
