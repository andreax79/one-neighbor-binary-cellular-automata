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

public class Cell {

	private int t;
	private boolean state;
	private Rule rule;
	private Color color;
	private double omega;
	private double bigOmega;
	private double alpha; // 0.5 0.55
	
	public Cell(Rule rule) {
		this(rule, 0);
	}
	
	public Cell(Rule rule, double alpha) {
		this.rule = rule;
		this.alpha = alpha;
		this.state = Math.random() < 0.5;
		this.color = state ? Color.black : Color.white;
		this.bigOmega = 1.0 / (1.0 - alpha);
		this.omega = this.state ? bigOmega : 0;
	}
	
	public Cell(Cell self, Cell neighbor) {
		this.alpha = self.alpha;
		this.rule = self.rule;
		this.t = self.t + 1;
		this.bigOmega = self.bigOmega;
		this.state = this.rule.compute(self, neighbor);
/*		if (Math.random() < 1d/1000d) {
			this.state = !state;
			color = Color.pink;
		}*/
		this.omega = (self.omega * this.alpha) + (this.state ? 1: 0);		
		if (this.omega != 0.5)
			this.state = (this.omega / this.bigOmega) > 0.5;

		if (state && color == null) {
			if (self.getState() && neighbor.getState()) color = Color.black;
			else if (!self.getState() && neighbor.getState()) color = Color.green;
			else if (self.getState() && !neighbor.getState()) color = Color.red;
			else color = Color.blue;
		} else {
			color = Color.white;
		}
	}
	
	public boolean getState() {
		return state;
	}

	public void setState(boolean state) {
		this.state = state;
		this.omega = this.state ? bigOmega : 0;
		this.color = state ? Color.black : Color.white;
	}
	
	public Color getColor() {
		return color;
	}

	public Color getOmegaColor() {
		float c = (float)(omega/bigOmega);
		if (c < 0) c = 0;
		if (c > 1) c = 1;
		return state ? new Color(1-c,1-c,1-c) : new Color(1,1f,1-c);
	}
	
	public double getBigOmega() {
		return bigOmega;
	}

	public double getOmega() {
		return omega;
	}

	public Rule getRule() {
		return rule;
	}

	public void setRule(Rule rule) {
		this.rule = rule;
	}
	
	@Override
	public int hashCode() {
		final int prime = 31;
		int result = 1;
		result = prime * result + (state ? 1231 : 1237);
		return result;
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj)
			return true;
		if (obj == null)
			return false;
		if (getClass() != obj.getClass())
			return false;
		Cell other = (Cell) obj;
		if (state != other.state)
			return false;
		return true;
	}
	
	@Override
	public String toString() {
		return state ? "1" : "0";
	}
	
}
