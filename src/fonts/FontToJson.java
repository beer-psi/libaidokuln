import javax.imageio.ImageIO;
import java.awt.*;
import java.awt.font.*;
import java.awt.geom.*;
import java.awt.image.*;
import java.io.*;
import java.util.*;

public class FontToJson {

    public static void writeJson(int[][] data, int fontHeight, String filename) throws IOException {
        FileWriter fw = new FileWriter(filename);
        String constName = filename.split("\\.")[0].replace("/", "").toUpperCase();

        fw.write("pub const " + constName + ": Font = Font { height: " + fontHeight + ".0, font: [");
        for (int[] character : data) {
            fw.write("&");
            fw.write(Arrays.toString(character));
            fw.write(", ");
        }
        fw.write("] };\n");
        fw.close();
    }

    public static int[] letterData(String letter, Font font) {
        BufferedImage img = new BufferedImage(1, 1, BufferedImage.TYPE_4BYTE_ABGR);
        Graphics g = img.getGraphics();

        //Set the font to be used when drawing the string
        g.setFont(font);

        //Get the string visual bounds
        FontRenderContext frc = g.getFontMetrics().getFontRenderContext();
        Rectangle2D rect = font.getStringBounds(letter, frc);
        //Release resources
        g.dispose();

        //Then, we have to draw the string on the final image

        //Create a new image where to print the character
        img = new BufferedImage((int) Math.ceil(rect.getWidth()), (int) Math.ceil(rect.getHeight()), BufferedImage.TYPE_INT_ARGB);
        g = img.getGraphics();
        g.setColor(Color.black); //Otherwise the text would be white
        g.setFont(font);

        //Calculate x and y for that string
        FontMetrics fm = g.getFontMetrics();
        int x = 0;
        int y = fm.getAscent(); //getAscent() = baseline
        g.drawString(letter, x, y);

        //Release resources
        g.dispose();
        int[] data = new int[img.getHeight()*img.getWidth()];
        for(int i = 0; i < img.getHeight(); i++) {
            for(int j = 0; j < img.getWidth(); j++) {
                data[i*img.getWidth()+j] = ((img.getRGB(j, i)&0xff000000)>>24)&0xff;
            }
        }
        return data;
    }

    public static int fontHeight(Font font) {
        BufferedImage img = new BufferedImage(1, 1, BufferedImage.TYPE_INT_ARGB);
        Graphics g = img.getGraphics();

        //Set the font to be used when drawing the string
        g.setFont(font);

        //Get the string visual bounds
        FontRenderContext frc = g.getFontMetrics().getFontRenderContext();
        Rectangle2D rect = font.getStringBounds(" ", frc);
        //Release resources
        g.dispose();

        //Then, we have to draw the string on the final image

        //Create a new image where to print the character
        return (int) Math.ceil(rect.getHeight());
    }

    public static void main(String[] args) throws IOException {
        
        int[] fontSizes = { 18, 24, 30, 36 };
        for (int size : fontSizes) {
            int[][] font = new int[127-32][];
            final Font fontf = new Font("Times New Roman", Font.PLAIN, size);
            for (char c = 32; c < 127; c++) {
                font[c-32] = letterData(Character.toString(c), fontf);
            }
            writeJson(font, fontHeight(fontf), "times/" + size + ".rs");
        }
//        BufferedImage test = new BufferedImage(48, 83, BufferedImage.TYPE_4BYTE_ABGR);
//        int[] s = letterData(Character.toString('S'), new Font("Arial", Font.PLAIN, 72));
//        for(int i = 0; i < 83; i++) {
//            for(int j = 0; j < 48; j++) {
//                test.setRGB(j, i, s[i*48+j] == 0 ? 0 : 0xFFFFFFFF);
//            }
//        }
//        ImageIO.write(test, "png", new File("test.png"));
    }

    public static int[] letterDataRaw(String letter, Font font) {
        BufferedImage img = new BufferedImage(1, 1, BufferedImage.TYPE_INT_ARGB);
        Graphics g = img.getGraphics();

        //Set the font to be used when drawing the string
        g.setFont(font);

        //Get the string visual bounds
        FontRenderContext frc = g.getFontMetrics().getFontRenderContext();
        Rectangle2D rect = font.getStringBounds(letter, frc);
        //Release resources
        g.dispose();

        //Then, we have to draw the string on the final image

        //Create a new image where to print the character
        img = new BufferedImage((int) Math.ceil(rect.getWidth()), (int) Math.ceil(rect.getHeight()), BufferedImage.TYPE_INT_ARGB);
        g = img.getGraphics();
        g.setColor(Color.black); //Otherwise the text would be white
        g.setFont(font);

        //Calculate x and y for that string
        FontMetrics fm = g.getFontMetrics();
        int x = 0;
        int y = fm.getAscent(); //getAscent() = baseline
        g.drawString(letter, x, y);

        //Release resources
        g.dispose();
        int[] data = new int[img.getHeight()*img.getWidth()];
        for(int i = 0; i < img.getHeight(); i++) {
            for(int j = 0; j < img.getWidth(); j++) {
                data[i*img.getWidth()+j] = img.getRGB(j, i);
            }
        }
        System.out.println(letter + ": ("+img.getWidth()+", "+img.getHeight()+")");
        return data;
    }
}
