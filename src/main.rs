/*  *--<Preface>--*  //

 -=-  Author Details  -=-
 Blair Edwards
 They wouldn't have paid me enough anyway.

 -=-  Dates  -=-
 Started 2023-08-29

 -=-  Description  -=-
 This programme attempts to ...
 IO is primarily through GTK GUI Widgets.
 Extra debug is provided via STDOUT.
 Config is through compile-time 'const's.

 -=-  Task  -=-
 -=>  Set up a Cairo canvas.
 -=>  Render it on a timeout (IE animated).
 -=>  Figure out how to draw something cool.
 -=>  Le Many Step #3.

 -=-  Notes  -=-
 -=>  I've developed my own commenting notation for things that "aren't done" one way or another.  Such as:
	 -  //#  TODO
	 -  //?  Not sure / query
	 -  //!  Important note / relevant as technology advances
 -=>  Logging with `_LOG ()` takes a 'logLevel' argument, which roughly indicates:
	 -  0:  Critical, major errors, should Always be printed.
	 -  1:	Important info particularly critical functions, minor / user errors.
	 -  2:	Useful info / general programme flow.
	 -  3:	Debug info, steps throughout a function.
	 -  4:  Useful spam - printed often such as in a loop.
	 -  5:  Debug spam - printed often such as in a loop.
 -=>

//  *--</Preface>--*  */



//  *--<Preparations>--*  //

//  Local Compiler Pragmas
#! [allow (unused_variables)]
#! [allow (non_snake_case)]
#! [allow (unused_parens)]
#! [allow (dead_code)]
#! [allow (unused_imports)]

//  Imports
//use std ::io ::{stdin, stdout};
//use std ::time;
//use std ::fs ::File;
//use std ::path ::Path;
//use std ::io ::{Read, BufReader, BufRead};
//use std ::io ::{Write, BufWriter};
use std ::sync ::atomic ::{AtomicUsize, Ordering};
//use std ::rc ::Rc;
//use std ::cell ::RefCell;
//use std ::f64 ::consts ::PI;
//  of Which are GTK4
use gtk4 as gtk;
use gtk ::prelude ::*;
use gtk ::glib;
use gtk ::glib ::clone;
//  of Which are Local
mod loading_logos;
use loading_logos ::LoadingLogo;


//  Enumerations


//  Global Constants
const APP_ID: &str = "uk.second-pair.testing.gtk.loading-logos";
const APP_TITLE: &str = "Gtk Template";
const TIME_ANIM: u16 = 50;
const TIME_ANIM_SLOW: u16 = 1000;
/*  Sizing:
0:  Do Not Expand
1:  Default Size (User-Expandable)
2:  Fixed Size
3:  Fullscreen
*/
const APP_SIZING: u8 = 1;
const APP_W: i32 = 600;
const APP_H: i32 = 600;

const DRAW_TARGET_LEN: f64 = 1000.0;
const DRAW_LINE_WIDTH_BASE: f64 = 10.0;

const ANIM_TYPE: LoadingLogo = LoadingLogo ::OrbitNBalls;
//const ANIM_TYPE: loading_logos ::LoadingLogo = loading_logos ::LoadingLogo ::OrbitNBalls;
//#  It should be possible to parameterise all of the animation types.

//  Global Variables

//  Local Constants

//  Local Variables

//  Structures

//  *--</Preparations>--*  //



//  *--<Macros>--*  //

//  TODO:  implement macros.rs.
/*macro_rules! macroName
{
	($a: expr, $b: expr) =>
	{
		$a + $b
	}
	($a: expr) =>
	{
		$a
	}
}*/

//  *--</Macros>--*  //



//  *--<Traits & Implementations>--*  //

//  *--</Traits & Implementations>--*  //



//  *--<Main Code>--*  //

fn main () -> glib ::ExitCode
{
	let app = gtk ::Application ::builder ()
		.application_id (APP_ID)
		.build ();

	app .connect_activate (|app| {guiMain_create (app)});
	return app .run ();
}

fn guiWindow_create (app: &gtk ::Application) -> gtk ::ApplicationWindow
{
	//  Create the window.
	let theWindow = gtk ::ApplicationWindow ::builder ()
		.application (app)
		.title (APP_TITLE)
		.build ();
	//  Set the sizing based off our #defines.
	match (APP_SIZING)
	{
		0 => (),
		1 =>
		{
			theWindow .set_default_width (APP_W);
			theWindow .set_default_height (APP_H);
			theWindow .set_resizable (true);
		},
		2 =>
		{
			theWindow .set_default_width (APP_W);
			theWindow .set_default_height (APP_H);
			theWindow .set_resizable (false);
		},
		3 => theWindow .set_fullscreened (true),
		_ => panic! ("'APP_SIZING' out-of-range!"),
	};
	//  Apply theming.
	//#  CSS Implement
	theWindow .settings () .set_gtk_application_prefer_dark_theme (true);

	return theWindow
}

fn guiMain_create (app: &gtk ::Application)
{
	//  Create the main window.
	let window_main = guiWindow_create (app);

	//  Create the top-level layout widget.
	let box_main = gtk ::Box ::builder ()
		.orientation (gtk ::Orientation ::Vertical)
		.build ();
	window_main .set_child (Some (&box_main));

	//  Attach the Cairo canvas.
	let cairo_loading = gtk ::DrawingArea ::builder ()
		.vexpand (true)
		.hexpand (true)
		.build ();
	cairo_loading .set_draw_func (cairo_loading_render);
	//  Local, so we don't mess with GTK's main-thread requirements.
	gtk ::glib ::timeout_add_local
	(
		core ::time ::Duration ::from_millis (TIME_ANIM as u64),
		clone! (@strong cairo_loading => move ||
		{
			cairo_loading .queue_draw ();
			return Continue (true);
		}
	));
	box_main .append (&cairo_loading);

	//  Show the window and get outta here.
	window_main .present ();
}

//  *--</Main Code>--*  //



//  *--<Callbacks>--*  //

fn cairo_loading_render (area: &gtk ::DrawingArea, cairo: &gtk ::cairo ::Context, width: i32, height: i32)
{
	//  'static' iteration counter.
	static ITER: AtomicUsize = AtomicUsize ::new (0);
	let iter = ITER .fetch_add (1, Ordering ::Relaxed) as f64;

	//  Scale factor - calculated from 'width' and 'height'.
	let areaScale = core ::cmp ::min (width, height) as f64 / DRAW_TARGET_LEN;

	//  Move the origin to the middle and flip the Y-axis.
	let matrix = gtk ::cairo ::Matrix ::new (1.0, 0.0, 0.0, -1.0, width as f64 / 2.0, height as f64 / 2.0);
	cairo .transform (matrix);

	//  Perform the draw.
	ANIM_TYPE .draw (cairo, iter, areaScale);

	//  Render that line.
	cairo .set_line_width (DRAW_LINE_WIDTH_BASE * areaScale);
	cairo .set_line_cap (gtk ::cairo ::LineCap ::Round);
	cairo .set_line_join (gtk ::cairo ::LineJoin ::Round);
	cairo .set_source_rgba (1.0, 1.0, 1.0, 1.0);
	cairo .stroke () .unwrap ();
}

//  *--</Callbacks>--*  //
