// FileManager.java

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.net.MalformedURLException;
import java.net.URL;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.awt.FileDialog;
import java.awt.Frame;
import java.util.List;
import java.util.ArrayList;
import javafx.stage.Storage;

public class FileManager {
    private Storage storage;

    public FileManager() {
        storage = Storage.getStorage();
    }

    public void downloadFile(String url, String filename, FileProgressListener listener) {
        try {
            URL downloadUrl = new URL(url);
            storage.save(filename, Files.readAllBytes(Paths.get(downloadUrl.toURI())));
            listener.onComplete(new File(filename));
        } catch (MalformedURLException e) {
            System.err.println("Invalid URL: " + e.getMessage());
        } catch (IOException e) {
            System.err.println("Error downloading file: " + e.getMessage());
        }
    }

    public void uploadFile(String filename, String url) {
        // For simplicity, we'll use the built-in Java HTTP client
        try (java.net.HttpURLConnection connection = (java.net.HttpURLConnection) new URL(url).openConnection()) {
            connection.setRequestMethod("PUT");
            connection.setDoOutput(true);
            connection.getOutputStream().write(storage.load(filename));
            System.out.println("File uploaded successfully");
        } catch (MalformedURLException e) {
            System.err.println("Invalid URL: " + e.getMessage());
        } catch (IOException e) {
            System.err.println("Error uploading file: " + e.getMessage());
        }
    }

    public List<String> listFiles() {
        return storage.list();
    }

    public void deleteFile(String filename) {
        storage.delete(filename);
    }

    public void createDirectory(String directoryName) {
        storage.createDirectory(directoryName);
    }

    public String selectFile() {
        // Create a file dialog to select a file
        FileDialog fileDialog = new FileDialog(new Frame(), "Select File", FileDialog.LOAD);
        fileDialog.setVisible(true);
        return fileDialog.getFile();
    }

    public interface FileProgressListener {
        void onProgress(int bytesTransferred);
        void onComplete(File transferredFile);
    }

    public interface UploadProgressListener {
        void onProgress(int bytesUploaded);
        void onComplete(File uploadedFile);
    }
}