// Copyright 2008 the V8 project authors. All rights reserved.
// Copyright 1996 John Maloney and Mario Wolczko.

// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA

// A class-for-class Dart translation of Octane's `deltablue.js`, the DeltaBlue
// incremental constraint solver ("The DeltaBlue Algorithm: An Incremental
// Constraint Hierarchy Solver", Freeman-Benson and Maloney, CACM January 1990).
// JS quirks are kept on purpose: `Strength.nextWeaker` maps REQUIRED to
// WEAKEST, values are doubles divided with `/`, and `destroyConstraint` only
// calls `removeFromGraph` for unsatisfied constraints.

library;

/* --- O b j e c t   M o d e l --- */

class OrderedCollection<T> {
  final List<T> elms = <T>[];

  void add(T elm) {
    elms.add(elm);
  }

  T at(int index) => elms[index];

  int size() => elms.length;

  T removeFirst() => elms.removeLast();

  void remove(T elm) {
    var index = 0, skipped = 0;
    for (var i = 0; i < elms.length; i++) {
      var value = elms[i];
      if (value != elm) {
        elms[index] = value;
        index++;
      } else {
        skipped++;
      }
    }
    for (var i = 0; i < skipped; i++) {
      elms.removeLast();
    }
  }
}

/* --- *
 * S t r e n g t h
 * --- */

/// Strengths are used to measure the relative importance of constraints. They
/// are canonical, so identity is value equality.
class Strength {
  final int strengthValue;
  final String name;

  const Strength._(this.strengthValue, this.name);

  static bool stronger(Strength s1, Strength s2) =>
      s1.strengthValue < s2.strengthValue;

  static bool weaker(Strength s1, Strength s2) =>
      s1.strengthValue > s2.strengthValue;

  static Strength weakestOf(Strength s1, Strength s2) =>
      weaker(s1, s2) ? s1 : s2;

  static Strength strongest(Strength s1, Strength s2) =>
      stronger(s1, s2) ? s1 : s2;

  Strength nextWeaker() {
    switch (strengthValue) {
      case 0:
        return WEAKEST;
      case 1:
        return WEAK_DEFAULT;
      case 2:
        return NORMAL;
      case 3:
        return STRONG_DEFAULT;
      case 4:
        return PREFERRED;
      case 5:
        return REQUIRED;
    }
    // JS returns `undefined` here; only WEAKEST reaches it and it is never
    // asked.
    throw StateError('nextWeaker');
  }

  static const REQUIRED = Strength._(0, "required");
  static const STONG_PREFERRED = Strength._(1, "strongPreferred");
  static const PREFERRED = Strength._(2, "preferred");
  static const STRONG_DEFAULT = Strength._(3, "strongDefault");
  static const NORMAL = Strength._(4, "normal");
  static const WEAK_DEFAULT = Strength._(5, "weakDefault");
  static const WEAKEST = Strength._(6, "weakest");
}

/* --- *
 * C o n s t r a i n t
 * --- */

/// A system-maintainable relationship between a set of variables.
abstract class Constraint {
  final Strength strength;

  Constraint(this.strength);

  void addToGraph();
  void removeFromGraph();
  void chooseMethod(int mark);
  bool isSatisfied();
  void markInputs(int mark);
  Variable output();
  void recalculate();
  void markUnsatisfied();
  bool inputsKnown(int mark);
  void execute();

  /// Activate this constraint and attempt to satisfy it.
  void addConstraint() {
    addToGraph();
    planner.incrementalAdd(this);
  }

  /// Attempt to find a way to enforce this constraint, returning the
  /// constraint it overrides, if any. Assume: not already satisfied.
  Constraint? satisfy(int mark) {
    chooseMethod(mark);
    if (!isSatisfied()) {
      if (strength == Strength.REQUIRED) {
        alert("Could not satisfy a required constraint!");
      }
      return null;
    }
    markInputs(mark);
    var out = output();
    var overridden = out.determinedBy;
    if (overridden != null) overridden.markUnsatisfied();
    out.determinedBy = this;
    if (!planner.addPropagate(this, mark)) alert("Cycle encountered");
    out.mark = mark;
    return overridden;
  }

  void destroyConstraint() {
    if (isSatisfied()) {
      planner.incrementalRemove(this);
    } else {
      removeFromGraph();
    }
  }

  /// An input constraint depends on external state, such as the mouse or
  /// imperative code.
  bool isInput() => false;
}

/* --- *
 * U n a r y   C o n s t r a i n t
 * --- */

/// Abstract superclass for constraints having a single possible output
/// variable.
abstract class UnaryConstraint extends Constraint {
  final Variable myOutput;
  bool satisfied = false;

  UnaryConstraint(this.myOutput, Strength strength) : super(strength) {
    addConstraint();
  }

  @override
  void addToGraph() {
    myOutput.addConstraint(this);
    satisfied = false;
  }

  @override
  void chooseMethod(int mark) {
    satisfied =
        (myOutput.mark != mark) &&
        Strength.stronger(strength, myOutput.walkStrength);
  }

  @override
  bool isSatisfied() => satisfied;

  @override
  void markInputs(int mark) {
    // has no inputs
  }

  @override
  Variable output() => myOutput;

  /// Calculate the walkabout strength, the stay flag, and, if it is 'stay',
  /// the value for the current output of this constraint. Assume this
  /// constraint is satisfied.
  @override
  void recalculate() {
    myOutput.walkStrength = strength;
    myOutput.stay = !isInput();
    if (myOutput.stay) execute(); // Stay optimization
  }

  @override
  void markUnsatisfied() {
    satisfied = false;
  }

  @override
  bool inputsKnown(int mark) => true;

  @override
  void removeFromGraph() {
    myOutput.removeConstraint(this);
    satisfied = false;
  }
}

/* --- *
 * S t a y   C o n s t r a i n t
 * --- */

/// Variables that should, with some level of preference, stay the same.
class StayConstraint extends UnaryConstraint {
  StayConstraint(Variable v, Strength str) : super(v, str);

  @override
  void execute() {
    // Stay constraints do nothing
  }
}

/* --- *
 * E d i t   C o n s t r a i n t
 * --- */

/// A unary input constraint used to mark a variable that the client wishes to
/// change.
class EditConstraint extends UnaryConstraint {
  EditConstraint(Variable v, Strength str) : super(v, str);

  @override
  bool isInput() => true;

  @override
  void execute() {
    // Edit constraints do nothing
  }
}

/* --- *
 * B i n a r y   C o n s t r a i n t
 * --- */

abstract final class Direction {
  static const NONE = 0;
  static const FORWARD = 1;
  static const BACKWARD = -1;
}

/// Abstract superclass for constraints having two possible output variables.
abstract class BinaryConstraint extends Constraint {
  Variable v1;
  Variable v2;
  int direction = Direction.NONE;

  BinaryConstraint(this.v1, this.v2, Strength strength) : super(strength) {
    addConstraint();
  }

  /// Decides if this constraint can be satisfied and which way it should flow
  /// based on the relative strength of the variables related, and record that
  /// decision.
  @override
  void chooseMethod(int mark) {
    if (v1.mark == mark) {
      direction =
          (v2.mark != mark && Strength.stronger(strength, v2.walkStrength))
          ? Direction.FORWARD
          : Direction.NONE;
    }
    if (v2.mark == mark) {
      direction =
          (v1.mark != mark && Strength.stronger(strength, v1.walkStrength))
          ? Direction.BACKWARD
          : Direction.NONE;
    }
    if (Strength.weaker(v1.walkStrength, v2.walkStrength)) {
      direction = Strength.stronger(strength, v1.walkStrength)
          ? Direction.BACKWARD
          : Direction.NONE;
    } else {
      direction = Strength.stronger(strength, v2.walkStrength)
          ? Direction.FORWARD
          : Direction.BACKWARD;
    }
  }

  @override
  void addToGraph() {
    v1.addConstraint(this);
    v2.addConstraint(this);
    direction = Direction.NONE;
  }

  @override
  bool isSatisfied() => direction != Direction.NONE;

  @override
  void markInputs(int mark) {
    input().mark = mark;
  }

  Variable input() => (direction == Direction.FORWARD) ? v1 : v2;

  @override
  Variable output() => (direction == Direction.FORWARD) ? v2 : v1;

  /// Calculate the walkabout strength, the stay flag, and, if it is 'stay',
  /// the value for the current output of this constraint. Assume this
  /// constraint is satisfied.
  @override
  void recalculate() {
    var ihn = input(), out = output();
    out.walkStrength = Strength.weakestOf(strength, ihn.walkStrength);
    out.stay = ihn.stay;
    if (out.stay) execute();
  }

  @override
  void markUnsatisfied() {
    direction = Direction.NONE;
  }

  @override
  bool inputsKnown(int mark) {
    var i = input();
    return i.mark == mark || i.stay || i.determinedBy == null;
  }

  @override
  void removeFromGraph() {
    v1.removeConstraint(this);
    v2.removeConstraint(this);
    direction = Direction.NONE;
  }
}

/* --- *
 * S c a l e   C o n s t r a i n t
 * --- */

/// Relates two variables by the linear scaling relationship
/// "v2 = (v1 * scale) + offset".
class ScaleConstraint extends BinaryConstraint {
  final Variable scale;
  final Variable offset;

  ScaleConstraint(
    Variable src,
    this.scale,
    this.offset,
    Variable dest,
    Strength strength,
  ) : super(src, dest, strength);

  @override
  void addToGraph() {
    super.addToGraph();
    scale.addConstraint(this);
    offset.addConstraint(this);
  }

  @override
  void removeFromGraph() {
    super.removeFromGraph();
    scale.removeConstraint(this);
    offset.removeConstraint(this);
  }

  @override
  void markInputs(int mark) {
    super.markInputs(mark);
    scale.mark = offset.mark = mark;
  }

  @override
  void execute() {
    if (direction == Direction.FORWARD) {
      v2.value = v1.value * scale.value + offset.value;
    } else {
      v1.value = (v2.value - offset.value) / scale.value;
    }
  }

  @override
  void recalculate() {
    var ihn = input(), out = output();
    out.walkStrength = Strength.weakestOf(strength, ihn.walkStrength);
    out.stay = ihn.stay && scale.stay && offset.stay;
    if (out.stay) execute();
  }
}

/* --- *
 * E q u a l i t  y   C o n s t r a i n t
 * --- */

/// Constrains two variables to have the same value.
class EqualityConstraint extends BinaryConstraint {
  EqualityConstraint(Variable var1, Variable var2, Strength strength)
    : super(var1, var2, strength);

  @override
  void execute() {
    output().value = input().value;
  }
}

/* --- *
 * V a r i a b l e
 * --- */

/// A constrained variable, which also maintains the structure of the
/// constraint graph and the current dataflow graph.
class Variable {
  double value;
  final OrderedCollection<Constraint> constraints =
      OrderedCollection<Constraint>();
  Constraint? determinedBy;
  int mark = 0;
  Strength walkStrength = Strength.WEAKEST;
  bool stay = true;
  final String name;

  Variable(this.name, [this.value = 0]);

  void addConstraint(Constraint c) {
    constraints.add(c);
  }

  void removeConstraint(Constraint c) {
    constraints.remove(c);
    if (determinedBy == c) determinedBy = null;
  }
}

/* --- *
 * P l a n n e r
 * --- */

/// The DeltaBlue planner.
class Planner {
  int currentMark = 0;

  /// Attempt to satisfy the given constraint and, if successful, incrementally
  /// update the dataflow graph, resatisfying any constraints it overrides.
  void incrementalAdd(Constraint c) {
    var mark = newMark();
    var overridden = c.satisfy(mark);
    while (overridden != null) {
      overridden = overridden.satisfy(mark);
    }
  }

  /// Remove the given constraint and try to satisfy, strongest first, the
  /// downstream constraints that this may free up. Assume: c is satisfied.
  void incrementalRemove(Constraint c) {
    var out = c.output();
    c.markUnsatisfied();
    c.removeFromGraph();
    var unsatisfied = removePropagateFrom(out);
    var strength = Strength.REQUIRED;
    do {
      for (var i = 0; i < unsatisfied.size(); i++) {
        var u = unsatisfied.at(i);
        if (u.strength == strength) incrementalAdd(u);
      }
      strength = strength.nextWeaker();
    } while (strength != Strength.WEAKEST);
  }

  int newMark() => ++currentMark;

  /// Extract a plan for resatisfaction starting from the given source
  /// constraints. The plan contains only constraints whose output variables
  /// are not stay. Assume: sources are all satisfied.
  Plan makePlan(OrderedCollection<Constraint> sources) {
    var mark = newMark();
    var plan = Plan();
    var todo = sources;
    while (todo.size() > 0) {
      var c = todo.removeFirst();
      if (c.output().mark != mark && c.inputsKnown(mark)) {
        plan.addConstraint(c);
        c.output().mark = mark;
        addConstraintsConsumingTo(c.output(), todo);
      }
    }
    return plan;
  }

  /// Extract a plan for resatisfying starting from the output of the given
  /// constraints, usually a set of input constraints.
  Plan extractPlanFromConstraints(OrderedCollection<Constraint> constraints) {
    var sources = OrderedCollection<Constraint>();
    for (var i = 0; i < constraints.size(); i++) {
      var c = constraints.at(i);
      if (c.isInput() && c.isSatisfied()) {
        // not in plan already and eligible for inclusion
        sources.add(c);
      }
    }
    return makePlan(sources);
  }

  /// Recompute the walkabout strengths and stay flags of all variables
  /// downstream of the given constraint. If a cycle is detected, remove the
  /// given constraint and answer false.
  bool addPropagate(Constraint c, int mark) {
    var todo = OrderedCollection<Constraint>();
    todo.add(c);
    while (todo.size() > 0) {
      var d = todo.removeFirst();
      if (d.output().mark == mark) {
        incrementalRemove(c);
        return false;
      }
      d.recalculate();
      addConstraintsConsumingTo(d.output(), todo);
    }
    return true;
  }

  /// Update the walkabout strengths and stay flags of all variables downstream
  /// of the given constraint, answering the unsatisfied constraints found.
  OrderedCollection<Constraint> removePropagateFrom(Variable out) {
    out.determinedBy = null;
    out.walkStrength = Strength.WEAKEST;
    out.stay = true;
    var unsatisfied = OrderedCollection<Constraint>();
    var todo = OrderedCollection<Variable>();
    todo.add(out);
    while (todo.size() > 0) {
      var v = todo.removeFirst();
      for (var i = 0; i < v.constraints.size(); i++) {
        var c = v.constraints.at(i);
        if (!c.isSatisfied()) unsatisfied.add(c);
      }
      var determining = v.determinedBy;
      for (var i = 0; i < v.constraints.size(); i++) {
        var next = v.constraints.at(i);
        if (next != determining && next.isSatisfied()) {
          next.recalculate();
          todo.add(next.output());
        }
      }
    }
    return unsatisfied;
  }

  void addConstraintsConsumingTo(
    Variable v,
    OrderedCollection<Constraint> coll,
  ) {
    var determining = v.determinedBy;
    var cc = v.constraints;
    for (var i = 0; i < cc.size(); i++) {
      var c = cc.at(i);
      if (c != determining && c.isSatisfied()) coll.add(c);
    }
  }
}

/* --- *
 * P l a n
 * --- */

/// An ordered list of constraints to be executed in sequence to resatisfy all
/// currently satisfiable constraints in the face of changing inputs.
class Plan {
  final OrderedCollection<Constraint> v = OrderedCollection<Constraint>();

  void addConstraint(Constraint c) {
    v.add(c);
  }

  int size() => v.size();

  Constraint constraintAt(int index) => v.at(index);

  void execute() {
    for (var i = 0; i < size(); i++) {
      var c = constraintAt(i);
      c.execute();
    }
  }
}

/* --- *
 * M a i n
 * --- */

/// Builds a chain of n equality constraints with a stay constraint on one end
/// and an edit constraint on the other, then propagates edits down the chain.
void chainTest(int n) {
  planner = Planner();
  Variable? prev, first, last;

  // Build chain of n equality constraints
  for (var i = 0; i <= n; i++) {
    var name = "v$i";
    var v = Variable(name);
    if (prev != null) EqualityConstraint(prev, v, Strength.REQUIRED);
    if (i == 0) first = v;
    if (i == n) last = v;
    prev = v;
  }

  StayConstraint(last!, Strength.STRONG_DEFAULT);
  var edit = EditConstraint(first!, Strength.PREFERRED);
  var edits = OrderedCollection<Constraint>();
  edits.add(edit);
  var plan = planner.extractPlanFromConstraints(edits);
  for (var i = 0; i < 100; i++) {
    first.value = i.toDouble();
    plan.execute();
    if (last.value != i) alert("Chain test failed.");
  }
}

/// Relates two sets of variables by a linear transformation, then changes a
/// variable on either side of the mapping and the scale and offset factors.
void projectionTest(int n) {
  planner = Planner();
  var scale = Variable("scale", 10);
  var offset = Variable("offset", 1000);
  Variable? src, dst;

  var dests = OrderedCollection<Variable>();
  for (var i = 0; i < n; i++) {
    src = Variable("src$i", i.toDouble());
    dst = Variable("dst$i", i.toDouble());
    dests.add(dst);
    StayConstraint(src, Strength.NORMAL);
    ScaleConstraint(src, scale, offset, dst, Strength.REQUIRED);
  }

  change(src!, 17);
  if (dst!.value != 1170) alert("Projection 1 failed");
  change(dst, 1050);
  if (src.value != 5) alert("Projection 2 failed");
  change(scale, 5);
  for (var i = 0; i < n - 1; i++) {
    if (dests.at(i).value != i * 5 + 1000) alert("Projection 3 failed");
  }
  change(offset, 2000);
  for (var i = 0; i < n - 1; i++) {
    if (dests.at(i).value != i * 5 + 2000) alert("Projection 4 failed");
  }
}

void change(Variable v, double newValue) {
  var edit = EditConstraint(v, Strength.PREFERRED);
  var edits = OrderedCollection<Constraint>();
  edits.add(edit);
  var plan = planner.extractPlanFromConstraints(edits);
  for (var i = 0; i < 10; i++) {
    v.value = newValue;
    plan.execute();
  }
  edit.destroyConstraint();
}

// Global variable holding the current planner.
Planner planner = Planner();

/// Number of `alert` calls; nonzero means the solver misbehaved.
int alerts = 0;

void alert(String message) {
  alerts++;
}

/// One run of the benchmark, returning the marks the two planners handed out
/// as a checksum of the work done.
int deltaBlue() {
  chainTest(100);
  var marks = planner.currentMark;
  projectionTest(100);
  return marks + planner.currentMark;
}
