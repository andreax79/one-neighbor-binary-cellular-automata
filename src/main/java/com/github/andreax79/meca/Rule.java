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

import java.io.Serializable;

public class Rule implements Serializable {

	private static final long serialVersionUID = 1L;
	
	private int n;
	
	public Rule(int n) {
		if (n < 0 || n > 15)
			throw new IllegalArgumentException();
		this.n = n;
	}
	
	public boolean compute(Cell selfCell, Cell neighborCell) {
		return compute(selfCell.getState(), neighborCell.getState());
	}
	
	public boolean compute(boolean self, boolean neighbor) {
		boolean result;
		switch(n) {
		case  0: result =  false; break;
		case  1: result =  ! ( self || neighbor); break;
		case  2: result =  ! self && neighbor; break;
		case  3: result =  ! self; break;
		case  4: result =  self && ! neighbor; break;
		case  5: result =  ! neighbor; break;
		case  6: result =  self ^ neighbor; break;
		case  7: result =  ! (self & neighbor); break;
		case  8: result =  self & neighbor; break;
		case  9: result =  !(self ^ neighbor); break;
		case 10: result =  neighbor; break;
		case 11: result =  !(self & !neighbor); break;
		case 12: result =  self; break;
		case 13: result =  !(!self & neighbor); break;
		case 14: result =  self || neighbor; break;
		default: result =  true; break;		
		}
		return result;
	}
	
	public double getSensitivity() { // Binder 1993, Binder 1994
		double result = 0;
		// 00
		result += compute(false, false) != compute(false, true) ? 1 : 0;
		result += compute(false, false) != compute(true, false) ? 1 : 0;
		// 01
		result += compute(false, true) != compute(false, false) ? 1 : 0;
		result += compute(false, true) != compute(true, true) ? 1 : 0;
		// 10
		result += compute(true, false) != compute(true, true) ? 1 : 0;
		result += compute(true, false) != compute(false, false) ? 1 : 0;
		// 11
		result += compute(true, true) != compute(true, false) ? 1 : 0;
		result += compute(true, true) != compute(false, true) ? 1 : 0;		
		result = result / 8;
		return result;
	}
	
	@Override
	public String toString() {
		StringBuilder sb = new StringBuilder();
		sb.append("Rule #").append(String.format("%-2d", n));
		return sb.toString();
	}
	
}
