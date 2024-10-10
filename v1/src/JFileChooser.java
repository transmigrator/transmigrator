import javax.swing.JFileChooser;
import javax.swing.filechooser.FileNameExtensionFilter;
import java.io.File;

public class JFileChooser {
    private JFileChooser fileChooser;

    public JFileChooser() {
        this.fileChooser = new JFileChooser();
    }

    public String selectFile() {
        int returnValue = fileChooser.showOpenDialog(null);
        if (returnValue == JFileChooser.APPROVE_OPTION) {
            File selectedFile = fileChooser.getSelectedFile();
            return selectedFile.getAbsolutePath();
        } else {
            return null;
        }
    }

    public void setFiles(Vector files) {
        // This method is not needed in JVM environment, as JFileChooser handles file selection
        // However, we can set a filter to show only specific file types
        FileNameExtensionFilter filter = new FileNameExtensionFilter("Text Files", "txt");
        fileChooser.setFileFilter(filter);
    }

    public int showFileChooser() {
        // This method is not needed in JVM environment, as JFileChooser handles file selection
        // However, we can show the file chooser dialog
        return fileChooser.showOpenDialog(null);
    }
}
