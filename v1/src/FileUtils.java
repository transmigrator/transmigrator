import javax.microedition.io.Connector;
import javax.microedition.io.FileConnection;
import javax.microedition.lcdui.Choice;
import javax.microedition.lcdui.ChoiceGroup;
import javax.microedition.lcdui.Command;
import javax.microedition.lcdui.CommandListener;
import javax.microedition.lcdui.Display;
import javax.microedition.lcdui.Displayable;
import javax.microedition.lcdui.Form;
import javax.microedition.lcdui.List;
import java.io.InputStream;
import java.io.OutputStream;
import java.util.Enumeration;
import java.util.Vector;
import javax.swing.JFileChooser;

public class FileUtils {
    public static String selectFile() {
        String platform = System.getProperty("microedition.platform");
        if (platform != null) {
            // J2ME platform
            return selectFileJ2ME();
        } else {
            // JVM platform
            return selectFileJVM();
        }
    }

    private static String selectFileJ2ME() {
        // J2ME implementation
        String selectedFile = null;
        try {
            // Create a file chooser UI component
            FileChooser fileChooser = new FileChooser();
            // Show the file chooser UI component
            fileChooser.show();
            // Get the selected file
            selectedFile = fileChooser.getSelectedFile();
        } catch (Exception e) {
            // Handle exception
        }
        return selectedFile;
    }

    private static String selectFileJVM() {
        // JVM implementation
        JFileChooser fileChooser = new JFileChooser();
        int returnValue = fileChooser.showOpenDialog(null);
        if (returnValue == JFileChooser.APPROVE_OPTION) {
            return fileChooser.getSelectedFile().getPath();
        } else {
            return null;
        }
    }

    public static class FileChooser extends Form implements CommandListener {
        private String selectedFile;
        private ChoiceGroup cg;
        private Command selectCommand;

        public FileChooser() {
            super("Select File");
            // Create a list of files
            String[] files = getFileList();
            cg = new ChoiceGroup("Files", Choice.POPUP);
            for (int i = 0; i < files.length; i++) {
                cg.append(files[i], null);
            }
            append(cg);
            // Create a command to select the file
            selectCommand = new Command("Select", Command.OK, 1);
            addCommand(selectCommand);
            setCommandListener(this);
        }

        public void show() {
            Display.getDisplay(null).setCurrent(this);
        }

        public String getSelectedFile() {
            return selectedFile;
        }

        public void commandAction(Command c, Displayable d) {
            if (c == selectCommand) {
                // Get the selected file
                selectedFile = cg.getString(cg.getSelectedIndex());
                // Go back to the previous screen
                Display.getDisplay(null).setCurrent(null);
            }
        }

        private String[] getFileList() {
            // Get a list of files from the device's file system
            String[] files = null;
            try {
                FileConnection fc = (FileConnection) Connector.open("file:///");
                if (fc.isDirectory()) {
                    Enumeration fileEnum = fc.list();
                    Vector fileVector = new Vector();
                    while (fileEnum.hasMoreElements()) {
                        String file = (String) fileEnum.nextElement();
                        if (file.endsWith(".txt")) {
                            fileVector.addElement(file);
                        }
                    }
                    files = new String[fileVector.size()];
                    fileVector.copyInto(files);
                }
                fc.close();
            } catch (Exception e) {
                // Handle exception
            }
            return files;
        }
    }
}
