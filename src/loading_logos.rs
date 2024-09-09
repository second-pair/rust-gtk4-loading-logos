/*  *--<Preface>--*  //

 -=-  Author Details  -=-
 Blair Edwards
 They wouldn't have paid me enough anyway.

 -=-  Part Of  -=-
 rust-gtk4-loading-logos.git

//  *--</Preface>--*  */



//  *--<Doc>--*  */

//!  Documentation!

//  *--</Doc>--*  */



//  *--<Preparations>--*  //

//  Local Compiler Pragmas
#! [allow (unused_variables)]
#! [allow (non_snake_case)]
#! [allow (unused_parens)]
#! [allow (dead_code)]
#! [allow (unused_imports)]

//  Imports
//use std ::io ::{stdin, stdout};
//use std ::io ::Result;
//use std ::time;
//use std ::fs ::File;
//use std ::path ::Path;
//use std ::io ::{Read, BufReader, BufRead};
//use std ::io ::{Write, BufWriter};
use std ::f64 ::consts ::PI;
//  of Which are GTK4
use gtk4 as gtk;
//use gtk ::prelude ::*;
//use gtk ::glib;
//use gtk ::glib ::clone;
//  of Which are Local

//  Global Enumerations

/*  Animation Types
1.  Pulsing radius-filling squircle.
2.  Circumference-filling circle CCW.
3.  Circumference-filling circle, CW.
4.  Orbiting N-Ary balls.
5.  Circumference-filling circle, with N-Ary orbiting balls.
6.  N-start circumference-following arcs.
7.  Concentric reverse-direction circumference-following circles.
8.  Concentric reverse-direction circumference-following circles V2.
9.  Concentric reverse-direction circumference-following circles, multi-speed.
10.  Orbiting N-Ary balls, with radius lines.
11.  Orbiting N-Ary balls, with radius-following pulsers.
*/
# [allow (non_camel_case_types)]
pub enum LoadingLogo
{
	PulseFillCircle,
	CircFillCircleCcw,
	CircFillCircleCw,
	OrbitNBalls,
	CircFillCircle_OrbitNBalls,
	NStartCircArcs,
	ConcentricCircArcsV1,
	ConcentricCircArcsV2,
	ConcentricCircArcsV3,
	OrbitNBalls_RadLines,
	OrbitNBalls_PulseRadLines,
}

//  Global Constants

//  Global Variables

//  Local Constants

//  Local Variables

//  *--</Preparations>--*  //



//  *--<Errors>--*  */

/*pub enum Error
{
	Nope,
}
impl std ::fmt ::Display for Error
{
	fn fmt (&self, fmtr: &mut std ::fmt ::Formatter) -> std ::fmt ::Result
	{
		let errMsg = match self
		{
			Error ::Nope => "Nope error reason.",
		};
		return write! (fmtr, "{}", errMsg);
	}
}
impl std ::fmt ::Debug for Error
{
	fn fmt (&self, fmtr: &mut std ::fmt ::Formatter) -> std ::fmt ::Result
	{
		let errMsg = match self
		{
			Error ::Nope => "Nope error debug reason.",
		};
		return write! (fmtr, "{}", errMsg);
	}
}*/

//  *--</Errors>--*  */



//  *--<Macros>--*  //

/*macro_rules! macroName
{
	($a :expr, $b :expr) =>
	{
		$a + $b
	}
	($a :expr) =>
	{
		$a
	}
}*/

//  *--</Macros>--*  //



//  *--<Traits & Implementations>--*  //

impl LoadingLogo
{
	//Work out how to break these down into smaller fucntions or something.  Does Rust have private impl functions?
	pub fn draw (self, cairo: &gtk ::cairo ::Context, iter: f64, areaScale: f64)
	{
		match (self)
		{
			//#  Parameterise more of these.
			LoadingLogo ::PulseFillCircle => self .draw_PulseFillCircle (cairo, iter, areaScale),
			LoadingLogo ::CircFillCircleCcw => self .draw_CircFillCircleCcw (cairo, iter, areaScale),
			LoadingLogo ::CircFillCircleCw => self .draw_CircFillCircleCw (cairo, iter, areaScale),
			LoadingLogo ::OrbitNBalls => self .draw_OrbitNBalls (cairo, iter, areaScale),
			LoadingLogo ::CircFillCircle_OrbitNBalls => self .draw_CircFillCircle_OrbitNBalls (cairo, iter, areaScale),
			LoadingLogo ::NStartCircArcs => self .draw_NStartCircArcs (cairo, iter, areaScale),
			LoadingLogo ::ConcentricCircArcsV1 => self .draw_ConcentricCircArcsV1 (cairo, iter, areaScale),
			LoadingLogo ::ConcentricCircArcsV2 => self .draw_ConcentricCircArcsV2 (cairo, iter, areaScale),
			LoadingLogo ::ConcentricCircArcsV3 => self .draw_ConcentricCircArcsV3 (cairo, iter, areaScale),
			LoadingLogo ::OrbitNBalls_RadLines => self .draw_OrbitNBalls_RadLines (cairo, iter, areaScale),
			LoadingLogo ::OrbitNBalls_PulseRadLines => self .draw_OrbitNBalls_PulseRadLines (cairo, iter, areaScale),
		}
	}

	fn draw_PulseFillCircle (self, cairo: &gtk ::cairo ::Context, iter: f64, areaScale: f64)
	{
		let iterScaled = iter * 3.0;
		let radMax = 100.0 * areaScale;
		if (iterScaled % (radMax * 2.0) <= radMax)
		{
			cairo .move_to (iterScaled % (radMax * 2.0), 0.0);
			cairo .arc (0.0, 0.0, iterScaled % (radMax * 2.0), 0.0, PI * 2.0);
		}
		else
		{
			cairo .move_to (radMax * 2.0 - iterScaled % (radMax * 2.0), 0.0);
			cairo .arc (0.0, 0.0, radMax * 2.0 - iterScaled % (radMax * 2.0), 0.0, PI * 2.0);
		}
	}
	fn draw_CircFillCircleCcw (self, cairo: &gtk ::cairo ::Context, iter: f64, areaScale: f64)
	{
		let iterScaled = iter * 0.26 % (PI * 4.0);
		let radMax = 100.0;
		cairo .move_to (radMax, 0.0);
		match (iterScaled <= PI * 2.0)
		{
			true => cairo .arc (0.0, 0.0, radMax, 0.0, iterScaled),
			false => cairo .arc_negative (0.0, 0.0, radMax, 0.0, iterScaled),
		};
	}
	fn draw_CircFillCircleCw (self, cairo: &gtk ::cairo ::Context, iter: f64, areaScale: f64)
	{
		let iterScaled = iter * 0.26 % (PI * 4.0);
		let iterRev = PI * 2.0 - iterScaled;
		let radMax = 100.0;
		cairo .move_to (radMax * iterRev .cos (), radMax * iterRev .sin ());
		match (iterScaled <= PI * 2.0)
		{
			true => cairo .arc (0.0, 0.0, radMax, iterRev, PI * 2.0),
			false => cairo .arc_negative (0.0, 0.0, radMax, iterRev, PI * 2.0),
		};
	}
	fn draw_OrbitNBalls (self, cairo: &gtk ::cairo ::Context, iter: f64, areaScale: f64)
	{
		let radCircle = 20.0 * areaScale;
		let radOrbit = 100.0 * areaScale;
		let countCircle = 3;
		let iterScaled = iter * 0.1;

		for circle in 0..countCircle
		{
			let iterStart = (iter * 0.1 + PI * 2.0 * circle as f64 / countCircle as f64) % (PI * 2.0);
			cairo .move_to (radCircle + radOrbit * iterStart .cos (), radOrbit * iterStart .sin ());
			cairo .arc (radOrbit * iterStart .cos (), radOrbit * iterStart .sin (), radCircle, 0.0, PI * 2.0);
		}
	}
	fn draw_CircFillCircle_OrbitNBalls (self, cairo: &gtk ::cairo ::Context, iter: f64, areaScale: f64)
	{
		let radOuter = 200.0;
		let radOrbit = 100.0;
		let radCircle = 20.0;
		let countCircle = 3;
		let iterCirc = (iter * 0.26) % (PI * 4.0);
		cairo .move_to (radOuter, 0.0);
		match (iterCirc <= PI * 2.0)
		{
			true => cairo .arc (0.0, 0.0, radOuter, 0.0, iterCirc),
			false => cairo .arc_negative (0.0, 0.0, radOuter, 0.0, iterCirc),
		};

		for circle in 0..countCircle
		{
			let iterStart = (iter * 0.1 + PI * 2.0 * circle as f64 / countCircle as f64) % (PI * 2.0);
			cairo .move_to (radCircle + radOrbit * iterStart .cos (), radOrbit * iterStart .sin ());
			cairo .arc (radOrbit * iterStart .cos (), radOrbit * iterStart .sin (), radCircle, 0.0, PI * 2.0);
		}
	}
	fn draw_NStartCircArcs (self, cairo: &gtk ::cairo ::Context, iter: f64, areaScale: f64)
	{
		let starts = 5;
		let iterCirc = (iter * 0.07) % (PI * 2.0);
		let radCircle = 120.0;
		let lengthArc = 0.4;
		for start in 0..starts
		{
			let iterStart = iterCirc + PI * 2.0 * start as f64 / starts as f64;
			cairo .move_to (radCircle * iterStart .cos (), radCircle * iterStart .sin ());
			cairo .arc (0.0, 0.0, radCircle, iterStart, iterStart + lengthArc);
		}
	}
	fn draw_ConcentricCircArcsV1 (self, cairo: &gtk ::cairo ::Context, iter: f64, areaScale: f64)
	{
		let iterScale = (iter * 0.26) % (PI * 4.0);
		let radStart = 80.0;
		let radSpace = 25.0;
		let countCircle = 3;
		for circle in 0..countCircle
		{
			let rad = radStart + (radSpace * circle as f64);
			if (circle % 2 == 0)
			{
				cairo .move_to (rad, 0.0);
				match (iterScale <= PI * 2.0)
				{
					true => cairo .arc (0.0, 0.0, rad, 0.0, iterScale),
					false => cairo .arc_negative (0.0, 0.0, rad, 0.0, iterScale),
				};
			}
			else
			{
				let iterRev = PI * 4.0 - iterScale;
				cairo .move_to (rad * iterRev .cos (), rad * iterRev .sin ());
				match (iterScale <= PI * 2.0)
				{
					true => cairo .arc (0.0, 0.0, rad, iterRev, PI * 2.0),
					false => cairo .arc_negative (0.0, 0.0, rad, iterRev, PI * 2.0),
				};
			}
		}
	}
	fn draw_ConcentricCircArcsV2 (self, cairo: &gtk ::cairo ::Context, iter: f64, areaScale: f64)
	{
		let iterScale = (iter * 0.26) % (PI * 4.0);
		let radStart = 80.0;
		let radSpace = 25.0;
		let countCircle = 3;
		for circle in 0..countCircle
		{
			let rad = radStart + (radSpace * circle as f64);
			if (circle % 2 == 0)
			{
				cairo .move_to (rad, 0.0);
				match (iterScale <= PI * 2.0)
				{
					true => cairo .arc (0.0, 0.0, rad, 0.0, iterScale),
					false => cairo .arc_negative (0.0, 0.0, rad, 0.0, iterScale),
				};
			}
			else
			{
				let iterRev = PI * 4.0 - iterScale;
				cairo .move_to (rad * (iterRev + PI) .cos (), rad * (iterRev + PI) .sin ());
				match (iterScale <= PI * 2.0)
				{
					true => cairo .arc (0.0, 0.0, rad, iterRev + PI, PI),
					false => cairo .arc_negative (0.0, 0.0, rad, iterRev + PI, PI),
				};
			}
		}
	}
	fn draw_ConcentricCircArcsV3 (self, cairo: &gtk ::cairo ::Context, iter: f64, areaScale: f64)
	{
		let iterScale = iter * 0.1;
		let radStart = 80.0;
		let radSpace = 25.0;
		let countCircle = 3;
		for circle in 0..countCircle
		{
			let rad = radStart + (radSpace * circle as f64);
			let iterScaleSpd = (iterScale + iterScale * 0.5 * circle as f64) % (PI * 4.0);
			if (circle % 2 == 0)
			{
				cairo .move_to (rad, 0.0);
				match (iterScaleSpd <= PI * 2.0)
				{
					true => cairo .arc (0.0, 0.0, rad, 0.0, iterScaleSpd),
					false => cairo .arc_negative (0.0, 0.0, rad, 0.0, iterScaleSpd),
				};
			}
			else
			{
				let iterScaleRev = PI * 4.0 - iterScaleSpd;
				cairo .move_to (rad * iterScaleRev .cos (), rad * iterScaleRev .sin ());
				match (iterScaleRev <= PI * 2.0)
				{
					true => cairo .arc_negative (0.0, 0.0, rad, iterScaleRev, PI * 2.0),
					false => cairo .arc (0.0, 0.0, rad, iterScaleRev, PI * 2.0),
				};
			}
		}
	}
	fn draw_OrbitNBalls_RadLines (self, cairo: &gtk ::cairo ::Context, iter: f64, areaScale: f64)
	{
		let radCircle = 20.0;
		let radOrbit = 200.0;
		let sparkStart = 30.0;
		let sparkGap = 40.0;
		let countCircle = 3;

		for circle in 0..countCircle
		{
			let iterStart = (iter * 0.1 + PI * 2.0 * circle as f64 / countCircle as f64) % (PI * 2.0);
			//  Spark Line
			cairo .move_to (sparkStart * iterStart .cos (), sparkStart * iterStart .sin ());
			cairo .line_to ((radOrbit - radCircle / 2.0 - sparkGap) * iterStart .cos (), (radOrbit - radCircle / 2.0 - sparkGap) * iterStart .sin ());
			//  Circle
			cairo .move_to (radCircle + radOrbit * iterStart .cos (), radOrbit * iterStart .sin ());
			cairo .arc (radOrbit * iterStart .cos (), radOrbit * iterStart .sin (), radCircle, 0.0, PI * 2.0);
		}
	}
	fn draw_OrbitNBalls_PulseRadLines (self, cairo: &gtk ::cairo ::Context, iter: f64, areaScale: f64)
	{
		let radCircle = 20.0;
		let radOrbit = 200.0;
		let sparkStart = 30.0;
		let sparkGap = 40.0;
		let sparkStop = (radOrbit - radCircle / 2.0 - sparkGap);
		let countCircle = 3;
		let iterSpark = (iter * 8.0) % ((sparkStop - sparkStart) * 2.0);

		for circle in 0..countCircle
		{
			let iterStart = (iter * 0.1 + PI * 2.0 * circle as f64 / countCircle as f64) % (PI * 2.0);
			//  Spark Line
			match (iterSpark <= sparkStop - sparkStart)
			{
				true =>
				{
					cairo .move_to (sparkStart * iterStart .cos (), sparkStart * iterStart .sin ());
					cairo .line_to ((sparkStart + iterSpark) * iterStart .cos (), (sparkStart + iterSpark) * iterStart .sin ());
				},
				false =>
				{
					let iterSpark = iterSpark % (sparkStop - sparkStart);
					cairo .move_to ((sparkStart + iterSpark) * iterStart .cos (), (sparkStart + iterSpark) * iterStart .sin ());
					cairo .line_to (sparkStop * iterStart .cos (), sparkStop * iterStart .sin ());
				},
			};
			//  Circle
			cairo .move_to (radCircle + radOrbit * iterStart .cos (), radOrbit * iterStart .sin ());
			cairo .arc (radOrbit * iterStart .cos (), radOrbit * iterStart .sin (), radCircle, 0.0, PI * 2.0);
		}
	}
}

//  *--</Traits & Implementations>--*  //



//  *--<Main Code>--*  //

/*fn subLocal () -> u8
{
	return 0;
}*/

//  *--</Main Code>--*  //



//  *--<Callbacks>--*  //

//  *--</Callbacks>--*  //
