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

import java.util.Arrays;
import java.util.LinkedList;
import java.util.List;

public class Row {

	private Cell cells[];
	private int t;
	private Boundaries boundaries;
	private UpdatePattern updatePattern;
	private Cell fixed;
	private int oasCyclicUpdateOrder[];
	private int oasEqClockedUpdate[];
	private int eqClockedTimes;
	
	public Row(int size, Rule rule, Boundaries boundaries, UpdatePattern updatePattern, boolean pattern[], double alpha) {
		this(size, rule, boundaries, updatePattern, alpha);
		if (pattern == null)
			return;
		for (int i = 0; i < cells.length; i++)
			cells[i].setState(pattern[i % pattern.length]);
	}

	public Row(int size, Rule rule, Boundaries boundaries, UpdatePattern updatePattern, String patternString, double alpha) {
		this(size, rule, boundaries, updatePattern, alpha);
		if (patternString == null || patternString.length() == 0)
			return;
		if (patternString.equalsIgnoreCase("S")) { // single seed
			for (int i=0; i<cells.length; i++)
				cells[i].setState(false);
			cells[cells.length/2].setState(true);
			return;
		}
		if (patternString.equalsIgnoreCase("SI")) { // single seed inverse
			for (int i=0; i<cells.length; i++)
				cells[i].setState(true);
			cells[cells.length/2].setState(false);
			return;
		}
		boolean pattern[] = new boolean[patternString.length()];
		for (int i = 0; i < patternString.length(); i++)
			pattern[i] = patternString.charAt(i) == '1';
		for (int i = 0; i < cells.length; i++)
			cells[i].setState(pattern[i % pattern.length]);
	}
	
	public Row(int size, Rule rule, Boundaries boundaries, UpdatePattern updatePattern, double alpha) {
		this.boundaries = boundaries;
		this.updatePattern = updatePattern;
		if (boundaries == Boundaries.fixed) {
			fixed = new Cell(rule);
			fixed.setState(false);
		}
		t = 0;
		cells = new Cell[size];
		for (int i = 0; i < cells.length; i++)
			cells[i] = new Cell(rule, alpha);
		if (updatePattern == UpdatePattern.oasCyclic)
			oasCyclicUpdateOrder = generateRandomOrderIndexes();
		if (updatePattern == UpdatePattern.oasEqClocked10)
			eqClockedTimes = 10;
		else if (updatePattern == UpdatePattern.oasEqClocked60)
			eqClockedTimes = 60;
		else if (updatePattern == UpdatePattern.oasEqClocked100)
			eqClockedTimes = 100;
		else if (updatePattern == UpdatePattern.oasEqClocked500)
			eqClockedTimes = 500;
		if (eqClockedTimes != 0)
			oasEqClockedUpdate = generateEqClockedTimes();
	}
	
	public Row(Row row) {
		this.boundaries = row.boundaries;
		this.updatePattern = row.updatePattern;
		this.oasEqClockedUpdate = row.oasEqClockedUpdate;
		this.eqClockedTimes = row.eqClockedTimes;
		this.fixed = row.fixed;
		this.oasCyclicUpdateOrder = row.oasCyclicUpdateOrder;
		t = row.getT() + 1;
		cells = new Cell[row.cells.length];
		
		switch (updatePattern) {
			case synchronous:
				synchronousUpdate(row);
				break;
			case rasRandomIndependent:
				rasRandomIndependentUpdate(row, 1);
				break;
			case rasRandomIndependent10:
				rasRandomIndependentUpdate(row, 10);
				break;
			case rasRandomIndependent60:
				rasRandomIndependentUpdate(row, 60);
				break;
			case rasRandomIndependent100:
				rasRandomIndependentUpdate(row, 100);
				break;
			case rasRandomIndependent500:
				rasRandomIndependentUpdate(row, 500);
				break;
			case rasRandomOrder:
				rasRandomOrderUpdate(row);
				break;
			case oasCyclic:
				oasCyclicUpdate(row);
				break;
			case oasEqClocked10:
			case oasEqClocked60:
			case oasEqClocked100:
			case oasEqClocked500:
				oasEqClockedUpdate(row);
				break;
			default:
				throw new RuntimeException("Invalid update pattern");
		}

	}
	
	private void synchronousUpdate(Row row) {
		// all cells are updated in parallel at each time step
		for (int i = 0; i < cells.length; i++)
			cells[i] = new Cell(row.getCell(i),row.getCell((t % 2 == 0) ? (i+1) : (i-1)));
	}
	
	private void rasRandomIndependentUpdate(Row row, int n) {
		// at each time step, n cells to update are chosen at random
		for (int i = 0; i < cells.length; i++)
			cells[i] = row.cells[i];
		for (int j = 0; j < n; j++) {
			int i = (int)Math.floor(Math.random() * cells.length);
			cells[i] = new Cell(getCell(i),getCell((t % 2 == 0) ? (i+1) : (i-1)));
		}
	}
	
	private void rasRandomOrderUpdate(Row row) {
		// at each time step, all nodes are updated, but in random order
		int randomOrderIndexes[] = generateRandomOrderIndexes();
		for (int i = 0; i < cells.length; i++)
			cells[i] = row.cells[i];
		for (int j = 0; j < cells.length; j++) {
			int i = randomOrderIndexes[j];
			cells[i] = new Cell(getCell(i),getCell((t % 2 == 0) ? (i+1) : (i-1)));
		}
	}
	
	private void oasCyclicUpdate(Row row) {
		// at each time step a cell is chosen according to a Þxed update order, 
		// which was decided at random during initialization phase
		for (int i = 0; i < cells.length; i++)
			cells[i] = row.cells[i];
		for (int j = 0; j < cells.length; j++) {
			int i = oasCyclicUpdateOrder[j];
			cells[i] = new Cell(getCell(i),getCell((t % 2 == 0) ? (i+1) : (i-1)));
		}
	}
	
	private void oasEqClockedUpdate(Row row) {
		Cell newCells[] = new Cell [cells.length];
		System.arraycopy(row.cells, 0, cells, 0, cells.length);
		for (int j = 0; j < eqClockedTimes; j++) {
			for (int i = 0; i < cells.length; i++) {
				if (oasEqClockedUpdate[i] == j)
					newCells[i] = new Cell(getCell(i),getCell((t % 2 == 0) ? (i+1) : (i-1)));
				else
					newCells[i] = cells[i];
			}
			System.arraycopy(newCells, 0, cells, 0, cells.length);
		}
	}
	
	private int[] generateRandomOrderIndexes() {
		int result[] = new int [cells.length];
		List<Integer> indexesSet = new LinkedList<Integer>();
		for (int i=0; i<cells.length; i++)
			indexesSet.add(i);
		for (int i=0; i<cells.length; i++)
			result[i] = indexesSet.remove((int)Math.floor(Math.random()*indexesSet.size()));
		return result;
	}
	
	private int[] generateEqClockedTimes() {
		int result[] = new int [cells.length];
		for (int i=0; i<cells.length; i++)
			result[i] = (int)Math.floor(Math.random()*eqClockedTimes);
		return result;
	}
	
	public Cell getCell(int i) {
		switch (boundaries) {
		case periodic:
			i = (i + cells.length) % cells.length;			
			break;
		case adiabatic:
			if (i < 0) i = 0;
			if (i >= cells.length) i = cells.length - 1;
			break;
		case reflective:
			if (i < 0) i = -i;
			if (i >= cells.length) i = cells.length - i + (cells.length - 2);
			break;
		case fixed:
			if ((i < 0) || (i >= cells.length)) return fixed;
			break;
		default:
			throw new RuntimeException("Invalid boundaries");
		}
		return cells[i];
	}
	
	public int getT() {
		return t;
	}

	public int getSize() {
		return cells.length;
	}
	
	public double getDensity() {
		return getOnes() * 1d / getSize();
	}
	
	public double getValue() {
		double omega = 0;
		for (int i=0; i < cells.length; i++)
			omega += cells[i].getOmega() / cells[i].getBigOmega();
		return omega;
	}
	
	public int getOnes() {
		int c = 0;
		for (int i=0; i < cells.length; i++)
			if(cells[i].getState())
				c++;
		return c;
	}
	
	public void setValue(int i, boolean value) {
		 getCell(i).setState(value);
	}
	
	public boolean getValue(int i) {
		return getCell(i).getState();
	}
	
	@Override
	public int hashCode() {
		final int prime = 31;
		int result = 1;
		result = prime * result + Arrays.hashCode(cells);
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
		Row other = (Row) obj;
		if (!Arrays.equals(cells, other.cells))
			return false;
		return true;
	}

	@Override
	public String toString() {
		StringBuilder sb = new StringBuilder();
		for (int i=0; i < cells.length; i++)
			sb.append(cells[i].toString());
		return sb.toString();
	}
	
}
