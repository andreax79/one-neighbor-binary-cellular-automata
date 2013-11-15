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

import java.util.EnumSet;

public enum UpdatePattern {

	synchronous, // all cells are updated in parallel at each time step
	rasRandomIndependent, // at each time step, a cell to update is chosen at random
	rasRandomIndependent10, // at each time step, 10 cells to update are chosen at random
	rasRandomIndependent60, // at each time step, 100 cells to update are chosen at random
	rasRandomIndependent100, // at each time step, 100 cells to update are chosen at random
	rasRandomIndependent500, // at each time step, 500 cells to update are chosen at random
	rasRandomOrder, // at each time step, all nodes are updated, but in random order
	oasCyclic, 	// at each time step a cell is chosen according to a Þxed update order,  which was decided at random during initialization phase
	oasEqClocked10,
	oasEqClocked60,
	oasEqClocked100,
	oasEqClocked500;
	
	public static UpdatePattern getUpdatePattern(String updatePatternAsString) throws IllegalArgumentException {
		for (UpdatePattern updatePattern : EnumSet.allOf(UpdatePattern.class)) {
			if (updatePattern.toString().equals(updatePatternAsString))
				return updatePattern;
		}
		throw new IllegalArgumentException("Invalid update pattern. Valid update patterns are: " + validValues());
	}
	
	public static final String validValues() {
		StringBuilder sb = new StringBuilder();
		boolean first = true;
		for (UpdatePattern updatePattern : EnumSet.allOf(UpdatePattern.class)) {
			if (!first)
				sb.append(", ");
			sb.append(updatePattern);
			first = false;
		}
		return sb.toString();
	}
	
}
