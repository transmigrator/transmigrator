import javax.microedition.io.Connector;
import javax.microedition.io.FileConnection;
import javax.swing.JFileChooser;
import java.io.File;

public class FileChooser {
    public static String selectFile() {
        if (isJ2ME()) {
            return selectFileJ2ME();
        } else {
            return selectFileJVM();
        }
    }

    private static boolean isJ2ME() {
        return System.getProperty("microedition.configuration") != null;
    }

    private static String selectFileJ2ME() {
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

    private static String selectFileJVM() {
        JFileChooser fileChooser = new JFileChooser();
        int returnValue = fileChooser.showOpenDialog(null);
        if (returnValue == JFileChooser.APPROVE_OPTION) {
            File selectedFile = fileChooser.getSelectedFile();
            return selectedFile.getAbsolutePath();
        } else {
            return null;
        }
    }
}