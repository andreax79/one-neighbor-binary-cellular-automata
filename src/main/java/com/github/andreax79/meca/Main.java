/**
 * Copyright (C) 2009 Andrea Bonomi - <andrea.bonomi@gmail.com>
 *
 * https://github.com/andreax79/one-neighbor-binary-cellular-automata
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the
 * Free Software Foundation; either version 2 of the License, or (at your
 * option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
 * or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License
 * for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.)
 *
 */

package com.github.andreax79.meca;

import java.awt.Color;
import java.awt.Graphics2D;
import java.awt.image.BufferedImage;
import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.util.LinkedList;
import java.util.List;

import org.apache.commons.cli.CommandLine;
import org.apache.commons.cli.CommandLineParser;
import org.apache.commons.cli.HelpFormatter;
import org.apache.commons.cli.Option;
import org.apache.commons.cli.OptionBuilder;
import org.apache.commons.cli.OptionGroup;
import org.apache.commons.cli.Options;
import org.apache.commons.cli.ParseException;
import org.apache.commons.cli.PosixParser;

import com.sun.image.codec.jpeg.JPEGCodec;
import com.sun.image.codec.jpeg.JPEGEncodeParam;
import com.sun.image.codec.jpeg.JPEGImageEncoder;

public class Main {

	private static int cellSize = 2;

	public static Stats drawRule(int ruleNumber, int size, Boundaries boundaries, UpdatePattern updatePattern, int steps, double alpha) throws IOException {
		return drawRule(ruleNumber, size, boundaries, updatePattern, steps, alpha, null, Output.all, ColorScheme.omegaColor);
	}

	public static Stats drawRule(int ruleNumber, int size, Boundaries boundaries, UpdatePattern updatePattern, int steps, double alpha, String pattern, Output output, ColorScheme colorScheme) throws IOException {
		Rule rule = new Rule(ruleNumber);
		Row row = new Row(size, rule, boundaries, updatePattern, pattern, alpha); // e.g. 00010011011111
		Stats stats = new Stats();

		FileOutputStream finalImage = null;
		Graphics2D g = null;
		BufferedImage img = null;
		
		if (output != Output.noOutput) {
			String fileName = "rule"+ruleNumber;
			// pattern
			if (pattern != null)
				fileName += pattern;
			// alpha
			if (alpha > 0)
				fileName += String.format("_a%02d", (int)(alpha*100));
			// updatePattern
			if (updatePattern != UpdatePattern.synchronous)
				fileName += "-" + updatePattern;
			fileName += ".jpeg";

			File file = new File(fileName);
			finalImage = new FileOutputStream(file);	

			int width = (int) (cellSize*(size+1) * (output == Output.all ? 1.25 : 1));
			int height = cellSize*(steps+1);
			img = new BufferedImage(width, height, BufferedImage.TYPE_INT_RGB);
			g = img.createGraphics();
			g.setBackground(Color.white);
			g.clearRect(0,0,width,height);
			g.setColor(Color.black);
		}
		
		int startMeansFromStep = 50;
		List<Double> densities = new LinkedList<Double>();
		double totalDensities = 0;
		
		double prevValue = 0;
		double prevDelta = 0;
		double prevOnes = 0;
		double prevOnesDelta = 0;
		
		for (int t=0; t<steps; t++) {
			if (t >= startMeansFromStep) {
				double density = row.getDensity();
				densities.add(density);
				totalDensities+=density;
			}
			// System.out.println(String.format("%4d", t) + " " + row.toString() + " ones=" + row.getOnes());
			if (output != Output.noOutput) {
				for (int j=0; j<row.getSize(); j++) {
					
					switch (colorScheme) {
					case noColor:						
						if (row.getCell(j).getState()) {
							g.setColor(Color.black);
							g.fillRect(j*cellSize,t*cellSize,cellSize,cellSize);
						}
						break;						
					case omegaColor:
						g.setColor(row.getCell(j).getOmegaColor());
						g.fillRect(j*cellSize,t*cellSize,cellSize,cellSize);
						break;
					case activationColor:
						if (row.getCell(j).getState()) {
							g.setColor(row.getCell(j).getColor());
							g.fillRect(j*cellSize,t*cellSize,cellSize,cellSize);
						}
						break;
					}
				}
				
				if (output == Output.all) {
					double value = row.getValue();
					double delta = Math.abs(value - prevValue);
					double ones = row.getOnes();
					double onesDelta = Math.abs(ones - prevOnes);
					if (t > 0) {
						g.setColor(Color.red);
						g.drawLine((int)(prevValue*cellSize/4.0)+cellSize*(size+1),(int)((t-1)*cellSize), (int)(value*cellSize/4.0)+cellSize*(size+1),(int)(t*cellSize));
						g.setColor(Color.blue);
						g.drawLine((int)(prevOnes*cellSize/4.0)+cellSize*(size+1),(int)((t-1)*cellSize), (int)(ones*cellSize/4.0)+cellSize*(size+1),(int)(t*cellSize));
						if (t >1) {
							g.setColor(Color.orange);
							g.drawLine((int)(prevDelta*cellSize/4.0)+cellSize*(size+1),(int)((t-1)*cellSize), (int)(delta*cellSize/4.0)+cellSize*(size+1),(int)(t*cellSize));
							g.setColor(Color.cyan);
							g.drawLine((int)(prevOnesDelta*cellSize/4.0)+cellSize*(size+1),(int)((t-1)*cellSize), (int)(onesDelta*cellSize/4.0)+cellSize*(size+1),(int)(t*cellSize));
						}
					}
					prevValue = value;
					prevDelta = delta;
					prevOnes = ones;
					prevOnesDelta = onesDelta;
				}
			}
				
			row = new Row(row);
		}
		
		double means = totalDensities / densities.size();
		double var = 0;
		for (double density : densities)
			var += Math.pow(density - means, 2);
		var = var / densities.size();
		System.out.println("Rule: " + ruleNumber + " Boundaties: " + boundaries + " UpdatePattern: " + updatePattern + " Alpha: " + String.format("%.3f",alpha) + " Means: " + String.format("%.6f",means) + " Variance: " + String.format("%.6f",var));
		stats.setMeans(means);
		stats.setVariance(var);
		
		if (output != Output.noOutput) {
			JPEGImageEncoder encoder = JPEGCodec.createJPEGEncoder(finalImage);
			JPEGEncodeParam param = encoder.getDefaultJPEGEncodeParam(img);
			param.setQuality(1.0f,true);
			encoder.encode(img, param);
			finalImage.flush();
			finalImage.close();
		}
				
		return stats;
	}
	
	@SuppressWarnings("static-access")
	public static void main(String[] args) {
		// create the command line parser
		CommandLineParser parser = new PosixParser();

		// create the Options
		Options options = new Options();
		options.addOption("X", "suppress-output", false, "don't create the output file" );

		OptionGroup boundariesOptions = new OptionGroup();
		boundariesOptions.addOption(new Option("P", "periodic", false, "periodic boundaries (default)" ));
		boundariesOptions.addOption(new Option("F", "fixed", false, "fixed-value boundaries" ));
		boundariesOptions.addOption(new Option("A", "adiabatic", false, "adiabatic boundaries" ));
		boundariesOptions.addOption(new Option("R", "reflective", false, "reflective boundaries" ));
		options.addOptionGroup(boundariesOptions);

		OptionGroup colorOptions = new OptionGroup();
		colorOptions.addOption(new Option("bn", "black-white", false, "black and white color scheme (default)"));
		colorOptions.addOption(new Option("ca", "activation-color", false, "activation color scheme" ));
		colorOptions.addOption(new Option("co", "omega-color", false, "omega color scheme" ));
		options.addOptionGroup(colorOptions);
		
		options.addOption(OptionBuilder.withLongOpt("rule")
                .withDescription("rule number (required)")
                .hasArg()
                .withArgName("rule")
                .create());
		
		options.addOption(OptionBuilder.withLongOpt("width")
                .withDescription("space width (required)")
                .hasArg()
                .withArgName("width")
                .create());

		options.addOption(OptionBuilder.withLongOpt("steps")
                .withDescription("number of steps (required)")
                .hasArg()
                .withArgName("steps")
                .create());
		
		options.addOption(OptionBuilder.withLongOpt("alpha")
                .withDescription("memory factor (default 0)")
                .hasArg()
                .withArgName("alpha")
                .create());
		
		options.addOption(OptionBuilder.withLongOpt("pattern")
                .withDescription("inititial pattern")
                .hasArg()
                .withArgName("pattern")
                .create());
		
		options.addOption("s", "single-seed", false, "single cell seed" );
		options.addOption("si", "single-seed-inverse", false, "all 1 except one cell" );

		options.addOption(OptionBuilder.withLongOpt("update-patter")
                .withDescription("update patter (valid values are " + UpdatePattern.validValues() + ")")
                .hasArg()
                .withArgName("updatepatter")
                .create());
		

		// test
		// args = new String[]{ "--rule=10", "--steps=500" , "--width=60", "-P" , "-s" };

		try {
			// parse the command line arguments
			CommandLine line = parser.parse( options, args );

			if (!line.hasOption("rule"))
				throw new ParseException("no rule number (use --rule=XX)");
			int rule;
			try {
				rule = Integer.parseInt(line.getOptionValue("rule"));
				if (rule < 0 || rule > 15)
					throw new ParseException("invalid rule number");
			} catch (NumberFormatException ex) {
				throw new ParseException("invalid rule number");
			}
			
			if (!line.hasOption("width"))
				throw new ParseException("no space width (use --width=XX)");
			int width;
			try {
				width = Integer.parseInt(line.getOptionValue("width"));
				if (width < 1)
					throw new ParseException("invalid width");
			} catch (NumberFormatException ex) {
				throw new ParseException("invalid width");
			}
			
			if (!line.hasOption("steps"))
				throw new ParseException("no number of steps (use --steps=XX)");
			int steps;
			try {
				steps = Integer.parseInt(line.getOptionValue("steps"));
				if (width < 1)
					throw new ParseException("invalid number of steps");
			} catch (NumberFormatException ex) {
				throw new ParseException("invalid number of steps");
			}
			
			double alpha = 0;
			if (line.hasOption("alpha")) {
				try {
					alpha = Double.parseDouble(line.getOptionValue("alpha"));
					if (alpha < 0 || alpha > 1)
						throw new ParseException("invalid alpha");
				} catch (NumberFormatException ex) {
					throw new ParseException("invalid alpha");
				}
			}

			String pattern = null;
			if (line.hasOption("pattern")) {
				pattern = line.getOptionValue("pattern");
				if (pattern != null)
					pattern = pattern.trim();
			}
			
			if (line.hasOption("single-seed"))
				pattern = "S";
			else if (line.hasOption("single-seed-inverse"))
				pattern = "SI";
			
			UpdatePattern updatePatter = UpdatePattern.synchronous;
			if (line.hasOption("update-patter")) {
				try {
					updatePatter = UpdatePattern.getUpdatePattern(line.getOptionValue("update-patter"));
				} catch (IllegalArgumentException ex) {
					throw new ParseException(ex.getMessage());
				}
			}
			
			Boundaries boundaries = Boundaries.periodic;
			if (line.hasOption("periodic"))
				boundaries = Boundaries.periodic;
			else if (line.hasOption("fixed"))
				boundaries = Boundaries.fixed;
			else if (line.hasOption("adiabatic"))
				boundaries = Boundaries.adiabatic;
			else if (line.hasOption("reflective"))
				boundaries = Boundaries.reflective;
			
			ColorScheme colorScheme = ColorScheme.noColor;
			if (line.hasOption("black-white"))
				colorScheme = ColorScheme.noColor;
			else if (line.hasOption("activation-color"))
				colorScheme = ColorScheme.activationColor;
			else if (line.hasOption("omega-color"))
				colorScheme = ColorScheme.omegaColor;
			
			Output output = Output.all;
			if (line.hasOption("suppress-output"))
				output = Output.noOutput;
			
			Main.drawRule(rule, width, boundaries, updatePatter, steps, alpha, pattern, output, colorScheme);

		} catch(ParseException ex) {
            System.err.println("Copyright (C) 2009 Andrea Bonomi - <andrea.bonomi@gmail.com>");
            System.err.println();
            System.err.println("https://github.com/andreax79/one-neighbor-binary-cellular-automata");
            System.err.println();
            System.err.println("This program is free software; you can redistribute it and/or modify it");
            System.err.println("under the terms of the GNU General Public License as published by the");
            System.err.println("Free Software Foundation; either version 2 of the License, or (at your");
            System.err.println("option) any later version.");
            System.err.println();
            System.err.println("This program is distributed in the hope that it will be useful, but");
            System.err.println("WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY");
            System.err.println("or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License");
            System.err.println("for more details.");
            System.err.println();
            System.err.println("You should have received a copy of the GNU General Public License along");
            System.err.println("with this program; if not, write to the Free Software Foundation, Inc.,");
            System.err.println("59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.)");
            System.err.println();
			System.err.println(ex.getMessage());
			HelpFormatter formatter = new HelpFormatter();
			formatter.printHelp("main", options);
			
		} catch (IOException ex) {
			System.err.println("IO exception:" + ex.getMessage());			
		}
		
	}

}
