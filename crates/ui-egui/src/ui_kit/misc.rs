//! Entries outside the main tabs: the application menus (File, Edit, View, Insert, Format,
//! Arrange, Slide Show, Help) and the contextual tabs — Shape Format, Picture Format, Arrange,
//! Table Design and Layout, Chart Design, SmartArt Design and Playback. Slides are drawn as
//! little 16:9 frames; what the command changes is in the accent colour, and most drawings are
//! a pair: the slide as it is, an arrow, the slide after the command.

#![allow(unused_imports)]

use egui::{Align2, Color32, Pos2, Rect, Shape, Stroke, Vec2, pos2, vec2};

use super::Entry;
use super::draw::{Dir, Draw, SIZE};

pub const ENTRIES: &[Entry] = &[
    // ---- Shape Format ----
    Entry::new(
        "Edit\nShape",
        "Changes the selected shape into another shape, or lets you drag its corner points one by one to draw your own outline. Example: a rectangle becomes a rounded speech bubble, or one corner is pulled out to make a pointer.",
        |d| d.ms_edit_shape(),
    ),
    Entry::new(
        "Merge\nShapes",
        "Joins two or more overlapping shapes into one new shape: their union, the part they share, or one cut out of the other. Example: a circle and a rectangle become one badge shape.",
        |d| d.ms_merge_shapes(),
    ),
    Entry::new(
        "Shape\nStyles",
        "Opens a gallery of ready looks for the selected shape — fill, outline and effect together — and applies the one you pick. Example: a plain grey rectangle becomes a filled blue box with a white outline.",
        |d| d.ms_shape_styles(),
    ),
    Entry::new(
        "Text Fill",
        "Colours the letters of the selected text; the shape behind them stays as it is. Example: a black title becomes blue.",
        |d| d.ms_text_fill(),
    ),
    Entry::new(
        "Text Outline",
        "Draws a thin line around each letter of the selected text, in the colour you pick. Example: white letters get a dark outline so they stay readable over a picture.",
        |d| d.ms_text_outline(),
    ),
    Entry::new(
        "Text Effects",
        "Adds a shadow to the selected text or bends it along a curve or a shape. Example: the title gets a soft shadow, or it is written in an arch over the picture.",
        |d| d.ms_text_effects(),
    ),
    Entry::new(
        "Bring\nForward",
        "Moves the selected object one step up in the pile, in front of the object that covered it. Example: a circle hidden behind a square comes out in front of it.",
        |d| d.ms_stack(2, 0, 1),
    ),
    Entry::new(
        "Send\nBackward",
        "Moves the selected object one step down in the pile, behind the object next to it. Example: a square covering a circle goes behind it.",
        |d| d.ms_stack(2, 1, 0),
    ),
    Entry::new(
        "Align",
        "Lines up the selected objects on one edge or centre line, or spreads them out evenly. Example: three boxes at different heights line up along their top edges.",
        |d| d.ms_align(),
    ),
    Entry::new(
        "Group",
        "Joins the selected objects into one group that moves, resizes and copies as a single object; Ungroup takes them apart again. Example: a picture and its caption become one object you can drag together.",
        |d| d.ms_group(Grouping::Group),
    ),
    Entry::new(
        "Rotate",
        "Turns the selected object a quarter turn left or right, or flips it into its mirror image. Example: a wide arrow turned right by 90° points down.",
        |d| d.ms_rotate(true, 1.0),
    ),
    // ---- Picture Format ----
    Entry::new(
        "Remove\nBackground",
        "Cuts the background away from the selected picture so only its subject stays, with the slide showing through around it. Example: a product photo on a white table becomes the product alone.",
        |d| d.ms_remove_background(),
    ),
    Entry::new(
        "Corrections",
        "Makes the selected picture brighter or darker, and its contrast stronger or softer. Example: a photo taken in a dark room is brightened by 40%.",
        |d| d.ms_picture_change(Pic::Dark, Pic::Light),
    ),
    Entry::new(
        "Color",
        "Changes the colours of the selected picture: stronger or weaker colours, black and white, a tint in one colour, or one colour made transparent. Example: a colour photo becomes black and white.",
        |d| d.ms_picture_change(Pic::Normal, Pic::Grey),
    ),
    Entry::new(
        "Artistic\nEffects",
        "Makes the selected picture look like a drawing or a painting: a pencil sketch, a blur, a mosaic. Example: a photo of the building becomes a pencil sketch.",
        |d| d.ms_picture_change(Pic::Normal, Pic::Sketch),
    ),
    Entry::new(
        "Transparency",
        "Makes the selected picture see-through by the amount you pick, so what is behind it shows. Example: at 65% the logo fades into a soft watermark behind the text.",
        |d| d.ms_transparency(),
    ),
    Entry::new(
        "Compress Pictures",
        "Makes the pictures smaller in file size — fewer pixels, cropped parts thrown away — so the presentation is lighter to send. Example: a 4 MB photo becomes 400 KB with no visible change on screen.",
        |d| d.ms_compress(),
    ),
    Entry::new(
        "Picture Border",
        "Draws a frame line around the selected picture in the colour, weight and dash style you pick. Example: a thin black line around the photo.",
        |d| d.ms_picture_border(),
    ),
    Entry::new(
        "Crop",
        "Cuts away the edges of the selected picture you do not want, or trims it to a shape. Example: the empty sky above the building is cut off.",
        |d| d.ms_crop(),
    ),
    // ---- Table Design ----
    Entry::new("Shading", "Fills the selected table cells with a colour. Example: the total row is filled light blue so it stands out.", |d| {
        d.ms_shading()
    }),
    Entry::new(
        "Borders",
        "Draws or removes the lines around and between the selected table cells, with the pen weight, style and colour you set. Example: all borders of the table drawn in a thin black line.",
        |d| d.ms_borders(),
    ),
    // ---- Table Layout ----
    Entry::new(
        "Delete",
        "Removes the selected rows or columns from the table, or the whole table. Example: the empty «Notes» column is deleted and the other columns close the gap.",
        |d| d.ms_table_delete(),
    ),
    // ---- Chart Design ----
    Entry::new(
        "Change\nChart Type",
        "Shows the same numbers as another kind of chart: columns, bars, a line, a pie. Example: the column chart of monthly sales becomes a line chart.",
        |d| d.ms_chart_type(),
    ),
    // ---- SmartArt Design ----
    Entry::new(
        "Text\nPane",
        "Opens a side pane where you type the diagram's text as a plain list; every line is one box and indented lines are sub-boxes. Example: typing a fourth line adds a fourth step to the process.",
        |d| d.ms_text_pane(),
    ),
    Entry::new(
        "Add Shape",
        "Adds one more box to the diagram after, before, above or below the selected one. Example: a three-step process gets a fourth step.",
        |d| d.ms_add_shape(),
    ),
    Entry::new(
        "Promote",
        "Moves the selected box one level up in the diagram, so a sub-item becomes a main item. Example: a point under «Costs» becomes a heading of its own.",
        |d| d.ms_level(true),
    ),
    Entry::new(
        "Demote",
        "Moves the selected box one level down in the diagram, so a main item becomes a sub-item of the one before it. Example: «Travel» becomes a point under «Costs».",
        |d| d.ms_level(false),
    ),
    Entry::new(
        "Insert Picture",
        "Puts a picture of your choice into the picture frame of the selected diagram box. Example: each team member's box in the organisation chart shows their photo.",
        |d| d.ms_smartart_picture(),
    ),
    Entry::new(
        "Layouts",
        "Opens a gallery of diagram layouts and rearranges the same boxes and text into the one you pick. Example: a row of steps becomes a cycle.",
        |d| d.ms_layouts(),
    ),
    Entry::new(
        "Convert",
        "Turns the diagram back into ordinary text or into separate shapes you can edit one by one. Example: the four steps become a bulleted list of four lines.",
        |d| d.ms_convert(),
    ),
    Entry::new(
        "Change Colors",
        "Applies another colour scheme to the whole diagram: one colour in shades, or a different colour for every box. Example: all-blue boxes become blue, orange, green and purple.",
        |d| d.ms_smartart_pair(Sa::plain(), Sa { colourful: true, ..Sa::plain() }),
    ),
    Entry::new(
        "SmartArt Styles",
        "Applies a ready look to every box of the diagram: flat, outlined, with a shadow or in 3-D. Example: the flat boxes get a soft shadow and a glossy top.",
        |d| d.ms_smartart_pair(Sa::plain(), Sa { styled: true, ..Sa::plain() }),
    ),
    // ---- Playback ----
    Entry::new(
        "Volume",
        "Sets how loud the selected audio or video plays during the show: low, medium, high or muted. Example: the background music is set to low so the speaker is heard.",
        |d| d.ms_volume(),
    ),
    Entry::new(
        "Alt\nText",
        "Lets you write a short description of the selected picture or shape, read aloud by screen readers to people who cannot see it. Example: «Map of the clinic's three buildings».",
        |d| d.ms_alt_text(),
    ),
    Entry::new(
        "Format\nPane",
        "Opens the side pane with every setting of the selected object — fill, line, effects, size and text. Example: set the fill to a gradient and the line to 2 pt.",
        |d| d.ms_format_pane(),
    ),
    Entry::new(
        "Change Picture",
        "Swaps the selected picture for another file, keeping the same place, size and style on the slide. Example: last year's photo is replaced by this year's in the same frame.",
        |d| d.ms_picture_change(Pic::Normal, Pic::City),
    ),
    Entry::new(
        "Reset Picture",
        "Throws away every change made to the selected picture — colour, crop, effects, style — and shows it as it was inserted. Example: the greyed, cropped photo is back in full colour.",
        |d| d.ms_reset_picture(),
    ),
    Entry::new(
        "Insert\nAbove",
        "Adds a new empty row to the table above the selected cell. Example: a row for January is added above the February row.",
        |d| d.ms_table_insert(Side::Above),
    ),
    Entry::new(
        "Insert\nBelow",
        "Adds a new empty row to the table below the selected cell. Example: a row for a new employee is added under the last one.",
        |d| d.ms_table_insert(Side::Below),
    ),
    Entry::new(
        "Insert\nLeft",
        "Adds a new empty column to the table to the left of the selected cell. Example: a «Total» column is added at the left end.",
        |d| d.ms_table_insert(Side::Left),
    ),
    Entry::new(
        "Insert\nRight",
        "Adds a new empty column to the table to the right of the selected cell. Example: a column for a new month is added at the right end.",
        |d| d.ms_table_insert(Side::Right),
    ),
    Entry::new(
        "Merge\nCells",
        "Joins the selected cells into one bigger cell. Example: the three cells of the top row become one wide title cell.",
        |d| d.ms_merge_cells(),
    ),
    Entry::new(
        "Split\nCells",
        "Divides the selected cell into the number of rows and columns you choose. Example: one wide cell becomes two side by side.",
        |d| d.ms_split_cells(),
    ),
    Entry::new(
        "Distribute Rows",
        "Makes the selected rows all the same height, sharing the space they take now. Example: one tall row and two short ones become three equal rows.",
        |d| d.ms_distribute(true),
    ),
    Entry::new(
        "Distribute Columns",
        "Makes the selected columns all the same width, sharing the space they take now. Example: a wide column and two narrow ones become three equal columns.",
        |d| d.ms_distribute(false),
    ),
    Entry::new(
        "Edit\nData",
        "Opens the small table of numbers behind the chart so you can change them; the chart redraws as you type. Example: change March from 120 to 150 and its column grows.",
        |d| d.ms_edit_data(),
    ),
    Entry::new(
        "Trim",
        "Cuts the start and the end off the selected audio or video, so only the part between the two markers plays. Example: the first 5 seconds of silence are trimmed away.",
        |d| d.ms_trim(),
    ),
    // ---- File menu ----
    Entry::new("file.new", "Starts a new, empty presentation with one title slide. Example: a blank deck ready for the quarterly report.", |d| {
        d.ms_new_presentation()
    }),
    Entry::new(
        "app.openDialog",
        "Opens a presentation saved on your computer, chosen from a file window. Example: open last week's «Training.pptx» to continue it.",
        |d| d.ms_open(),
    ),
    Entry::new(
        "app.closePresentation",
        "Closes the current presentation, asking to save first if it has unsaved changes; DeckCraft stays open. Example: close the finished deck and start another.",
        |d| d.ms_close(),
    ),
    Entry::new(
        "file.save",
        "Writes the presentation to its file on disk, keeping every change you made. Example: press it after each finished slide so nothing is lost.",
        |d| d.ms_save(),
    ),
    Entry::new(
        "app.saveAsDialog",
        "Saves a copy of the presentation under a new name or in another folder; the original stays as it was. Example: «Report v2.pptx» next to «Report.pptx».",
        |d| d.ms_save_as(),
    ),
    Entry::new(
        "app.exportDialog",
        "Makes another kind of file from the presentation: a PDF, a video, or a picture of each slide. Example: a PDF of the deck to send by e-mail.",
        |d| d.ms_export(),
    ),
    Entry::new(
        "app.print",
        "Prints the slides on paper: full slides, handouts with several slides per page, or notes pages. Example: six slides per page as a handout for the audience.",
        |d| d.ms_print(),
    ),
    Entry::new(
        "file.properties",
        "Shows the facts about the file: its title, author, size, number of slides and when it was changed. Example: see that the deck has 24 slides and was last saved on Tuesday.",
        |d| d.ms_properties(),
    ),
    // ---- Edit menu ----
    Entry::new(
        "edit.undo",
        "Takes back the last thing you did, and the one before that each time you press it again. Example: a deleted picture comes back.",
        |d| d.ms_undo(false),
    ),
    Entry::new("edit.redo", "Does again what Undo just took back. Example: the picture is deleted again.", |d| d.ms_undo(true)),
    Entry::new(
        "edit.cut",
        "Takes the selected object or text off the slide and keeps it on the clipboard, so you can paste it somewhere else. Example: cut a chart from slide 2 and paste it on slide 5.",
        |d| d.ms_clip(Clip::Cut),
    ),
    Entry::new(
        "edit.copy",
        "Keeps a copy of the selected object or text on the clipboard to paste elsewhere; the original stays where it is. Example: copy the logo and paste it on every slide.",
        |d| d.ms_clip(Clip::Copy),
    ),
    Entry::new(
        "edit.paste",
        "Puts what you cut or copied onto the current slide, or into the text where the cursor is. Example: the copied logo appears on this slide.",
        |d| d.ms_clip(Clip::Paste),
    ),
    Entry::new(
        "edit.pasteText",
        "Pastes copied text so it takes the font, size and colour of the text around it, instead of bringing its own. Example: red bold text from a web page is pasted as the slide's plain black text.",
        |d| d.ms_clip(Clip::PasteText),
    ),
    Entry::new(
        "app.pasteSpecial",
        "Lets you choose in what form to paste what you copied: a picture, plain text, or the original object. Example: paste a table from a spreadsheet as a picture so it cannot be changed.",
        |d| d.ms_paste_special(),
    ),
    Entry::new(
        "edit.selectAll",
        "Selects everything on the slide — every shape, picture and text box — or every slide in the slide list. Example: select all, then move the whole layout a little to the right.",
        |d| d.ms_select_all(),
    ),
    Entry::new(
        "edit.duplicate",
        "Makes a copy of the selected object right next to the original, in one step without the clipboard. Example: one box becomes two identical boxes.",
        |d| d.ms_duplicate(),
    ),
    Entry::new(
        "slide.delete",
        "Removes the selected slide from the presentation; the slides after it move up. Example: slide 3 is deleted and slide 4 becomes the new slide 3.",
        |d| d.ms_delete_slide(),
    ),
    Entry::new(
        "app.find",
        "Looks for a word or phrase in all the slides and shows where it is. Example: find every slide that mentions «Basra».",
        |d| d.ms_find(),
    ),
    Entry::new(
        "app.replace",
        "Finds a word or phrase in all the slides and swaps it for another, one at a time or all at once. Example: replace «2025» with «2026» everywhere.",
        |d| d.ms_replace(),
    ),
    // ---- View menu ----
    Entry::new(
        "view.normal",
        "The everyday view for editing: the slide list on the side, the current slide large in the middle, and the notes under it. Example: click a slide on the side to edit it.",
        |d| d.ms_view_normal(),
    ),
    Entry::new(
        "view.sorter",
        "Shows all the slides small, side by side, so you can see the whole deck and drag slides into another order. Example: drag slide 7 to come before slide 3.",
        |d| d.ms_view_sorter(),
    ),
    Entry::new(
        "view.notesPage",
        "Shows each slide on a page with its speaker notes below, the way they print. Example: write the full speech under the slide picture.",
        |d| d.ms_view_notes_page(),
    ),
    Entry::new(
        "view.outline",
        "Shows only the text of all the slides as a list — titles and points — so you can write and reorder the content fast. Example: type a new title and a slide is made for it.",
        |d| d.ms_view_outline(),
    ),
    Entry::new(
        "view.reading",
        "Plays the presentation inside the window instead of the full screen, with buttons to move between slides. Example: check the slides while another window stays visible.",
        |d| d.ms_view_reading(),
    ),
    Entry::new(
        "show.start",
        "Starts the slide show full screen from the first slide. Example: the screen goes to the title slide; click or press the arrow to continue.",
        |d| d.ms_show(0),
    ),
    Entry::new(
        "view.slideMaster",
        "Opens the master slide and its layouts — the designs every slide is based on; change them here and all the slides follow. Example: put the logo on the master and it appears on every slide.",
        |d| d.ms_view_master(Master::Slide),
    ),
    Entry::new(
        "view.handoutMaster",
        "Opens the design of the printed handout pages: how many slides per page and the header and footer. Example: three slides per page with lines for notes.",
        |d| d.ms_view_master(Master::Handout),
    ),
    Entry::new(
        "view.notesMaster",
        "Opens the design of the notes pages: where the slide picture and the notes text go and what they look like. Example: a larger notes area in a bigger font.",
        |d| d.ms_view_master(Master::Notes),
    ),
    Entry::new(
        "view.ruler",
        "Shows or hides the rulers along the top and the side of the slide, for placing things by measure. Example: see that the picture starts 2 cm from the edge.",
        |d| d.ms_view_helper(Helper::Ruler),
    ),
    Entry::new(
        "view.gridlines",
        "Shows or hides a grid of dots over the slide to line things up by eye; it is not printed or shown in the show. Example: place the three boxes on the same grid line.",
        |d| d.ms_view_helper(Helper::Grid),
    ),
    Entry::new(
        "view.guides",
        "Shows or hides a vertical and a horizontal guide line through the middle of the slide; objects snap to them as you drag. Example: centre the title on the vertical guide.",
        |d| d.ms_view_helper(Helper::Guides),
    ),
    Entry::new(
        "view.gridOptions",
        "Sets how far apart the grid dots are and whether objects snap to the grid and to each other. Example: a 1 cm grid with snapping on.",
        |d| d.ms_grid_options(),
    ),
    Entry::new(
        "view.zoomDialog",
        "Sets the size the slide is shown at, as a percentage you pick or type. Example: 200% to work on a small detail.",
        |d| d.ms_zoom_dialog(),
    ),
    Entry::new("view.zoomIn", "Shows the slide bigger, one step at a time. Example: from 100% to 125%.", |d| d.ms_zoom(true)),
    Entry::new("view.zoomOut", "Shows the slide smaller, one step at a time. Example: from 100% to 75%.", |d| d.ms_zoom(false)),
    Entry::new(
        "view.fit",
        "Sizes the slide so the whole of it fits in the window. Example: after zooming in on a detail, see the whole slide again.",
        |d| d.ms_fit(),
    ),
    Entry::new(
        "view.newWindow",
        "Opens the same presentation in a second window, so you can look at two slides at once. Example: slide 2 in one window while editing slide 9 in the other.",
        |d| d.ms_new_window(),
    ),
    // ---- Insert menu ----
    Entry::new(
        "slide.new",
        "Adds a new slide after the current one, with the same layout. Example: after the title slide, a new slide with a title and a text area.",
        |d| d.ms_new_slide(false),
    ),
    Entry::new(
        "slide.duplicate",
        "Adds a copy of the current slide right after it, with everything on it. Example: copy the table slide and change only the numbers.",
        |d| d.ms_new_slide(true),
    ),
    Entry::new(
        "section.add",
        "Starts a named section at the current slide, to group the slides into chapters you can collapse and move together. Example: a «Results» section from slide 8 on.",
        |d| d.ms_section(),
    ),
    Entry::new(
        "insert.textBox",
        "Adds a box you can type in anywhere on the slide; drag to set its size. Example: a caption under the picture.",
        |d| d.ms_insert_text_box(),
    ),
    Entry::new(
        "insert.wordArt",
        "Adds decorative text with a fill, an outline and effects, for a big title or a word to stand out. Example: the word «Welcome» in large outlined letters with a shadow.",
        |d| d.ms_word_art(),
    ),
    Entry::new(
        "insert.dateTime",
        "Puts today's date, and the time if you want, in the slide's footer; it can update itself every time the file is opened. Example: «18/10/2026» at the bottom of every slide.",
        |d| d.ms_date_time(),
    ),
    Entry::new(
        "insert.slideNumber",
        "Puts the slide's number in its footer, updated when slides are added or moved. Example: «3» in the corner of the third slide.",
        |d| d.ms_slide_number(),
    ),
    Entry::new(
        "insert.table",
        "Adds a table to the slide with the number of rows and columns you choose. Example: a table of 4 rows and 3 columns for the monthly figures.",
        |d| d.ms_insert_table(),
    ),
    Entry::new(
        "insert.chart",
        "Adds a chart to the slide — columns, a line, a pie — drawn from numbers you type into a small table. Example: a column chart of sales for each month.",
        |d| d.ms_insert_chart(),
    ),
    Entry::new(
        "app.insertPictureDialog",
        "Puts a picture from a file on your computer onto the slide. Example: a photo of the new building from the Pictures folder.",
        |d| d.ms_insert_picture(),
    ),
    Entry::new(
        "insert.photoAlbum",
        "Makes a new presentation from a set of pictures, one or more on each slide, with captions if you want. Example: the 30 photos of the trip become a 30-slide album.",
        |d| d.ms_photo_album(false),
    ),
    Entry::new(
        "insert.photoAlbumEdit",
        "Changes a photo album you made: add or remove pictures, change their order, their layout and their captions. Example: move the group photo to the first slide.",
        |d| d.ms_photo_album(true),
    ),
    Entry::new(
        "app.insertAudioDialog",
        "Puts a sound file on the slide, shown as a small speaker you can play during the show. Example: a recorded welcome message that plays when the slide opens.",
        |d| d.ms_insert_audio(),
    ),
    Entry::new(
        "app.insertVideoDialog",
        "Puts a video file on the slide, with its own play button during the show. Example: a 30-second clip of the machine at work.",
        |d| d.ms_insert_video(),
    ),
    // ---- Format menu ----
    Entry::new(
        "format.fontDialog",
        "Opens the window with every text setting at once: font, size, style, colour, underline and effects. Example: set the title to 32 pt bold in dark blue.",
        |d| d.ms_font_dialog(),
    ),
    Entry::new(
        "format.paragraphDialog",
        "Opens the window with every paragraph setting at once: alignment, indents, line spacing and space before and after. Example: 1.5 line spacing with 6 pt after each paragraph.",
        |d| d.ms_paragraph_dialog(),
    ),
    Entry::new(
        "format.bold",
        "Makes the selected text bold: thicker, darker letters that stand out. Example: the name of the product is made bold.",
        |d| d.ms_text_style(TextStyle::Bold),
    ),
    Entry::new("format.italic", "Slants the selected text to the side, like handwriting. Example: a quotation is written slanted.", |d| {
        d.ms_text_style(TextStyle::Italic)
    }),
    Entry::new("format.underline", "Draws a line under the selected text. Example: the key figure is underlined.", |d| {
        d.ms_text_style(TextStyle::Underline)
    }),
    Entry::new(
        "format.alignLeft",
        "Lines the text up along the left edge of its box, leaving the right edge ragged. Example: English text lined up on the left.",
        |d| d.ms_text_align(TextAlign::Left),
    ),
    Entry::new("format.alignCenter", "Centres every line of the text in its box. Example: a title centred over the slide.", |d| {
        d.ms_text_align(TextAlign::Center)
    }),
    Entry::new(
        "format.alignRight",
        "Lines the text up along the right edge of its box, leaving the left edge ragged. Example: Arabic text lined up on the right.",
        |d| d.ms_text_align(TextAlign::Right),
    ),
    Entry::new(
        "format.justify",
        "Stretches every line of the text to reach both edges of its box, widening the spaces between words. Example: a paragraph with straight edges on both sides.",
        |d| d.ms_text_align(TextAlign::Justify),
    ),
    Entry::new(
        "shape.sizePosition",
        "Opens the window where you type the exact width, height, position and rotation of the selected object. Example: a picture exactly 10 cm wide, 2 cm from the top.",
        |d| d.ms_size_position(),
    ),
    Entry::new(
        "format.rtl",
        "Makes the selected text run from right to left, as Arabic does, with its punctuation and numbers placed the Arabic way. Example: a sentence pasted the wrong way round reads correctly again.",
        |d| d.ms_rtl(),
    ),
    // ---- Arrange menu ----
    Entry::new(
        "arrange.bringToFront",
        "Moves the selected object to the very top of the pile, in front of everything else on the slide. Example: a label hidden under two pictures comes out on top of both.",
        |d| d.ms_stack(3, 0, 2),
    ),
    Entry::new(
        "arrange.sendToBack",
        "Moves the selected object to the very bottom of the pile, behind everything else on the slide. Example: a big rectangle goes behind the text and pictures as a background.",
        |d| d.ms_stack(3, 2, 0),
    ),
    Entry::new(
        "arrange.bringForward",
        "Moves the selected object one step up in the pile, in front of the object that covered it. Example: a circle hidden behind a square comes out in front of it.",
        |d| d.ms_stack(2, 0, 1),
    ),
    Entry::new(
        "arrange.sendBackward",
        "Moves the selected object one step down in the pile, behind the object next to it. Example: a square covering a circle goes behind it.",
        |d| d.ms_stack(2, 1, 0),
    ),
    Entry::new(
        "arrange.group",
        "Joins the selected objects into one group that moves, resizes and copies as a single object. Example: a picture and its caption become one object you can drag together.",
        |d| d.ms_group(Grouping::Group),
    ),
    Entry::new(
        "arrange.ungroup",
        "Takes a group apart into its separate objects again, each with its own handles. Example: the diagram's boxes can be moved one by one.",
        |d| d.ms_group(Grouping::Ungroup),
    ),
    Entry::new(
        "arrange.regroup",
        "Puts back together the objects of a group you took apart, without selecting them all again. Example: after fixing one box, the diagram is one group again.",
        |d| d.ms_group(Grouping::Regroup),
    ),
    Entry::new(
        "arrange.rotateLeft",
        "Turns the selected object a quarter turn to the left (anticlockwise). Example: an arrow pointing right points up.",
        |d| d.ms_rotate(false, -1.0),
    ),
    Entry::new(
        "arrange.rotateRight",
        "Turns the selected object a quarter turn to the right (clockwise). Example: an arrow pointing right points down.",
        |d| d.ms_rotate(false, 1.0),
    ),
    Entry::new(
        "arrange.flipHorizontal",
        "Mirrors the selected object left to right, as in a mirror beside it. Example: an arrow pointing right points left.",
        |d| d.ms_flip(true),
    ),
    Entry::new(
        "arrange.flipVertical",
        "Mirrors the selected object top to bottom, as in a mirror under it. Example: an arrow pointing up points down.",
        |d| d.ms_flip(false),
    ),
    // ---- Slide Show menu ----
    Entry::new(
        "show.fromStart",
        "Starts the slide show full screen from the first slide, whichever slide you are on. Example: the show begins with the title slide.",
        |d| d.ms_show(0),
    ),
    Entry::new(
        "show.fromCurrent",
        "Starts the slide show full screen from the slide you are on. Example: rehearse the last part from slide 12 without going through the beginning.",
        |d| d.ms_show(2),
    ),
    Entry::new(
        "slide.hide",
        "Keeps the selected slide in the file but skips it during the show; its number is crossed out in the slide list. Example: the slide with the backup figures is hidden from the audience.",
        |d| d.ms_hide_slide(),
    ),
    Entry::new(
        "show.setup",
        "Sets how the show runs: on which screen, full screen or in a window, looping or not, and which slides to show. Example: loop the show on the reception screen all day.",
        |d| d.ms_setup_show(),
    ),
    // ---- Help menu ----
    Entry::new(
        "app.about",
        "Shows which version of DeckCraft this is and who made it. Example: the version number to quote when you report a problem.",
        |d| d.ms_about(),
    ),
    Entry::new(
        "app.palette",
        "Opens a search box where you type a few letters of what you want to do and pick the command from the list, instead of looking for its button. Example: type «hide» and choose Hide Slide.",
        |d| d.ms_palette(),
    ),
];

/// How a drawn picture looks: as inserted, or after one of the picture commands.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Pic {
    Normal,
    Grey,
    Light,
    Dark,
    Sketch,
    Faded,
    NoSky,
    City,
}

/// What a group command does to three shapes.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Grouping {
    Group,
    Ungroup,
    Regroup,
}

/// Where a table row or column is inserted.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Side {
    Above,
    Below,
    Left,
    Right,
}

/// A clipboard command.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Clip {
    Cut,
    Copy,
    Paste,
    PasteText,
}

/// A master view.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Master {
    Slide,
    Handout,
    Notes,
}

/// A drawing aid over the slide.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Helper {
    Ruler,
    Grid,
    Guides,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum TextStyle {
    Bold,
    Italic,
    Underline,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
}

/// The look of a drawn SmartArt diagram.
#[derive(Clone, Copy)]
pub(super) struct Sa {
    pub colourful: bool,
    pub styled: bool,
    pub pictures: bool,
    /// The box drawn in the full accent colour (the new or selected one).
    pub hot: Option<usize>,
}

impl Sa {
    pub(super) const fn plain() -> Self {
        Self { colourful: false, styled: false, pictures: false, hot: None }
    }
}

/// `n` in Arabic-Indic digits, for the numbers drawn on slides.
pub(super) fn ms_num(n: usize) -> String {
    n.to_string()
        .chars()
        .map(|c| match c {
            '0' => '٠',
            '1' => '١',
            '2' => '٢',
            '3' => '٣',
            '4' => '٤',
            '5' => '٥',
            '6' => '٦',
            '7' => '٧',
            '8' => '٨',
            '9' => '٩',
            other => other,
        })
        .collect()
}

const SLIDE_RATIO: f32 = 9.0 / 16.0;

impl Draw<'_> {
    // ---- slides ----

    /// A 16:9 slide `w` wide at `x, y`; returns its content area.
    pub(super) fn ms_slide(&self, x: f32, y: f32, w: f32) -> Rect {
        self.ms_slide_rect(Rect::from_min_size(self.at(x, y), vec2(w, w * SLIDE_RATIO)))
    }
    /// A slide filling `r`; returns its content area.
    pub(super) fn ms_slide_rect(&self, r: Rect) -> Rect {
        self.p.rect_filled(r, 2.0, Color32::WHITE);
        self.p.rect_stroke(r, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        r.shrink(r.width() * 0.07)
    }
    /// One large slide in the middle of the thumbnail; returns its content area.
    pub(super) fn ms_slide_big(&self) -> Rect {
        self.ms_slide(20.0, 13.5, 136.0)
    }
    /// The outer rect of [`Draw::ms_slide_big`].
    pub(super) fn ms_slide_big_rect(&self) -> Rect {
        Rect::from_min_size(self.at(20.0, 13.5), vec2(136.0, 136.0 * SLIDE_RATIO))
    }
    /// A title line (right-aligned, thick) and `n` body lines on a slide's content area.
    pub(super) fn ms_title_body(&self, a: Rect, n: usize) {
        let y = a.min.y + 3.0;
        self.line(pos2(a.max.x - a.width() * 0.6, y), pos2(a.max.x, y), self.ink(), 2.5);
        if n == 0 {
            return;
        }
        let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 7.0), a.max);
        let pitch = ((body.height() - 6.0) / n as f32).clamp(4.0, 8.0);
        let widths: Vec<f32> = (0..n).map(|i| body.width() * [0.9, 0.72, 0.84, 0.6].get(i % 4).copied().unwrap_or(0.8)).collect();
        self.text_lines(body, true, &widths, pitch, self.grey());
    }
    /// Two slides with an arrow between them: `before(area)` then `after(area)`, the second
    /// wiped in as the film plays.
    pub(super) fn ms_before_after(&self, before: impl FnOnce(&Draw<'_>, Rect), after: impl FnOnce(&Draw<'_>, Rect)) {
        const W: f32 = 76.0;
        let h = W * SLIDE_RATIO;
        let y = (SIZE.y - h) / 2.0;
        let b = self.ms_slide(4.0, y, W);
        before(self, b);
        let arrow = self.reveal(0.05, 0.3);
        let mid = y + h / 2.0;
        if arrow > 0.0 {
            self.arrow_right(self.at(82.0, mid), self.at(82.0 + 12.0 * arrow, mid));
        }
        let outer = Rect::from_min_size(self.at(96.0, y), vec2(W, h));
        let wipe = self.reveal(0.3, 0.75);
        if wipe <= 0.0 {
            self.p.rect_filled(outer, 2.0, Color32::WHITE);
            self.p.rect_stroke(outer, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
            return;
        }
        let clip = Rect::from_min_max(outer.min - vec2(2.0, 2.0), pos2(outer.min.x + outer.width() * wipe + 2.0, outer.max.y + 2.0));
        let clipped = self.p.with_clip_rect(clip.intersect(self.p.clip_rect()));
        let d = self.with_painter(&clipped);
        let a = d.ms_slide_rect(outer);
        after(&d, a);
        if wipe < 1.0 {
            let rest =
                self.p.with_clip_rect(Rect::from_min_max(pos2(clip.max.x, clip.min.y), outer.max + vec2(2.0, 2.0)).intersect(self.p.clip_rect()));
            rest.rect_filled(outer, 2.0, Color32::WHITE);
            rest.rect_stroke(outer, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        }
    }
    /// A row of `n` numbered slide thumbnails `w` wide at `y`, the `hi` one outlined in the
    /// accent; returns their outer rects.
    pub(super) fn ms_strip(&self, n: usize, w: f32, y: f32, hi: Option<usize>) -> Vec<Rect> {
        let gap = 6.0;
        let total = n as f32 * (w + gap) - gap;
        let x0 = (SIZE.x - total) / 2.0;
        (0..n)
            .map(|i| {
                let outer = Rect::from_min_size(self.at(x0 + i as f32 * (w + gap), y), vec2(w, w * SLIDE_RATIO));
                let a = self.ms_slide_rect(outer);
                self.ms_title_body(a, 2);
                if hi == Some(i) {
                    self.p.rect_stroke(outer.expand(1.5), 3.0, Stroke::new(1.5, self.accent()), egui::StrokeKind::Outside);
                }
                self.label(pos2(outer.center().x, outer.max.y + 7.0), &ms_num(i + 1), 7.0, self.grey(), Align2::CENTER_CENTER);
                outer
            })
            .collect()
    }
    /// A window frame (title bar with three dots) filling `r`; returns its content area.
    pub(super) fn ms_window(&self, r: Rect) -> Rect {
        self.p.rect_filled(r, 3.0, self.t.menu);
        self.p.rect_stroke(r, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.p.rect_filled(Rect::from_min_size(r.min, vec2(r.width(), 9.0)), 3.0, self.t.hover);
        for i in 0..3 {
            self.p.circle_filled(pos2(r.min.x + 6.0 + i as f32 * 6.0, r.min.y + 4.5), 1.6, self.grey());
        }
        Rect::from_min_max(pos2(r.min.x + 3.0, r.min.y + 12.0), r.max - vec2(3.0, 3.0))
    }
    /// A picture in `r`, drawn as `look` says.
    pub(super) fn ms_picture(&self, r: Rect, look: Pic) {
        let tint = |c: Color32| -> Color32 {
            match look {
                Pic::Grey => {
                    let y = 0.299 * f32::from(c.r()) + 0.587 * f32::from(c.g()) + 0.114 * f32::from(c.b());
                    Color32::from_gray(y.clamp(0.0, 255.0) as u8)
                }
                Pic::Light => c.lerp_to_gamma(Color32::WHITE, 0.45),
                Pic::Dark => c.lerp_to_gamma(Color32::BLACK, 0.45),
                Pic::Faded => c.gamma_multiply(0.35),
                _ => c,
            }
        };
        let sky = tint(if look == Pic::City { Color32::from_rgb(250, 214, 170) } else { Color32::from_rgb(214, 232, 246) });
        let edge = self.stroke(if look == Pic::Faded { self.t.border } else { self.t.border_strong });
        match look {
            Pic::NoSky => self.dashed_rect(r, self.t.border_strong),
            Pic::Sketch => {
                self.p.rect_filled(r, 1.0, Color32::WHITE);
                self.p.rect_stroke(r, 1.0, edge, egui::StrokeKind::Inside);
            }
            _ => {
                self.p.rect_filled(r, 1.0, sky);
                self.p.rect_stroke(r, 1.0, edge, egui::StrokeKind::Inside);
            }
        }
        let base = r.max.y - 2.0;
        if look == Pic::City {
            for (k, h) in [0.5, 0.75, 0.4, 0.65].iter().enumerate() {
                let w = r.width() * 0.2;
                let x = r.min.x + 3.0 + k as f32 * (w + 1.0);
                self.p.rect_filled(Rect::from_min_max(pos2(x, base - r.height() * h), pos2(x + w, base)), 0.0, tint(Color32::from_rgb(70, 80, 110)));
            }
            self.p.circle_filled(
                pos2(r.max.x - r.width() * 0.25, r.min.y + r.height() * 0.3),
                r.height() * 0.13,
                tint(Color32::from_rgb(240, 130, 50)),
            );
            return;
        }
        let hill =
            vec![pos2(r.min.x + 2.0, base), pos2(r.min.x + r.width() * 0.35, r.min.y + r.height() * 0.45), pos2(r.min.x + r.width() * 0.6, base)];
        let hill2 =
            vec![pos2(r.min.x + r.width() * 0.4, base), pos2(r.min.x + r.width() * 0.7, r.min.y + r.height() * 0.3), pos2(r.max.x - 2.0, base)];
        let sun = pos2(r.min.x + r.width() * 0.22, r.min.y + r.height() * 0.25);
        if look == Pic::Sketch {
            let pen = Stroke::new(1.0, self.grey());
            self.p.add(Shape::closed_line(hill, pen));
            self.p.add(Shape::closed_line(hill2, pen));
            self.p.circle_stroke(sun, r.height() * 0.1, pen);
            return;
        }
        self.p.add(Shape::convex_polygon(hill, tint(Color32::from_rgb(96, 150, 96)), Stroke::NONE));
        self.p.add(Shape::convex_polygon(hill2, tint(Color32::from_rgb(70, 120, 80)), Stroke::NONE));
        self.p.circle_filled(sun, r.height() * 0.1, tint(Color32::from_rgb(240, 190, 60)));
    }
    /// A clipboard (board with a clip) centred at `c`; returns the area for what it holds.
    pub(super) fn ms_clipboard(&self, c: Pos2) -> Rect {
        let r = Rect::from_center_size(c, vec2(26.0, 32.0));
        self.p.rect_filled(r, 2.0, Color32::from_rgb(222, 196, 150));
        self.p.rect_stroke(r, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        let clip = Rect::from_center_size(pos2(c.x, r.min.y), vec2(12.0, 6.0));
        self.p.rect_filled(clip, 1.0, self.t.text_dim);
        Rect::from_min_max(pos2(r.min.x + 4.0, r.min.y + 6.0), r.max - vec2(4.0, 4.0))
    }
    /// A folder centred at `c`.
    pub(super) fn ms_folder(&self, c: Pos2) {
        let body = Rect::from_center_size(c, vec2(40.0, 28.0));
        let tab = Rect::from_min_size(pos2(body.min.x, body.min.y - 5.0), vec2(16.0, 7.0));
        let fill = Color32::from_rgb(240, 196, 90);
        self.p.rect_filled(tab, 2.0, fill);
        self.p.rect_filled(body, 2.0, fill);
        self.p.rect_stroke(body, 2.0, self.stroke(Color32::from_rgb(200, 150, 50)), egui::StrokeKind::Inside);
    }
    /// A play triangle centred at `c`.
    pub(super) fn ms_play(&self, c: Pos2, r: f32, fill: Color32) {
        self.p.add(Shape::convex_polygon(vec![c + vec2(-r * 0.7, -r), c + vec2(r, 0.0), c + vec2(-r * 0.7, r)], fill, Stroke::NONE));
    }
    /// An arrow shape (a wide arrow pointing along `dir`) centred at `c`, `len` long.
    pub(super) fn ms_fat_arrow(&self, c: Pos2, dir: Vec2, len: f32, fill: Color32) {
        let n = vec2(-dir.y, dir.x);
        let half = len / 2.0;
        let pts = vec![
            c - dir * half - n * 4.0,
            c + dir * (half - 8.0) - n * 4.0,
            c + dir * (half - 8.0) - n * 9.0,
            c + dir * half,
            c + dir * (half - 8.0) + n * 9.0,
            c + dir * (half - 8.0) + n * 4.0,
            c - dir * half + n * 4.0,
        ];
        self.p.add(Shape::convex_polygon(pts.clone(), fill, Stroke::NONE));
        self.p.add(Shape::closed_line(pts, Stroke::new(1.0, self.t.accent)));
    }
    /// A curved arrow (three quarters of a circle) around `c`, anticlockwise when `ccw`.
    pub(super) fn ms_curve_arrow(&self, c: Pos2, radius: f32, ccw: bool, k: f32) {
        let n = 16;
        let sweep = std::f32::consts::PI * 1.5 * k.max(0.05);
        let pts: Vec<Pos2> = (0..=n)
            .map(|i| {
                let a = -std::f32::consts::FRAC_PI_2 + sweep * i as f32 / n as f32 * if ccw { -1.0 } else { 1.0 };
                c + vec2(a.cos(), a.sin()) * radius
            })
            .collect();
        let Some(&end) = pts.last() else { return };
        let Some(&prev) = pts.get(n - 1) else { return };
        self.p.add(Shape::line(pts.clone(), Stroke::new(1.8, self.t.accent)));
        let t = (end - prev).normalized();
        let nn = vec2(-t.y, t.x);
        self.line(end, end - t * 5.0 + nn * 4.0, self.t.accent, 1.8);
        self.line(end, end - t * 5.0 - nn * 4.0, self.t.accent, 1.8);
    }
    /// A `rows × cols` table in `r` with relative row heights and column widths.
    pub(super) fn ms_table_w(&self, r: Rect, rows: &[f32], cols: &[f32]) {
        self.p.rect_filled(r, 1.0, Color32::WHITE);
        let rs: f32 = rows.iter().sum::<f32>().max(0.001);
        let cs: f32 = cols.iter().sum::<f32>().max(0.001);
        let mut y = r.min.y;
        for (i, h) in rows.iter().enumerate() {
            let rh = r.height() * h / rs;
            if i == 0 {
                self.p.rect_filled(Rect::from_min_size(pos2(r.min.x, y), vec2(r.width(), rh)), 0.0, self.t.hover);
            }
            let mut x = r.min.x;
            for w in cols {
                let cw = r.width() * w / cs;
                let ty = y + rh / 2.0;
                self.line(pos2(x + cw - 3.0, ty), pos2(x + cw - 3.0 - (cw - 6.0) * 0.6, ty), if i == 0 { self.t.text } else { self.t.text_dim }, 1.5);
                x += cw;
                if x < r.max.x - 0.5 {
                    self.line(pos2(x, r.min.y), pos2(x, r.max.y), self.t.border, 1.0);
                }
            }
            y += rh;
            if y < r.max.y - 0.5 {
                self.line(pos2(r.min.x, y), pos2(r.max.x, y), self.t.border, 1.0);
            }
        }
        self.p.rect_stroke(r, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
    }
    /// A table whose rows and columns are all the same size; `hi` outlines one cell.
    pub(super) fn ms_table_eq(&self, r: Rect, rows: usize, cols: usize, hi: Option<(usize, usize)>) {
        let rw = vec![1.0; rows.max(1)];
        let cw = vec![1.0; cols.max(1)];
        self.ms_table_w(r, &rw, &cw);
        if let Some((ri, ci)) = hi {
            self.p.rect_stroke(self.cell(r, rows, cols, ri, ci), 0.0, Stroke::new(1.5, self.t.accent), egui::StrokeKind::Inside);
        }
    }
    /// A SmartArt process: `n` boxes in a row, read right to left, with arrows between them.
    pub(super) fn ms_smartart(&self, r: Rect, n: usize, o: Sa) {
        let n = n.max(1);
        let gap = 7.0;
        let bw = (r.width() - gap * (n as f32 - 1.0)) / n as f32;
        let bh = r.height().min(bw * 0.9);
        let colours = [self.t.accent, Color32::from_rgb(230, 120, 60), Color32::from_rgb(60, 170, 120), Color32::from_rgb(180, 80, 150)];
        for i in 0..n {
            // The first step is on the right.
            let x = r.max.x - (i as f32 + 1.0) * bw - i as f32 * gap;
            let b = Rect::from_min_size(pos2(x, r.center().y - bh / 2.0), vec2(bw, bh));
            let base = if o.colourful { colours.get(i % 4).copied().unwrap_or(self.t.accent) } else { self.t.accent };
            let fill = if o.hot == Some(i) || o.colourful { base } else { base.linear_multiply(0.4) };
            if o.styled {
                self.p.rect_filled(b.translate(vec2(2.0, 2.0)), 3.0, Color32::from_black_alpha(60));
            }
            self.p.rect_filled(b, 3.0, fill);
            if o.styled {
                self.p.rect_filled(Rect::from_min_size(b.min, vec2(b.width(), b.height() * 0.45)), 3.0, Color32::from_white_alpha(70));
            }
            self.p.rect_stroke(b, 3.0, Stroke::new(1.0, base), egui::StrokeKind::Inside);
            if o.pictures {
                let pr = Rect::from_min_max(b.min + vec2(3.0, 3.0), pos2(b.max.x - 3.0, b.min.y + b.height() * 0.55));
                if i == 0 { self.ms_picture(pr, Pic::Normal) } else { self.dashed_rect(pr, Color32::WHITE) }
            }
            let ty = if o.pictures { b.max.y - 4.0 } else { b.center().y };
            let tc = if o.hot == Some(i) || o.colourful || o.pictures { Color32::WHITE } else { self.t.text };
            self.line(pos2(b.min.x + 4.0, ty), pos2(b.max.x - 4.0, ty), tc, 1.5);
            if i + 1 < n {
                self.arrow_left(pos2(x - 1.0, b.center().y), pos2(x - gap + 1.0, b.center().y));
            }
        }
    }
    /// A SmartArt hierarchy in `r`: a top box, then the children listed in `levels` (each
    /// 1 or 2 deep); the box at `hot` in the full accent.
    pub(super) fn ms_tree(&self, r: Rect, levels: &[u8], hot: Option<usize>) {
        let bw = 22.0;
        let bh = 9.0;
        let top = Rect::from_center_size(pos2(r.center().x, r.min.y + bh / 2.0), vec2(bw, bh));
        self.p.rect_filled(top, 2.0, self.t.accent.linear_multiply(0.4));
        self.p.rect_stroke(top, 2.0, self.stroke(self.t.accent), egui::StrokeKind::Inside);
        let n1 = levels.iter().filter(|l| **l == 1).count().max(1);
        let span = (r.width() - bw).max(bw);
        let mut col = 0usize;
        let mut parent_x = r.center().x;
        let y1 = r.min.y + bh + 12.0;
        for (i, l) in levels.iter().enumerate() {
            let hot_box = hot == Some(i);
            let (cx, cy) = if *l <= 1 {
                let cx = r.max.x - bw / 2.0 - span * col as f32 / (n1.max(2) - 1) as f32;
                col += 1;
                parent_x = cx;
                (cx, y1)
            } else {
                (parent_x, y1 + bh + 10.0)
            };
            let b = Rect::from_center_size(pos2(cx, cy), vec2(bw, bh));
            // Connector: from the top box or the parent down to this box.
            let from = if *l <= 1 { top.center_bottom() } else { pos2(parent_x, y1 + bh / 2.0) };
            let c = if hot_box { self.t.accent } else { self.t.border_strong };
            self.line(from, pos2(from.x, (from.y + b.min.y) / 2.0), c, 1.0);
            self.line(pos2(from.x, (from.y + b.min.y) / 2.0), pos2(cx, (from.y + b.min.y) / 2.0), c, 1.0);
            self.line(pos2(cx, (from.y + b.min.y) / 2.0), b.center_top(), c, 1.0);
            self.p.rect_filled(b, 2.0, if hot_box { self.t.accent } else { self.t.accent.linear_multiply(0.4) });
            self.p.rect_stroke(b, 2.0, self.stroke(self.t.accent), egui::StrokeKind::Inside);
        }
    }
    /// A text box with ragged lines aligned as `align` says.
    pub(super) fn ms_text_box_aligned(&self, r: Rect, align: TextAlign, c: Color32) {
        self.p.rect_stroke(r, 0.0, self.stroke(self.t.border), egui::StrokeKind::Inside);
        let widths = [1.0, 0.72, 0.88, 0.5];
        let inner = r.shrink(4.0);
        for (i, w) in widths.iter().enumerate() {
            let y = inner.min.y + 3.0 + i as f32 * 8.0;
            if y > inner.max.y {
                break;
            }
            let lw = if align == TextAlign::Justify { inner.width() } else { inner.width() * w };
            let x0 = match align {
                TextAlign::Left | TextAlign::Justify => inner.min.x,
                TextAlign::Center => inner.center().x - lw / 2.0,
                TextAlign::Right => inner.max.x - lw,
            };
            self.line(pos2(x0, y), pos2(x0 + lw, y), c, 1.5);
        }
    }

    // ---- Shape Format ----

    pub(super) fn ms_edit_shape(&self) {
        let k = self.reveal(0.2, 0.7);
        let a = self.ms_slide_big();
        self.ms_title_body(a, 0);
        let c = pos2(a.center().x, a.center().y + 6.0);
        // A rectangle whose bottom-right point is being pulled out.
        let pull = vec2(16.0, 10.0) * k;
        let pts = vec![c + vec2(-26.0, -14.0), c + vec2(26.0, -14.0), c + vec2(26.0, 14.0) + pull, c + vec2(-26.0, 14.0)];
        self.p.add(Shape::convex_polygon(pts.clone(), self.t.accent.linear_multiply(0.35), Stroke::NONE));
        self.p.add(Shape::closed_line(pts.clone(), Stroke::new(1.5, self.t.accent)));
        for p in &pts {
            self.p.rect_filled(Rect::from_center_size(*p, vec2(4.0, 4.0)), 0.0, self.ink());
        }
        if let Some(moved) = pts.get(2) {
            self.pointer(*moved + vec2(2.0, 2.0));
        }
    }
    pub(super) fn ms_merge_shapes(&self) {
        self.ms_before_after(
            |d, a| {
                let c = a.center();
                d.p.circle_filled(c + vec2(-9.0, 0.0), 11.0, d.t.accent.linear_multiply(0.35));
                d.p.circle_stroke(c + vec2(-9.0, 0.0), 11.0, Stroke::new(1.5, d.t.accent));
                let r = Rect::from_center_size(c + vec2(9.0, 2.0), vec2(22.0, 16.0));
                d.p.rect_filled(r, 2.0, Color32::from_rgb(230, 120, 60).linear_multiply(0.4));
                d.p.rect_stroke(r, 2.0, Stroke::new(1.5, Color32::from_rgb(230, 120, 60)), egui::StrokeKind::Inside);
            },
            |d, a| {
                let c = a.center();
                // The union: the circle and the rectangle in one fill, one outline.
                d.p.circle_filled(c + vec2(-9.0, 0.0), 11.0, d.t.accent.linear_multiply(0.35));
                let r = Rect::from_center_size(c + vec2(9.0, 2.0), vec2(22.0, 16.0));
                d.p.rect_filled(r, 2.0, d.t.accent.linear_multiply(0.35));
                d.p.circle_filled(c + vec2(-9.0, 0.0), 11.0, d.t.accent.linear_multiply(0.35));
                d.p.circle_stroke(c + vec2(-9.0, 0.0), 11.0, Stroke::new(1.5, d.t.accent));
                d.p.rect_stroke(r, 2.0, Stroke::new(1.5, d.t.accent), egui::StrokeKind::Inside);
                // Hide the inner lines where they overlap.
                let overlap = Rect::from_min_max(pos2(r.min.x + 1.5, r.min.y + 1.5), pos2(c.x - 9.0 + 10.0, r.max.y - 1.5));
                d.p.rect_filled(overlap, 0.0, d.t.accent.linear_multiply(0.35));
            },
        );
    }
    pub(super) fn ms_shape_styles(&self) {
        let k = self.reveal(0.3, 0.7);
        let a = self.ms_slide(4.0, 24.0, 88.0);
        self.ms_title_body(a, 0);
        let r = Rect::from_center_size(a.center() + vec2(0.0, 5.0), vec2(40.0, 20.0));
        if k >= 1.0 {
            self.p.rect_filled(r, 3.0, self.t.accent);
            self.p.rect_stroke(r, 3.0, Stroke::new(2.0, Color32::WHITE), egui::StrokeKind::Inside);
        } else {
            self.p.rect_filled(r, 3.0, self.t.hover);
            self.p.rect_stroke(r, 3.0, self.stroke(self.t.text_dim), egui::StrokeKind::Inside);
        }
        // The gallery: six tiles, the chosen one framed.
        let looks: [(Color32, Color32, f32); 6] = [
            (Color32::WHITE, self.t.text_dim, 1.0),
            (self.t.accent.linear_multiply(0.3), self.t.accent, 1.0),
            (self.t.accent, Color32::WHITE, 2.0),
            (Color32::from_rgb(230, 120, 60).linear_multiply(0.3), Color32::from_rgb(230, 120, 60), 1.0),
            (Color32::from_rgb(230, 120, 60), Color32::WHITE, 2.0),
            (self.t.text, Color32::WHITE, 1.0),
        ];
        for (i, (fill, line, w)) in looks.iter().enumerate() {
            let x = 100.0 + (i % 3) as f32 * 24.0;
            let y = 30.0 + (i / 3) as f32 * 22.0;
            let tile = Rect::from_min_size(self.at(x, y), vec2(20.0, 16.0));
            self.p.rect_filled(tile.shrink(2.0), 2.0, *fill);
            self.p.rect_stroke(tile.shrink(2.0), 2.0, Stroke::new(*w, *line), egui::StrokeKind::Inside);
            if i == 2 && k > 0.0 {
                self.p.rect_stroke(tile, 2.0, Stroke::new(1.5, self.t.accent), egui::StrokeKind::Outside);
            }
        }
    }
    pub(super) fn ms_text_fill(&self) {
        self.ms_before_after(|d, a| d.glyph(a.center(), "عنوان", 16.0, d.ink()), |d, a| d.glyph(a.center(), "عنوان", 16.0, d.accent()));
    }
    pub(super) fn ms_text_outline(&self) {
        self.ms_before_after(
            |d, a| d.glyph(a.center(), "عنوان", 16.0, d.ink()),
            |d, a| {
                for off in [
                    vec2(-1.0, 0.0),
                    vec2(1.0, 0.0),
                    vec2(0.0, -1.0),
                    vec2(0.0, 1.0),
                    vec2(-1.0, -1.0),
                    vec2(1.0, 1.0),
                    vec2(-1.0, 1.0),
                    vec2(1.0, -1.0),
                ] {
                    d.glyph(a.center() + off, "عنوان", 16.0, d.accent());
                }
                d.glyph(a.center(), "عنوان", 16.0, Color32::WHITE);
            },
        );
    }
    pub(super) fn ms_text_effects(&self) {
        self.ms_before_after(
            |d, a| d.glyph(a.center(), "عنوان", 16.0, d.ink()),
            |d, a| {
                d.glyph(a.center() + vec2(2.0, 2.5), "عنوان", 16.0, Color32::from_black_alpha(70));
                d.glyph(a.center(), "عنوان", 16.0, d.accent());
            },
        );
    }
    /// `n` overlapping shapes; the accent one moves from z-order `from` to `to`.
    pub(super) fn ms_stack(&self, n: usize, from: usize, to: usize) {
        let draw = |d: &Draw<'_>, a: Rect, hot: usize| {
            let c = a.center();
            for z in 0..n {
                let i = n - 1 - z;
                let off = (i as f32 - (n as f32 - 1.0) / 2.0) * 9.0;
                // z is the drawing order: 0 at the bottom. The hot shape sits at z == hot.
                let (fill, line) = if z == hot { (d.t.accent, d.t.accent) } else { (Color32::from_rgb(200, 200, 205), d.t.text_dim) };
                let r = Rect::from_center_size(c + vec2(off, off * 0.6), vec2(28.0, 18.0));
                d.p.rect_filled(r, 2.0, fill);
                d.p.rect_stroke(r, 2.0, Stroke::new(1.0, line), egui::StrokeKind::Inside);
            }
        };
        self.ms_before_after(|d, a| draw(d, a, from), |d, a| draw(d, a, to));
    }
    pub(super) fn ms_align(&self) {
        self.ms_before_after(
            |d, a| {
                for (i, dy) in [4.0, 12.0, 8.0].iter().enumerate() {
                    let r = Rect::from_min_size(pos2(a.min.x + 6.0 + i as f32 * 20.0, a.min.y + dy), vec2(14.0, 12.0));
                    d.shape(r);
                }
            },
            |d, a| {
                let y = a.min.y + 4.0;
                d.line(pos2(a.min.x, y), pos2(a.max.x, y), d.t.accent, 1.0);
                for i in 0..3 {
                    let r = Rect::from_min_size(pos2(a.min.x + 6.0 + i as f32 * 20.0, y), vec2(14.0, 12.0));
                    d.shape(r);
                }
            },
        );
    }
    pub(super) fn ms_group(&self, g: Grouping) {
        let apart = |d: &Draw<'_>, a: Rect, ghost: bool| {
            let rects = [
                Rect::from_min_size(a.min + vec2(6.0, 4.0), vec2(20.0, 12.0)),
                Rect::from_min_size(a.min + vec2(32.0, 8.0), vec2(16.0, 16.0)),
                Rect::from_min_size(a.min + vec2(10.0, 20.0), vec2(16.0, 9.0)),
            ];
            if ghost {
                d.dashed_rect(Rect::from_min_max(a.min + vec2(4.0, 2.0), a.min + vec2(50.0, 31.0)), d.t.border_strong);
            }
            for r in rects {
                d.shape(r);
                d.handles(r);
            }
        };
        let together = |d: &Draw<'_>, a: Rect| {
            for r in [
                Rect::from_min_size(a.min + vec2(6.0, 4.0), vec2(20.0, 12.0)),
                Rect::from_min_size(a.min + vec2(32.0, 8.0), vec2(16.0, 16.0)),
                Rect::from_min_size(a.min + vec2(10.0, 20.0), vec2(16.0, 9.0)),
            ] {
                d.shape(r);
            }
            let all = Rect::from_min_max(a.min + vec2(4.0, 2.0), a.min + vec2(50.0, 31.0));
            d.p.rect_stroke(all, 0.0, Stroke::new(1.0, d.t.accent), egui::StrokeKind::Outside);
            d.handles(all);
        };
        match g {
            Grouping::Group => self.ms_before_after(|d, a| apart(d, a, false), together),
            Grouping::Ungroup => self.ms_before_after(together, |d, a| apart(d, a, false)),
            Grouping::Regroup => self.ms_before_after(|d, a| apart(d, a, true), together),
        }
    }
    /// A wide arrow turning a quarter turn; `dir` 1 clockwise, -1 anticlockwise. The Rotate
    /// button (`handle`) also shows the round rotation handle above the shape.
    pub(super) fn ms_rotate(&self, handle: bool, dir: f32) {
        let k = self.reveal(0.2, 0.7);
        let a = self.ms_slide_big();
        self.ms_title_body(a, 0);
        let c = a.center() + vec2(0.0, 6.0);
        let ang = std::f32::consts::FRAC_PI_2 * k * dir;
        self.ms_fat_arrow(c, vec2(ang.cos(), ang.sin()), 44.0, self.t.accent.linear_multiply(0.4));
        self.ms_curve_arrow(c + vec2(0.0, -2.0), 30.0, dir < 0.0, 0.6);
        if handle {
            let h = c + vec2(0.0, -34.0);
            self.p.circle_filled(h, 3.0, Color32::WHITE);
            self.p.circle_stroke(h, 3.0, Stroke::new(1.2, self.t.accent));
            self.line(h + vec2(0.0, 3.0), c + vec2(0.0, -24.0), self.t.accent, 1.0);
        }
    }
    pub(super) fn ms_flip(&self, horizontal: bool) {
        let a = self.ms_slide_big();
        self.ms_title_body(a, 0);
        let c = a.center() + vec2(0.0, 6.0);
        let k = self.reveal(0.2, 0.7);
        if horizontal {
            self.ms_fat_arrow(c + vec2(-26.0, 0.0), vec2(1.0, 0.0), 36.0, self.t.accent.linear_multiply(0.4));
            self.dashed_rect(Rect::from_min_max(c + vec2(-0.5, -22.0), c + vec2(0.5, 22.0)), self.t.accent);
            if k > 0.0 {
                self.ms_fat_arrow(c + vec2(26.0, 0.0), vec2(-1.0, 0.0), 36.0 * k, self.t.accent.linear_multiply(0.4));
            }
        } else {
            self.ms_fat_arrow(c + vec2(0.0, -16.0), vec2(0.0, -1.0), 24.0, self.t.accent.linear_multiply(0.4));
            self.dashed_rect(Rect::from_min_max(c + vec2(-40.0, -0.5), c + vec2(40.0, 0.5)), self.t.accent);
            if k > 0.0 {
                self.ms_fat_arrow(c + vec2(0.0, 16.0), vec2(0.0, 1.0), 24.0 * k, self.t.accent.linear_multiply(0.4));
            }
        }
    }

    // ---- Picture Format ----

    /// The picture area on a before/after slide.
    fn ms_pic_rect(a: Rect) -> Rect {
        Rect::from_center_size(a.center() + vec2(0.0, 2.0), vec2(44.0, 26.0))
    }
    pub(super) fn ms_picture_change(&self, from: Pic, to: Pic) {
        self.ms_before_after(
            |d, a| {
                d.ms_title_body(a, 0);
                d.ms_picture(Self::ms_pic_rect(a), from);
            },
            |d, a| {
                d.ms_title_body(a, 0);
                d.ms_picture(Self::ms_pic_rect(a), to);
            },
        );
    }
    pub(super) fn ms_remove_background(&self) {
        self.ms_picture_change(Pic::Normal, Pic::NoSky);
    }
    pub(super) fn ms_transparency(&self) {
        self.ms_before_after(
            |d, a| {
                d.ms_title_body(a, 0);
                d.ms_picture(Self::ms_pic_rect(a), Pic::Normal);
            },
            |d, a| {
                d.ms_title_body(a, 3);
                d.ms_picture(Self::ms_pic_rect(a), Pic::Faded);
            },
        );
    }
    pub(super) fn ms_compress(&self) {
        self.ms_before_after(
            |d, a| {
                d.ms_picture(Self::ms_pic_rect(a), Pic::Normal);
                d.badge(pos2(a.max.x - 14.0, a.max.y - 4.0), "4 MB", d.red());
            },
            |d, a| {
                d.ms_picture(Self::ms_pic_rect(a), Pic::Normal);
                d.badge(pos2(a.max.x - 14.0, a.max.y - 4.0), "400 KB", d.green());
            },
        );
    }
    pub(super) fn ms_picture_border(&self) {
        self.ms_before_after(
            |d, a| {
                d.ms_title_body(a, 0);
                d.ms_picture(Self::ms_pic_rect(a), Pic::Normal);
            },
            |d, a| {
                d.ms_title_body(a, 0);
                let r = Self::ms_pic_rect(a);
                d.ms_picture(r, Pic::Normal);
                d.p.rect_stroke(r, 0.0, Stroke::new(2.5, d.ink()), egui::StrokeKind::Outside);
            },
        );
    }
    pub(super) fn ms_crop(&self) {
        let k = self.reveal(0.2, 0.7);
        let a = self.ms_slide_big();
        self.ms_title_body(a, 0);
        let full = Rect::from_center_size(a.center() + vec2(0.0, 6.0), vec2(72.0, 42.0));
        self.ms_picture(full, Pic::Normal);
        // The crop frame closes in from the top and the right; what is outside goes dim.
        let keep = Rect::from_min_max(pos2(full.min.x, full.min.y + 12.0 * k), pos2(full.max.x - 18.0 * k, full.max.y));
        let dim = Color32::from_black_alpha(90);
        self.p.rect_filled(Rect::from_min_max(full.min, pos2(full.max.x, keep.min.y)), 0.0, dim);
        self.p.rect_filled(Rect::from_min_max(pos2(keep.max.x, keep.min.y), full.max), 0.0, dim);
        let c = self.ink();
        for (corner, dx, dy) in
            [(keep.left_top(), 1.0, 1.0), (keep.right_top(), -1.0, 1.0), (keep.left_bottom(), 1.0, -1.0), (keep.right_bottom(), -1.0, -1.0)]
        {
            self.line(corner, corner + vec2(7.0 * dx, 0.0), c, 2.5);
            self.line(corner, corner + vec2(0.0, 7.0 * dy), c, 2.5);
        }
    }
    pub(super) fn ms_reset_picture(&self) {
        self.ms_before_after(
            |d, a| {
                d.ms_title_body(a, 0);
                let r = Self::ms_pic_rect(a);
                d.ms_picture(Rect::from_min_max(r.min + vec2(8.0, 4.0), r.max), Pic::Grey);
            },
            |d, a| {
                d.ms_title_body(a, 0);
                d.ms_picture(Self::ms_pic_rect(a), Pic::Normal);
            },
        );
    }

    // ---- tables ----

    fn ms_tbl_rect(a: Rect) -> Rect {
        Rect::from_min_max(pos2(a.min.x + 4.0, a.min.y + 8.0), pos2(a.max.x - 4.0, a.max.y - 2.0))
    }
    pub(super) fn ms_shading(&self) {
        self.ms_before_after(
            |d, a| {
                d.ms_title_body(a, 0);
                d.ms_table_eq(Self::ms_tbl_rect(a), 3, 3, Some((2, 0)));
            },
            |d, a| {
                d.ms_title_body(a, 0);
                let r = Self::ms_tbl_rect(a);
                d.ms_table_eq(r, 3, 3, None);
                let cell = d.cell(r, 3, 3, 2, 0);
                d.p.rect_filled(cell, 0.0, d.t.accent.linear_multiply(0.45));
                d.p.rect_stroke(cell, 0.0, Stroke::new(1.5, d.t.accent), egui::StrokeKind::Inside);
            },
        );
    }
    pub(super) fn ms_borders(&self) {
        let plain = |d: &Draw<'_>, r: Rect, lines: bool| {
            d.p.rect_filled(r, 0.0, Color32::WHITE);
            for i in 0..3 {
                for j in 0..3 {
                    let c = d.cell(r, 3, 3, i, j);
                    d.line(pos2(c.max.x - 3.0, c.center().y), pos2(c.max.x - 3.0 - c.width() * 0.5, c.center().y), d.t.text_dim, 1.5);
                }
            }
            if lines {
                d.grid(r, 3, 3, d.t.accent);
            }
        };
        self.ms_before_after(
            |d, a| {
                d.ms_title_body(a, 0);
                plain(d, Self::ms_tbl_rect(a), false);
            },
            |d, a| {
                d.ms_title_body(a, 0);
                plain(d, Self::ms_tbl_rect(a), true);
            },
        );
    }
    pub(super) fn ms_table_delete(&self) {
        self.ms_before_after(
            |d, a| {
                d.ms_title_body(a, 0);
                let r = Self::ms_tbl_rect(a);
                d.ms_table_eq(r, 3, 4, None);
                let col = Rect::from_min_max(d.cell(r, 3, 4, 0, 1).min, d.cell(r, 3, 4, 2, 1).max);
                d.p.rect_filled(col, 0.0, d.red().linear_multiply(0.3));
                d.cross(col.center());
            },
            |d, a| {
                d.ms_title_body(a, 0);
                let r = Self::ms_tbl_rect(a);
                d.ms_table_eq(Rect::from_min_max(r.min, pos2(r.min.x + r.width() * 0.75, r.max.y)), 3, 3, None);
            },
        );
    }
    pub(super) fn ms_table_insert(&self, side: Side) {
        self.ms_before_after(
            |d, a| {
                d.ms_title_body(a, 0);
                d.ms_table_eq(Self::ms_tbl_rect(a), 3, 3, Some((1, 1)));
            },
            |d, a| {
                d.ms_title_body(a, 0);
                let r = Self::ms_tbl_rect(a);
                let (rows, cols) = match side {
                    Side::Above | Side::Below => (4, 3),
                    Side::Left | Side::Right => (3, 4),
                };
                d.ms_table_eq(r, rows, cols, None);
                let (from, to) = match side {
                    Side::Above => (d.cell(r, rows, cols, 1, 0), d.cell(r, rows, cols, 1, 2)),
                    Side::Below => (d.cell(r, rows, cols, 2, 0), d.cell(r, rows, cols, 2, 2)),
                    Side::Left => (d.cell(r, rows, cols, 0, 1), d.cell(r, rows, cols, 2, 1)),
                    Side::Right => (d.cell(r, rows, cols, 0, 2), d.cell(r, rows, cols, 2, 2)),
                };
                let new = Rect::from_min_max(from.min, to.max);
                d.p.rect_filled(new, 0.0, d.t.accent.linear_multiply(0.4));
                d.p.rect_stroke(new, 0.0, Stroke::new(1.5, d.t.accent), egui::StrokeKind::Inside);
            },
        );
    }
    pub(super) fn ms_merge_cells(&self) {
        self.ms_before_after(
            |d, a| {
                d.ms_title_body(a, 0);
                let r = Self::ms_tbl_rect(a);
                d.ms_table_eq(r, 3, 3, None);
                d.selection(Rect::from_min_max(d.cell(r, 3, 3, 0, 0).min, d.cell(r, 3, 3, 0, 2).max).shrink(1.0));
            },
            |d, a| {
                d.ms_title_body(a, 0);
                let r = Self::ms_tbl_rect(a);
                d.ms_table_eq(r, 3, 3, None);
                let top = Rect::from_min_max(d.cell(r, 3, 3, 0, 0).min, d.cell(r, 3, 3, 0, 2).max);
                d.p.rect_filled(top.shrink(1.0), 0.0, d.t.hover);
                d.p.rect_stroke(top, 0.0, Stroke::new(1.5, d.t.accent), egui::StrokeKind::Inside);
                d.line(pos2(top.center().x - 12.0, top.center().y), pos2(top.center().x + 12.0, top.center().y), d.t.text, 1.5);
            },
        );
    }
    pub(super) fn ms_split_cells(&self) {
        self.ms_before_after(
            |d, a| {
                d.ms_title_body(a, 0);
                let r = Self::ms_tbl_rect(a);
                d.ms_table_eq(r, 3, 2, Some((1, 1)));
            },
            |d, a| {
                d.ms_title_body(a, 0);
                let r = Self::ms_tbl_rect(a);
                d.ms_table_eq(r, 3, 2, None);
                let c = d.cell(r, 3, 2, 1, 1);
                d.p.rect_filled(c, 0.0, Color32::WHITE);
                d.line(pos2(c.center().x, c.min.y), pos2(c.center().x, c.max.y), d.t.accent, 1.5);
                d.p.rect_stroke(c, 0.0, Stroke::new(1.5, d.t.accent), egui::StrokeKind::Inside);
                for x in [c.min.x + 3.0, c.center().x + 3.0] {
                    d.line(pos2(x, c.center().y), pos2(x + c.width() * 0.3, c.center().y), d.t.text_dim, 1.5);
                }
            },
        );
    }
    pub(super) fn ms_distribute(&self, rows: bool) {
        self.ms_before_after(
            |d, a| {
                d.ms_title_body(a, 0);
                let r = Self::ms_tbl_rect(a);
                if rows {
                    d.ms_table_w(r, &[1.0, 2.2, 0.8], &[1.0, 1.0, 1.0]);
                } else {
                    d.ms_table_w(r, &[1.0, 1.0, 1.0], &[0.7, 2.2, 1.0]);
                }
            },
            |d, a| {
                d.ms_title_body(a, 0);
                let r = Self::ms_tbl_rect(a);
                d.ms_table_eq(r, 3, 3, None);
                if rows {
                    d.measure_v(pos2(r.max.x + 3.0, r.min.y), pos2(r.max.x + 3.0, r.max.y));
                } else {
                    d.measure_h(pos2(r.min.x, r.max.y + 3.0), pos2(r.max.x, r.max.y + 3.0));
                }
            },
        );
    }

    // ---- charts ----

    /// A line chart in `r` (the same five values as [`Draw::chart`]).
    pub(super) fn ms_line_chart(&self, r: Rect) {
        self.p.rect_filled(r, 1.0, Color32::WHITE);
        self.p.rect_stroke(r, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        let vals = [0.5, 0.8, 0.35, 0.95, 0.6];
        let pts: Vec<Pos2> = vals
            .iter()
            .enumerate()
            .map(|(i, h)| pos2(r.min.x + 6.0 + (r.width() - 12.0) * i as f32 / 4.0, r.max.y - 4.0 - (r.height() - 8.0) * h))
            .collect();
        self.p.add(Shape::line(pts.clone(), Stroke::new(1.8, self.t.accent)));
        for p in pts {
            self.p.circle_filled(p, 2.0, self.t.accent);
        }
    }
    pub(super) fn ms_chart_type(&self) {
        self.ms_before_after(
            |d, a| {
                d.ms_title_body(a, 0);
                d.chart(Self::ms_pic_rect(a));
            },
            |d, a| {
                d.ms_title_body(a, 0);
                d.ms_line_chart(Self::ms_pic_rect(a));
            },
        );
    }
    pub(super) fn ms_edit_data(&self) {
        let k = self.reveal(0.3, 0.7);
        let a = self.ms_slide(4.0, 20.0, 80.0);
        self.ms_title_body(a, 0);
        let cr = Rect::from_min_max(pos2(a.min.x + 4.0, a.min.y + 8.0), a.max - vec2(4.0, 2.0));
        // The bar being edited grows with the number.
        self.p.rect_filled(cr, 1.0, Color32::WHITE);
        self.p.rect_stroke(cr, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        let bw = (cr.width() - 8.0) / 5.0;
        for (i, h) in [0.5, 0.8, 0.35, 0.95, 0.6].iter().enumerate() {
            let h = if i == 2 { 0.35 + 0.45 * k } else { *h };
            let x = cr.min.x + 4.0 + i as f32 * bw + 2.0;
            let top = cr.max.y - 4.0 - (cr.height() - 8.0) * h;
            let c = if i == 2 { self.t.accent } else { self.t.accent.linear_multiply(0.5) };
            self.p.rect_filled(Rect::from_min_max(pos2(x, top), pos2(x + bw - 4.0, cr.max.y - 4.0)), 1.0, c);
        }
        // The data sheet beside it.
        let sheet = Rect::from_min_size(self.at(94.0, 18.0), vec2(76.0, 68.0));
        self.p.rect_filled(sheet, 2.0, self.t.menu);
        self.p.rect_stroke(sheet, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        let g = Rect::from_min_max(sheet.min + vec2(5.0, 5.0), sheet.max - vec2(5.0, 5.0));
        self.grid(g, 5, 2, self.t.border);
        let nums = ["١٢٠", "١٩٠", "١٥٠", "٢٣٠", "١٤٠"];
        for (i, n) in nums.iter().enumerate() {
            let c = self.cell(g, 5, 2, i, 0);
            let (txt, col) = if i == 2 { (if k >= 1.0 { "١٥٠" } else { "٨٠" }, self.t.accent) } else { (*n, self.t.text) };
            self.label(c.center(), txt, 7.0, col, Align2::CENTER_CENTER);
            let l = self.cell(g, 5, 2, i, 1);
            self.line(pos2(l.min.x + 4.0, l.center().y), pos2(l.max.x - 4.0, l.center().y), self.t.text_dim, 1.5);
            if i == 2 {
                self.p.rect_stroke(c, 0.0, Stroke::new(1.5, self.t.accent), egui::StrokeKind::Inside);
            }
        }
    }

    // ---- SmartArt ----

    fn ms_sa_rect(a: Rect) -> Rect {
        Rect::from_min_max(pos2(a.min.x + 2.0, a.min.y + 9.0), pos2(a.max.x - 2.0, a.max.y - 2.0))
    }
    pub(super) fn ms_smartart_pair(&self, from: Sa, to: Sa) {
        self.ms_before_after(
            |d, a| {
                d.ms_title_body(a, 0);
                d.ms_smartart(Self::ms_sa_rect(a), 3, from);
            },
            |d, a| {
                d.ms_title_body(a, 0);
                d.ms_smartart(Self::ms_sa_rect(a), 3, to);
            },
        );
    }
    pub(super) fn ms_text_pane(&self) {
        let pane = self.pane(true, 60.0, 0);
        let a = self.ms_slide(6.0, 24.0, 100.0);
        self.ms_title_body(a, 0);
        self.ms_smartart(Self::ms_sa_rect(a), 3, Sa::plain());
        // The pane: one bullet line per box, the third being typed.
        let k = self.reveal(0.3, 0.8);
        let area = Rect::from_min_max(pos2(pane.min.x + 6.0, pane.min.y + 18.0), pos2(pane.max.x - 6.0, pane.max.y - 4.0));
        for i in 0..3 {
            let y = area.min.y + 4.0 + i as f32 * 12.0;
            self.p.circle_filled(pos2(area.max.x - 2.0, y), 1.5, self.t.text);
            let w = if i == 2 { (area.width() - 8.0) * 0.6 * k } else { (area.width() - 8.0) * 0.8 };
            self.line(pos2(area.max.x - 7.0, y), pos2(area.max.x - 7.0 - w, y), if i == 2 { self.t.accent } else { self.grey() }, 1.5);
            if i == 2 && k < 1.0 {
                self.caret(pos2(area.max.x - 7.0 - w, y - 4.0), 8.0);
            }
        }
    }
    pub(super) fn ms_add_shape(&self) {
        self.ms_before_after(
            |d, a| {
                d.ms_title_body(a, 0);
                d.ms_smartart(Self::ms_sa_rect(a), 3, Sa::plain());
            },
            |d, a| {
                d.ms_title_body(a, 0);
                d.ms_smartart(Self::ms_sa_rect(a), 4, Sa { hot: Some(3), ..Sa::plain() });
            },
        );
    }
    pub(super) fn ms_level(&self, promote: bool) {
        let deep = |d: &Draw<'_>, a: Rect| d.ms_tree(Self::ms_sa_rect(a), &[1, 1, 2], Some(2));
        let flat = |d: &Draw<'_>, a: Rect| d.ms_tree(Self::ms_sa_rect(a), &[1, 1, 1], Some(0));
        if promote {
            self.ms_before_after(deep, flat);
        } else {
            self.ms_before_after(flat, deep);
        }
    }
    pub(super) fn ms_smartart_picture(&self) {
        self.ms_before_after(
            |d, a| {
                d.ms_title_body(a, 0);
                // Every frame still empty: the first box's picture frame is dashed too.
                let r = Self::ms_sa_rect(a);
                d.ms_smartart(r, 3, Sa { pictures: true, ..Sa::plain() });
                let bw = (r.width() - 14.0) / 3.0;
                let b = Rect::from_min_size(pos2(r.max.x - bw, r.center().y - r.height().min(bw * 0.9) / 2.0), vec2(bw, r.height().min(bw * 0.9)));
                let pr = Rect::from_min_max(b.min + vec2(3.0, 3.0), pos2(b.max.x - 3.0, b.min.y + b.height() * 0.55));
                d.p.rect_filled(pr, 0.0, d.t.accent.linear_multiply(0.4));
                d.dashed_rect(pr, Color32::WHITE);
            },
            |d, a| {
                d.ms_title_body(a, 0);
                d.ms_smartart(Self::ms_sa_rect(a), 3, Sa { pictures: true, ..Sa::plain() });
            },
        );
    }
    pub(super) fn ms_layouts(&self) {
        self.ms_before_after(
            |d, a| {
                d.ms_title_body(a, 0);
                d.ms_smartart(Self::ms_sa_rect(a), 3, Sa::plain());
            },
            |d, a| {
                d.ms_title_body(a, 0);
                // A cycle: the same three boxes around a ring.
                let c = Self::ms_sa_rect(a).center();
                d.p.circle_stroke(c, 11.0, Stroke::new(1.0, d.t.accent));
                for i in 0..3 {
                    let ang = -std::f32::consts::FRAC_PI_2 + i as f32 * std::f32::consts::TAU / 3.0;
                    let b = Rect::from_center_size(c + vec2(ang.cos(), ang.sin()) * 12.0, vec2(16.0, 8.0));
                    d.p.rect_filled(b, 2.0, d.t.accent.linear_multiply(0.4));
                    d.p.rect_stroke(b, 2.0, d.stroke(d.t.accent), egui::StrokeKind::Inside);
                }
            },
        );
    }
    pub(super) fn ms_convert(&self) {
        self.ms_before_after(
            |d, a| {
                d.ms_title_body(a, 0);
                d.ms_smartart(Self::ms_sa_rect(a), 3, Sa::plain());
            },
            |d, a| {
                d.ms_title_body(a, 0);
                d.list(Rect::from_min_max(pos2(a.min.x + 4.0, a.min.y + 6.0), a.max), Dir::Rtl, &["•", "•", "•"], d.t.accent);
            },
        );
    }

    // ---- Playback and panes ----

    pub(super) fn ms_volume(&self) {
        let k = self.reveal(0.2, 0.8);
        self.speaker(self.at(50.0, 52.0));
        let track = Rect::from_min_size(self.at(76.0, 49.0), vec2(90.0, 6.0));
        self.p.rect_filled(track, 3.0, self.t.border);
        let level = 0.25 + 0.6 * k;
        self.p.rect_filled(Rect::from_min_size(track.min, vec2(track.width() * level, 6.0)), 3.0, self.t.accent);
        let knob = pos2(track.min.x + track.width() * level, track.center().y);
        self.p.circle_filled(knob, 5.0, Color32::WHITE);
        self.p.circle_stroke(knob, 5.0, Stroke::new(1.5, self.t.accent));
        let pct = ms_num((level * 100.0) as usize);
        self.label(pos2(track.center().x, track.max.y + 12.0), &format!("{pct}٪"), 8.0, self.grey(), Align2::CENTER_CENTER);
    }
    pub(super) fn ms_trim(&self) {
        let k = self.reveal(0.2, 0.8);
        let frame = Rect::from_min_size(self.at(48.0, 8.0), vec2(80.0, 45.0));
        self.ms_picture(frame, Pic::Normal);
        self.ms_play(frame.center(), 7.0, Color32::from_white_alpha(220));
        let track = Rect::from_min_size(self.at(16.0, 66.0), vec2(144.0, 10.0));
        self.p.rect_filled(track, 2.0, self.t.border);
        let start = track.min.x + 30.0 * k;
        let end = track.max.x - 24.0 * k;
        self.p.rect_filled(Rect::from_min_max(pos2(start, track.min.y), pos2(end, track.max.y)), 2.0, self.t.accent.linear_multiply(0.6));
        for (x, c) in [(start, self.green()), (end, self.red())] {
            self.p.rect_filled(Rect::from_center_size(pos2(x, track.center().y), vec2(5.0, 18.0)), 2.0, c);
        }
        self.label(pos2(start, 90.0 + self.r.min.y), "٠:٠٥", 7.0, self.grey(), Align2::CENTER_CENTER);
        self.label(pos2(end, 90.0 + self.r.min.y), "٠:٤٠", 7.0, self.grey(), Align2::CENTER_CENTER);
    }
    pub(super) fn ms_alt_text(&self) {
        let a = self.ms_slide_big();
        self.ms_picture(Rect::from_min_size(a.min + vec2(4.0, 10.0), vec2(40.0, 26.0)), Pic::Normal);
        let inner = self.dialog("Alt Text", 0);
        self.paragraph(Rect::from_min_max(inner.min + vec2(4.0, 2.0), inner.max), Dir::Rtl, 3, self.grey());
        let k = self.reveal(0.3, 0.8);
        if k < 1.0 {
            self.caret(pos2(inner.max.x - inner.width() * 0.55 * k, inner.min.y + 2.0), 8.0);
        }
    }
    pub(super) fn ms_format_pane(&self) {
        let pane = self.pane(true, 64.0, 0);
        let a = self.ms_slide(6.0, 26.0, 96.0);
        self.ms_title_body(a, 0);
        self.shape(Rect::from_center_size(a.center() + vec2(0.0, 5.0), vec2(36.0, 20.0)));
        let area = Rect::from_min_max(pos2(pane.min.x + 6.0, pane.min.y + 18.0), pos2(pane.max.x - 6.0, pane.max.y - 4.0));
        let swatches = [self.t.accent, Color32::from_rgb(230, 120, 60), Color32::from_rgb(60, 170, 120)];
        for (i, c) in swatches.iter().enumerate() {
            let y = area.min.y + i as f32 * 14.0;
            self.line(pos2(area.max.x, y + 4.0), pos2(area.max.x - 22.0, y + 4.0), self.grey(), 1.5);
            self.p.rect_filled(Rect::from_min_size(pos2(area.min.x, y), vec2(14.0, 9.0)), 1.0, *c);
        }
        let y = area.min.y + 44.0;
        self.line(pos2(area.max.x, y + 4.0), pos2(area.max.x - 22.0, y + 4.0), self.grey(), 1.5);
        let f = Rect::from_min_size(pos2(area.min.x, y), vec2(22.0, 9.0));
        self.p.rect_filled(f, 1.0, self.t.input);
        self.p.rect_stroke(f, 1.0, self.stroke(self.t.input_border), egui::StrokeKind::Inside);
    }

    // ---- File ----

    pub(super) fn ms_new_presentation(&self) {
        let k = self.reveal(0.1, 0.6);
        let outer = self.ms_slide_big_rect();
        let a = self.ms_slide_rect(outer);
        if k > 0.0 {
            let clip = Rect::from_min_max(outer.min, pos2(outer.max.x, outer.min.y + outer.height() * k));
            let clipped = self.p.with_clip_rect(clip.intersect(self.p.clip_rect()));
            let d = self.with_painter(&clipped);
            d.dashed_rect(Rect::from_min_max(pos2(a.min.x + 10.0, a.min.y + 12.0), pos2(a.max.x - 10.0, a.min.y + 30.0)), d.t.accent);
            d.dashed_rect(Rect::from_min_max(pos2(a.min.x + 20.0, a.min.y + 36.0), pos2(a.max.x - 20.0, a.min.y + 50.0)), d.t.border_strong);
        }
        self.badge(self.at(150.0, 16.0), "+", self.green());
    }
    pub(super) fn ms_open(&self) {
        let k = self.reveal(0.1, 0.7);
        self.ms_folder(self.at(40.0, 60.0));
        let x = 70.0 + 24.0 * k;
        let a = self.ms_slide(x, 32.0 - 8.0 * k, 80.0);
        self.ms_title_body(a, 3);
        self.arrow_right(self.at(58.0, 48.0), self.at(66.0, 48.0));
    }
    pub(super) fn ms_close(&self) {
        let k = self.reveal(0.2, 0.7);
        let w = 136.0 - 70.0 * k;
        let h = w * SLIDE_RATIO;
        let outer = Rect::from_center_size(self.at(88.0, 52.0), vec2(w, h));
        let a = self.ms_slide_rect(outer);
        self.ms_title_body(a, 3);
        self.cross(outer.right_top());
    }
    pub(super) fn ms_save(&self) {
        let k = self.reveal(0.1, 0.7);
        let a = self.ms_slide(12.0, 12.0, 72.0);
        self.ms_title_body(a, 3);
        let drive = Rect::from_min_size(self.at(104.0, 36.0), vec2(56.0, 36.0));
        self.p.rect_filled(drive, 3.0, self.t.hover);
        self.p.rect_stroke(drive, 3.0, self.stroke(self.t.text), egui::StrokeKind::Inside);
        self.p.rect_filled(Rect::from_min_size(drive.min + vec2(6.0, 24.0), vec2(44.0, 6.0)), 1.0, self.t.border_strong);
        self.p.circle_filled(drive.min + vec2(46.0, 8.0), 2.0, if k >= 1.0 { self.green() } else { self.t.border_strong });
        self.arrow_right(self.at(86.0, 52.0), self.at(86.0 + 14.0 * k, 52.0));
        if k >= 1.0 {
            self.check(drive.center() - vec2(0.0, 2.0));
        }
    }
    pub(super) fn ms_save_as(&self) {
        let k = self.reveal(0.2, 0.7);
        let a = self.ms_slide(8.0, 20.0, 60.0);
        self.ms_title_body(a, 3);
        self.label(self.at(38.0, 62.0), "تقرير", 7.0, self.grey(), Align2::CENTER_CENTER);
        self.arrow_right(self.at(72.0, 36.0), self.at(72.0 + 12.0 * k, 36.0));
        let b = self.ms_slide(92.0, 20.0, 60.0);
        self.ms_title_body(b, 3);
        self.p.rect_stroke(
            Rect::from_min_size(self.at(92.0, 20.0), vec2(60.0, 60.0 * SLIDE_RATIO)),
            2.0,
            Stroke::new(1.5, self.t.accent),
            egui::StrokeKind::Outside,
        );
        self.label(self.at(122.0, 62.0), "تقرير ٢", 7.0, self.accent(), Align2::CENTER_CENTER);
        let f = Rect::from_min_size(self.at(30.0, 76.0), vec2(116.0, 14.0));
        self.p.rect_filled(f, 2.0, self.t.input);
        self.p.rect_stroke(f, 2.0, self.stroke(self.t.input_border), egui::StrokeKind::Inside);
        self.label(pos2(f.max.x - 4.0, f.center().y), "تقرير ٢.pptx", 7.5, self.t.text, Align2::RIGHT_CENTER);
    }
    pub(super) fn ms_export(&self) {
        let k = self.reveal(0.1, 0.7);
        let a = self.ms_slide(8.0, 24.0, 64.0);
        self.ms_title_body(a, 3);
        self.arrow_right(self.at(76.0, 42.0), self.at(76.0 + 12.0 * k, 42.0));
        for (i, (txt, c)) in [("PDF", self.red()), ("MP4", self.accent()), ("PNG", self.green())].iter().enumerate() {
            let y = 24.0 + i as f32 * 22.0;
            if self.reveal(0.3 + i as f32 * 0.15, 0.5 + i as f32 * 0.15) > 0.0 {
                self.badge(self.at(122.0, y), txt, *c);
            }
        }
    }
    pub(super) fn ms_print(&self) {
        let k = self.reveal(0.2, 0.8);
        self.printer(self.at(88.0, 42.0));
        let page = Rect::from_min_size(self.at(72.0, 54.0), vec2(32.0, 20.0 * k + 2.0));
        let clip = self.p.with_clip_rect(page.intersect(self.p.clip_rect()));
        let d = self.with_painter(&clip);
        let a = d.ms_slide(72.0, 54.0, 32.0);
        d.ms_title_body(a, 2);
        self.line(self.at(72.0, 54.0), self.at(104.0, 54.0), self.t.text, 1.0);
    }
    pub(super) fn ms_properties(&self) {
        let inner = self.dialog("Properties", 0);
        let rows = [("العنوان", "تقرير"), ("الشرائح", "٢٤"), ("الحجم", "٣ MB")];
        for (i, (k, v)) in rows.iter().enumerate() {
            let y = inner.min.y + 4.0 + i as f32 * 13.0;
            self.label(pos2(inner.max.x, y), k, 7.0, self.t.text, Align2::RIGHT_CENTER);
            self.label(pos2(inner.min.x, y), v, 7.0, self.accent(), Align2::LEFT_CENTER);
        }
        self.badge(self.at(22.0, 20.0), "i", self.accent());
    }

    // ---- Edit ----

    pub(super) fn ms_undo(&self, redo: bool) {
        let k = self.reveal(0.2, 0.7);
        let a = self.ms_slide_big();
        self.ms_title_body(a, 2);
        let r = Rect::from_min_size(pos2(a.min.x + 6.0, a.min.y + 26.0), vec2(40.0, 26.0));
        // Undo brings the deleted picture back; Redo takes it away again.
        let alpha = if redo { 1.0 - k } else { k };
        if alpha > 0.02 {
            self.ms_picture(r, if alpha >= 1.0 { Pic::Normal } else { Pic::Faded });
        } else {
            self.dashed_rect(r, self.t.border_strong);
        }
        self.ms_curve_arrow(pos2(a.max.x - 24.0, a.min.y + 38.0), 12.0, !redo, 1.0);
    }
    pub(super) fn ms_clip(&self, op: Clip) {
        let k = self.reveal(0.2, 0.7);
        let a = self.ms_slide(4.0, 22.0, 100.0);
        self.ms_title_body(a, 0);
        let slot = Rect::from_min_size(pos2(a.min.x + 8.0, a.min.y + 14.0), vec2(34.0, 22.0));
        let board = self.ms_clipboard(self.at(146.0, 52.0));
        let held = Rect::from_center_size(board.center(), vec2(16.0, 10.0));
        match op {
            Clip::Cut => {
                if k < 1.0 {
                    self.shape(slot);
                    self.handles(slot);
                } else {
                    self.dashed_rect(slot, self.t.border_strong);
                }
                self.arrow_right(self.at(110.0, 52.0), self.at(110.0 + 16.0 * k, 52.0));
                if k >= 1.0 {
                    self.shape(held);
                }
            }
            Clip::Copy => {
                self.shape(slot);
                self.handles(slot);
                self.arrow_right(self.at(110.0, 52.0), self.at(110.0 + 16.0 * k, 52.0));
                if k >= 1.0 {
                    self.shape(held);
                }
            }
            Clip::Paste => {
                self.shape(held);
                self.arrow_left(self.at(128.0, 52.0), self.at(128.0 - 16.0 * k, 52.0));
                if k >= 1.0 {
                    self.shape(slot);
                    self.handles(slot);
                } else {
                    self.dashed_rect(slot, self.t.border_strong);
                }
            }
            Clip::PasteText => {
                // Red bold lines on the clipboard become the slide's plain grey lines.
                for i in 0..3 {
                    let y = board.min.y + 5.0 + i as f32 * 6.0;
                    self.line(pos2(board.min.x + 2.0, y), pos2(board.max.x - 2.0, y), self.red(), 2.5);
                }
                self.arrow_left(self.at(128.0, 52.0), self.at(128.0 - 16.0 * k, 52.0));
                let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 10.0), a.max);
                self.paragraph(body, Dir::Rtl, 2, self.grey());
                if k >= 1.0 {
                    self.text_lines(
                        Rect::from_min_max(pos2(body.min.x, body.min.y + 20.0), body.max),
                        true,
                        &[body.width() * 0.8, body.width() * 0.5],
                        10.0,
                        self.grey(),
                    );
                    self.p.rect_filled(
                        Rect::from_min_max(pos2(body.max.x - body.width() * 0.8, body.min.y + 19.0), pos2(body.max.x, body.min.y + 21.0)),
                        0.0,
                        self.t.selection,
                    );
                }
            }
        }
    }
    pub(super) fn ms_paste_special(&self) {
        let inner = self.dialog("Paste Special", 0);
        let k = self.reveal(0.3, 0.7);
        let items = ["صورة", "نص فقط", "كائن"];
        for (i, s) in items.iter().enumerate() {
            let row = Rect::from_min_size(pos2(inner.min.x, inner.min.y + i as f32 * 13.0), vec2(inner.width(), 12.0));
            if i == 0 && k > 0.0 {
                self.p.rect_filled(row, 2.0, self.t.checked);
            }
            self.label(pos2(row.max.x - 4.0, row.center().y), s, 7.5, self.t.text, Align2::RIGHT_CENTER);
            self.p.circle_stroke(pos2(row.min.x + 6.0, row.center().y), 3.0, Stroke::new(1.0, self.t.text_dim));
            if i == 0 && k > 0.0 {
                self.p.circle_filled(pos2(row.min.x + 6.0, row.center().y), 1.8, self.t.accent);
            }
        }
    }
    pub(super) fn ms_select_all(&self) {
        let k = self.reveal(0.1, 0.7);
        let a = self.ms_slide_big();
        self.ms_title_body(a, 2);
        let objs = [
            Rect::from_min_size(pos2(a.min.x + 6.0, a.min.y + 26.0), vec2(30.0, 20.0)),
            Rect::from_min_size(pos2(a.min.x + 44.0, a.min.y + 30.0), vec2(22.0, 22.0)),
            Rect::from_min_size(pos2(a.max.x - 44.0, a.min.y + 24.0), vec2(38.0, 26.0)),
        ];
        if let Some(first) = objs.first() {
            self.shape(*first);
        }
        if let Some(second) = objs.get(1) {
            self.p.circle_filled(second.center(), 10.0, Color32::from_rgb(230, 120, 60).linear_multiply(0.5));
        }
        if let Some(third) = objs.get(2) {
            self.ms_picture(*third, Pic::Normal);
        }
        let marquee = Rect::from_min_max(a.min, a.min + (a.max - a.min) * k);
        self.p.rect_filled(marquee, 0.0, self.t.selection.linear_multiply(0.5));
        self.dashed_rect(marquee, self.t.accent);
        if k >= 1.0 {
            for r in objs {
                self.handles(r);
            }
        }
    }
    pub(super) fn ms_duplicate(&self) {
        let k = self.reveal(0.2, 0.7);
        let a = self.ms_slide_big();
        self.ms_title_body(a, 0);
        let r = Rect::from_min_size(pos2(a.min.x + 14.0, a.min.y + 18.0), vec2(40.0, 26.0));
        self.shape(r);
        let copy = r.translate(vec2(40.0, 10.0) * k);
        if k > 0.0 {
            self.shape(copy);
            self.handles(copy);
        }
    }
    pub(super) fn ms_delete_slide(&self) {
        let k = self.reveal(0.25, 0.7);
        if k >= 1.0 {
            self.ms_strip(3, 44.0, 34.0, None);
            self.badge(self.at(88.0, 12.0), "٤ ← ٣", self.accent());
        } else {
            let rects = self.ms_strip(4, 36.0, 34.0, Some(2));
            if let Some(r) = rects.get(2) {
                self.p.rect_filled(*r, 2.0, self.red().linear_multiply(0.25));
                self.cross(r.center());
            }
        }
    }
    pub(super) fn ms_find(&self) {
        let a = self.ms_slide_big();
        self.ms_title_body(a, 0);
        let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 12.0), a.max);
        let k = self.reveal(0.2, 0.6);
        if k > 0.0 {
            self.highlight(Rect::from_min_max(pos2(body.max.x - 44.0, body.min.y + 9.0), pos2(body.max.x - 20.0, body.min.y + 19.0)));
        }
        self.paragraph(body, Dir::Rtl, 4, self.grey());
        self.magnifier(pos2(body.max.x - 48.0, body.min.y + 30.0));
        let f = Rect::from_min_size(self.at(60.0, 2.0), vec2(96.0, 11.0));
        self.p.rect_filled(f, 2.0, self.t.input);
        self.p.rect_stroke(f, 2.0, self.stroke(self.t.input_border), egui::StrokeKind::Inside);
        self.label(pos2(f.max.x - 4.0, f.center().y), "البصرة", 7.0, self.t.text, Align2::RIGHT_CENTER);
    }
    pub(super) fn ms_replace(&self) {
        self.ms_before_after(
            |d, a| {
                d.ms_title_body(a, 0);
                let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 8.0), a.max);
                d.highlight(Rect::from_min_max(pos2(body.max.x - 30.0, body.min.y + 10.0), pos2(body.max.x - 4.0, body.min.y + 20.0)));
                d.paragraph(body, Dir::Rtl, 2, d.grey());
                d.label(pos2(body.max.x - 17.0, body.min.y + 15.0), "٢٠٢٥", 7.0, d.ink(), Align2::CENTER_CENTER);
            },
            |d, a| {
                d.ms_title_body(a, 0);
                let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 8.0), a.max);
                d.paragraph(body, Dir::Rtl, 2, d.grey());
                d.label(pos2(body.max.x - 17.0, body.min.y + 15.0), "٢٠٢٦", 7.0, d.accent(), Align2::CENTER_CENTER);
            },
        );
    }

    // ---- View ----

    pub(super) fn ms_view_normal(&self) {
        let w = self.ms_window(Rect::from_min_size(self.at(4.0, 4.0), vec2(168.0, 96.0)));
        // Slide list on the right (RTL), the slide in the middle, notes below.
        let list = Rect::from_min_max(pos2(w.max.x - 30.0, w.min.y), w.max);
        self.p.rect_filled(list, 0.0, self.t.panel);
        for i in 0..3 {
            let a = self.ms_slide_rect(Rect::from_min_size(pos2(list.min.x + 4.0, list.min.y + 3.0 + i as f32 * 20.0), vec2(22.0, 12.4)));
            self.ms_title_body(a, 0);
        }
        let main = Rect::from_min_max(w.min, pos2(list.min.x - 4.0, w.max.y - 16.0));
        let sw = main.width().min(main.height() / SLIDE_RATIO) - 4.0;
        let a = self.ms_slide_rect(Rect::from_center_size(main.center(), vec2(sw, sw * SLIDE_RATIO)));
        self.ms_title_body(a, 3);
        let notes = Rect::from_min_max(pos2(main.min.x, main.max.y + 3.0), pos2(main.max.x, w.max.y));
        self.p.rect_filled(notes, 1.0, self.t.panel);
        self.text_lines(notes.shrink(2.0), true, &[notes.width() * 0.6], 6.0, self.grey());
    }
    pub(super) fn ms_view_sorter(&self) {
        let k = self.reveal(0.2, 0.7);
        for i in 0..6 {
            let col = i % 3;
            let row = i / 3;
            let mut x = 14.0 + (2 - col) as f32 * 52.0;
            let y = 10.0 + row as f32 * 44.0;
            // Slide 5 is being dragged towards slide 4's place.
            if i == 4 {
                x += 26.0 * k;
            }
            let a = self.ms_slide(x, y, 44.0);
            self.ms_title_body(a, 2);
            self.label(self.at(x + 22.0, y + 31.0), &ms_num(i + 1), 7.0, self.grey(), Align2::CENTER_CENTER);
            if i == 4 {
                self.p.rect_stroke(
                    Rect::from_min_size(self.at(x, y), vec2(44.0, 44.0 * SLIDE_RATIO)),
                    2.0,
                    Stroke::new(1.5, self.t.accent),
                    egui::StrokeKind::Outside,
                );
            }
        }
    }
    pub(super) fn ms_view_notes_page(&self) {
        let a = self.page(58.0, 4.0, 60.0, 96.0);
        let s = self.ms_slide_rect(Rect::from_min_size(pos2(a.min.x, a.min.y), vec2(a.width(), a.width() * SLIDE_RATIO)));
        self.ms_title_body(s, 2);
        let k = self.reveal(0.2, 0.8);
        let notes = Rect::from_min_max(pos2(a.min.x, a.min.y + a.width() * SLIDE_RATIO + 6.0), a.max);
        let n = (1.0 + 4.0 * k) as usize;
        self.paragraph(notes, Dir::Rtl, n, self.accent());
    }
    pub(super) fn ms_view_outline(&self) {
        let w = self.ms_window(Rect::from_min_size(self.at(4.0, 4.0), vec2(168.0, 96.0)));
        let pane = Rect::from_min_max(pos2(w.max.x - 74.0, w.min.y), w.max);
        self.p.rect_filled(pane, 0.0, self.t.panel);
        let area = pane.shrink(4.0);
        for (i, (num, level)) in [(1, 0), (0, 1), (0, 1), (2, 0), (0, 1), (3, 0)].iter().enumerate() {
            let y = area.min.y + 5.0 + i as f32 * 11.0;
            if *level == 0 {
                self.label(pos2(area.max.x, y), &ms_num(*num), 7.0, self.accent(), Align2::RIGHT_CENTER);
                self.line(pos2(area.max.x - 10.0, y), pos2(area.max.x - 10.0 - area.width() * 0.6, y), self.ink(), 2.5);
            } else {
                self.line(pos2(area.max.x - 16.0, y), pos2(area.max.x - 16.0 - area.width() * 0.5, y), self.grey(), 1.5);
            }
        }
        let main = Rect::from_min_max(w.min, pos2(pane.min.x - 4.0, w.max.y));
        let sw = main.width().min(main.height() / SLIDE_RATIO) - 4.0;
        let a = self.ms_slide_rect(Rect::from_center_size(main.center(), vec2(sw, sw * SLIDE_RATIO)));
        self.ms_title_body(a, 2);
    }
    pub(super) fn ms_view_reading(&self) {
        let w = self.ms_window(Rect::from_min_size(self.at(4.0, 4.0), vec2(168.0, 96.0)));
        let bar = Rect::from_min_max(pos2(w.min.x, w.max.y - 10.0), w.max);
        let main = Rect::from_min_max(w.min, pos2(w.max.x, bar.min.y - 2.0));
        let sw = main.width().min(main.height() / SLIDE_RATIO) - 2.0;
        let a = self.ms_slide_rect(Rect::from_center_size(main.center(), vec2(sw, sw * SLIDE_RATIO)));
        self.ms_title_body(a, 3);
        self.p.rect_filled(bar, 1.0, self.t.panel);
        self.arrow_left(pos2(bar.center().x - 14.0, bar.center().y), pos2(bar.center().x - 24.0, bar.center().y));
        self.arrow_right(pos2(bar.center().x + 14.0, bar.center().y), pos2(bar.center().x + 24.0, bar.center().y));
        self.label(bar.center(), "٣ / ١٢", 7.0, self.t.text, Align2::CENTER_CENTER);
    }
    /// The show starting from slide `from` of three: the strip, then the screen filling up.
    pub(super) fn ms_show(&self, from: usize) {
        let k = self.reveal(0.2, 0.7);
        let rects = self.ms_strip(3, 36.0, 6.0, Some(from));
        let screen = Rect::from_min_size(self.at(16.0, 44.0), vec2(144.0, 56.0));
        self.p.rect_filled(screen, 2.0, Color32::BLACK);
        let sw = 36.0 + (screen.height() / SLIDE_RATIO - 36.0).min(screen.width() - 36.0) * k;
        let a = self.ms_slide_rect(Rect::from_center_size(screen.center(), vec2(sw, sw * SLIDE_RATIO)));
        self.ms_title_body(a, 3);
        self.label(pos2(a.min.x + 2.0, a.max.y - 2.0), &ms_num(from + 1), 7.0, self.grey(), Align2::LEFT_BOTTOM);
        if let Some(r) = rects.get(from) {
            self.arrow_down(pos2(r.center().x, r.max.y + 11.0), pos2(r.center().x, r.max.y + 16.0));
        }
        if k < 1.0 {
            self.ms_play(screen.center(), 8.0, Color32::from_white_alpha(200));
        }
    }
    pub(super) fn ms_view_master(&self, m: Master) {
        match m {
            Master::Slide => {
                let a = self.ms_slide(56.0, 6.0, 112.0);
                self.dashed_rect(Rect::from_min_max(pos2(a.min.x + 6.0, a.min.y + 4.0), pos2(a.max.x - 6.0, a.min.y + 16.0)), self.t.accent);
                self.dashed_rect(Rect::from_min_max(pos2(a.min.x + 6.0, a.min.y + 22.0), pos2(a.max.x - 6.0, a.max.y - 4.0)), self.t.border_strong);
                self.p.circle_filled(pos2(a.max.x - 4.0, a.max.y - 3.0), 2.5, self.t.accent);
                // The layouts under it, linked to the master.
                for i in 0..3 {
                    let x = 56.0 + i as f32 * 38.0;
                    let s = self.ms_slide(x, 76.0, 32.0);
                    self.dashed_rect(Rect::from_min_max(pos2(s.min.x + 2.0, s.min.y + 1.0), pos2(s.max.x - 2.0, s.min.y + 5.0)), self.t.accent);
                    self.line(self.at(x + 16.0, 76.0), self.at(x + 16.0, 72.0), self.t.border_strong, 1.0);
                }
                self.line(self.at(72.0, 72.0), self.at(148.0, 72.0), self.t.border_strong, 1.0);
                self.line(self.at(112.0, 69.0), self.at(112.0, 72.0), self.t.border_strong, 1.0);
                self.badge(self.at(26.0, 20.0), "Master", self.accent());
            }
            Master::Handout => {
                let a = self.page(58.0, 4.0, 60.0, 96.0);
                self.dashed_rect(Rect::from_min_max(a.min, pos2(a.max.x, a.min.y + 5.0)), self.t.border_strong);
                for i in 0..6 {
                    let col = i % 2;
                    let row = i / 2;
                    let s = Rect::from_min_size(pos2(a.min.x + col as f32 * 23.0, a.min.y + 9.0 + row as f32 * 20.0), vec2(21.0, 21.0 * SLIDE_RATIO));
                    self.dashed_rect(s, self.t.accent);
                }
                self.dashed_rect(Rect::from_min_max(pos2(a.min.x, a.max.y - 5.0), a.max), self.t.border_strong);
            }
            Master::Notes => {
                let a = self.page(58.0, 4.0, 60.0, 96.0);
                self.dashed_rect(Rect::from_min_max(a.min, pos2(a.max.x, a.min.y + 5.0)), self.t.border_strong);
                let s = Rect::from_min_size(pos2(a.min.x, a.min.y + 8.0), vec2(a.width(), a.width() * SLIDE_RATIO));
                self.dashed_rect(s, self.t.accent);
                let notes = Rect::from_min_max(pos2(a.min.x, s.max.y + 4.0), pos2(a.max.x, a.max.y - 7.0));
                self.dashed_rect(notes, self.t.accent);
                self.text_lines(notes.shrink(3.0), true, &[notes.width() * 0.7, notes.width() * 0.5, notes.width() * 0.6], 7.0, self.grey());
                self.dashed_rect(Rect::from_min_max(pos2(a.min.x, a.max.y - 5.0), a.max), self.t.border_strong);
            }
        }
    }
    pub(super) fn ms_view_helper(&self, h: Helper) {
        let k = self.reveal(0.2, 0.6);
        let outer = Rect::from_min_size(self.at(26.0, 18.0), vec2(128.0, 72.0));
        let a = self.ms_slide_rect(outer);
        self.ms_title_body(a, 0);
        let r = Rect::from_center_size(a.center() + vec2(0.0, 6.0), vec2(36.0, 20.0));
        self.shape(r);
        if k <= 0.0 {
            return;
        }
        match h {
            Helper::Ruler => {
                let top = Rect::from_min_max(pos2(outer.min.x, outer.min.y - 9.0), pos2(outer.max.x, outer.min.y - 1.0));
                let left = Rect::from_min_max(pos2(outer.min.x - 9.0, outer.min.y), pos2(outer.min.x - 1.0, outer.max.y));
                for rr in [top, left] {
                    self.p.rect_filled(rr, 1.0, self.t.panel);
                    self.p.rect_stroke(rr, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
                }
                let mut x = top.min.x + 4.0;
                let mut i = 0;
                while x < top.max.x {
                    let len = if i % 4 == 0 { 5.0 } else { 2.5 };
                    self.line(pos2(x, top.max.y), pos2(x, top.max.y - len), self.t.text, 1.0);
                    x += 6.0;
                    i += 1;
                }
                let mut y = left.min.y + 4.0;
                i = 0;
                while y < left.max.y {
                    let len = if i % 4 == 0 { 5.0 } else { 2.5 };
                    self.line(pos2(left.max.x, y), pos2(left.max.x - len, y), self.t.text, 1.0);
                    y += 6.0;
                    i += 1;
                }
            }
            Helper::Grid => {
                let mut y = outer.min.y + 8.0;
                while y < outer.max.y {
                    let mut x = outer.min.x + 8.0;
                    while x < outer.max.x {
                        self.p.circle_filled(pos2(x, y), 0.8, self.t.text_dim);
                        x += 8.0;
                    }
                    y += 8.0;
                }
            }
            Helper::Guides => {
                self.dashed_rect(
                    Rect::from_min_max(pos2(outer.center().x - 0.5, outer.min.y), pos2(outer.center().x + 0.5, outer.max.y)),
                    self.t.accent,
                );
                self.dashed_rect(
                    Rect::from_min_max(pos2(outer.min.x, outer.center().y - 0.5), pos2(outer.max.x, outer.center().y + 0.5)),
                    self.t.accent,
                );
            }
        }
    }
    pub(super) fn ms_grid_options(&self) {
        let inner = self.dialog("Grid", 1);
        // A small grid preview and two checkboxes.
        let g = Rect::from_min_size(pos2(inner.min.x, inner.min.y + 18.0), vec2(30.0, 24.0));
        self.grid(g, 3, 3, self.t.text_dim);
        for (i, s) in ["محاذاة إلى الشبكة", "محاذاة إلى الأشكال"].iter().enumerate() {
            let y = inner.min.y + 22.0 + i as f32 * 12.0;
            let b = Rect::from_center_size(pos2(inner.max.x - 5.0, y), vec2(8.0, 8.0));
            self.p.rect_filled(b, 1.0, self.t.input);
            self.p.rect_stroke(b, 1.0, self.stroke(self.t.input_border), egui::StrokeKind::Inside);
            if i == 0 {
                self.line(b.min + vec2(2.0, 4.0), b.min + vec2(3.5, 6.0), self.accent(), 1.5);
                self.line(b.min + vec2(3.5, 6.0), b.min + vec2(6.5, 2.0), self.accent(), 1.5);
            }
            self.label(pos2(b.min.x - 4.0, y), s, 7.0, self.t.text, Align2::RIGHT_CENTER);
        }
    }
    pub(super) fn ms_zoom_dialog(&self) {
        let inner = self.dialog("Zoom", 0);
        let k = self.reveal(0.3, 0.7);
        for (i, s) in ["٤٠٠٪", "٢٠٠٪", "١٠٠٪", "٥٠٪"].iter().enumerate() {
            let y = inner.min.y + 5.0 + i as f32 * 11.0;
            let on = i == 1 && k > 0.0;
            self.p.circle_stroke(pos2(inner.max.x - 5.0, y), 3.0, Stroke::new(1.0, self.t.text_dim));
            if on {
                self.p.circle_filled(pos2(inner.max.x - 5.0, y), 1.8, self.t.accent);
            }
            self.label(pos2(inner.max.x - 12.0, y), s, 7.5, if on { self.accent() } else { self.t.text }, Align2::RIGHT_CENTER);
        }
        self.magnifier(pos2(inner.min.x + 14.0, inner.center().y - 4.0));
    }
    pub(super) fn ms_zoom(&self, in_: bool) {
        let k = self.reveal(0.2, 0.7);
        let w = self.ms_window(Rect::from_min_size(self.at(4.0, 4.0), vec2(168.0, 96.0)));
        let sw = if in_ { 70.0 + 60.0 * k } else { 130.0 - 60.0 * k };
        let outer = Rect::from_center_size(w.center(), vec2(sw, sw * SLIDE_RATIO));
        let clip = self.p.with_clip_rect(w.intersect(self.p.clip_rect()));
        let d = self.with_painter(&clip);
        let a = d.ms_slide_rect(outer);
        d.ms_title_body(a, 3);
        let m = pos2(w.max.x - 22.0, w.max.y - 22.0);
        self.magnifier(m);
        let c = self.t.accent;
        self.line(m + vec2(-4.0, 0.0), m + vec2(4.0, 0.0), c, 2.0);
        if in_ {
            self.line(m + vec2(0.0, -4.0), m + vec2(0.0, 4.0), c, 2.0);
        }
    }
    pub(super) fn ms_fit(&self) {
        let k = self.reveal(0.2, 0.7);
        let w = self.ms_window(Rect::from_min_size(self.at(4.0, 4.0), vec2(168.0, 96.0)));
        let big = 220.0 - (220.0 - (w.height() - 4.0) / SLIDE_RATIO) * k;
        let outer = Rect::from_center_size(w.center(), vec2(big, big * SLIDE_RATIO));
        let clip = self.p.with_clip_rect(w.intersect(self.p.clip_rect()));
        let d = self.with_painter(&clip);
        let a = d.ms_slide_rect(outer);
        d.ms_title_body(a, 3);
        if k >= 1.0 {
            self.measure_h(pos2(w.min.x + 2.0, w.max.y - 4.0), pos2(w.max.x - 2.0, w.max.y - 4.0));
        }
    }
    pub(super) fn ms_new_window(&self) {
        let k = self.reveal(0.2, 0.7);
        let w1 = self.ms_window(Rect::from_min_size(self.at(4.0, 4.0), vec2(110.0, 70.0)));
        let a = self.ms_slide_rect(Rect::from_center_size(w1.center(), vec2(90.0, 90.0 * SLIDE_RATIO)));
        self.ms_title_body(a, 3);
        if k > 0.0 {
            let w2 = self.ms_window(Rect::from_min_size(self.at(62.0 + (1.0 - k) * 10.0, 30.0 + (1.0 - k) * 10.0), vec2(110.0, 70.0)));
            let b = self.ms_slide_rect(Rect::from_center_size(w2.center(), vec2(90.0, 90.0 * SLIDE_RATIO)));
            self.ms_title_body(b, 3);
        }
    }

    // ---- Insert ----

    pub(super) fn ms_new_slide(&self, duplicate: bool) {
        let k = self.reveal(0.25, 0.7);
        if k <= 0.0 {
            self.ms_strip(2, 44.0, 34.0, Some(0));
            return;
        }
        let rects = self.ms_strip(3, 44.0, 34.0, Some(1));
        if let Some(r) = rects.get(1) {
            // The new slide: empty with placeholders, or a copy of the first.
            if duplicate {
                self.p.rect_filled(r.shrink(1.5), 2.0, self.t.accent.linear_multiply(0.12));
            } else {
                let a = r.shrink(3.0);
                self.p.rect_filled(r.shrink(1.5), 2.0, Color32::WHITE);
                self.dashed_rect(Rect::from_min_max(pos2(a.min.x + 2.0, a.min.y + 2.0), pos2(a.max.x - 2.0, a.min.y + 8.0)), self.t.accent);
                self.dashed_rect(Rect::from_min_max(pos2(a.min.x + 4.0, a.min.y + 11.0), pos2(a.max.x - 4.0, a.max.y - 2.0)), self.t.border_strong);
            }
            self.badge(pos2(r.center().x, r.min.y - 10.0), if duplicate { "نسخة" } else { "+" }, self.green());
        }
    }
    pub(super) fn ms_section(&self) {
        let k = self.reveal(0.2, 0.7);
        let list = Rect::from_min_size(self.at(40.0, 4.0), vec2(96.0, 96.0));
        self.p.rect_filled(list, 2.0, self.t.panel);
        let mut y = list.min.y + 12.0;
        for i in 0..3 {
            if i == 2 && k > 0.0 {
                // The section header slides in above slide 3.
                let h = Rect::from_min_size(pos2(list.min.x + 4.0, y), vec2(88.0, 10.0 * k));
                self.p.rect_filled(h, 2.0, self.t.accent.linear_multiply(0.3));
                self.label(pos2(h.max.x - 3.0, h.center().y), "القسم ٢", 7.0, self.accent(), Align2::RIGHT_CENTER);
                self.line(pos2(h.min.x + 3.0, h.center().y - 2.0), pos2(h.min.x + 6.0, h.center().y + 1.0), self.accent(), 1.5);
                self.line(pos2(h.min.x + 6.0, h.center().y + 1.0), pos2(h.min.x + 9.0, h.center().y - 2.0), self.accent(), 1.5);
                y += 12.0 * k;
            }
            let a = self.ms_slide(list.min.x + 34.0 - self.r.min.x, y - self.r.min.y, 36.0);
            self.ms_title_body(a, 1);
            self.label(pos2(list.max.x - 6.0, y + 10.0), &ms_num(i + 1), 7.0, self.grey(), Align2::RIGHT_CENTER);
            y += 36.0 * SLIDE_RATIO + 4.0;
        }
        self.label(pos2(list.max.x - 3.0, list.min.y + 1.0), "القسم ١", 6.5, self.grey(), Align2::RIGHT_TOP);
    }
    pub(super) fn ms_insert_text_box(&self) {
        let k = self.reveal(0.1, 0.5);
        let k2 = self.reveal(0.5, 0.9);
        let a = self.ms_slide_big();
        self.ms_title_body(a, 0);
        let full = Rect::from_min_size(pos2(a.min.x + 14.0, a.min.y + 18.0), vec2(72.0, 30.0));
        let r = Rect::from_min_max(full.min, full.min + (full.max - full.min) * k.max(0.05));
        self.dashed_rect(r, self.t.accent);
        if k >= 1.0 {
            let n = (1.0 + 2.0 * k2) as usize;
            self.paragraph(r.shrink(4.0), Dir::Rtl, n, self.grey());
            if k2 < 1.0 {
                self.caret(pos2(r.max.x - 4.0 - r.width() * 0.5 * k2, r.min.y + 3.0), 8.0);
            }
        } else {
            self.pointer(r.max);
        }
    }
    pub(super) fn ms_word_art(&self) {
        let a = self.ms_slide_big();
        self.ms_title_body(a, 0);
        let c = a.center() + vec2(0.0, 6.0);
        let k = self.reveal(0.2, 0.7);
        self.glyph(c + vec2(2.5, 3.0), "أهلاً", 26.0, Color32::from_black_alpha((80.0 * k) as u8));
        for off in [vec2(-1.2, 0.0), vec2(1.2, 0.0), vec2(0.0, -1.2), vec2(0.0, 1.2)] {
            self.glyph(c + off, "أهلاً", 26.0, self.ink());
        }
        self.glyph(c, "أهلاً", 26.0, self.t.accent.lerp_to_gamma(Color32::WHITE, 0.3 * (1.0 - k)));
    }
    pub(super) fn ms_date_time(&self) {
        let k = self.reveal(0.3, 0.7);
        let a = self.ms_slide_big();
        self.ms_title_body(a, 3);
        let f = Rect::from_min_size(pos2(a.max.x - 44.0, a.max.y - 8.0), vec2(44.0, 9.0));
        if k > 0.0 {
            self.field(f.min, 44.0);
            self.label(f.center() + vec2(0.0, -1.0), "٢٠٢٦/١٠/١٨", 7.0, self.t.text, Align2::CENTER_CENTER);
        } else {
            self.dashed_rect(f, self.t.border_strong);
        }
    }
    pub(super) fn ms_slide_number(&self) {
        let k = self.reveal(0.3, 0.7);
        let a = self.ms_slide_big();
        self.ms_title_body(a, 3);
        let f = Rect::from_min_size(pos2(a.min.x, a.max.y - 8.0), vec2(14.0, 9.0));
        if k > 0.0 {
            self.field(f.min, 14.0);
            self.label(f.center() + vec2(0.0, -1.0), "٣", 7.0, self.t.text, Align2::CENTER_CENTER);
        } else {
            self.dashed_rect(f, self.t.border_strong);
        }
    }
    pub(super) fn ms_insert_table(&self) {
        let k = self.reveal(0.2, 0.7);
        let a = self.ms_slide(4.0, 24.0, 92.0);
        self.ms_title_body(a, 0);
        let r = Rect::from_min_max(pos2(a.min.x + 4.0, a.min.y + 8.0), a.max - vec2(4.0, 2.0));
        if k >= 1.0 {
            self.ms_table_eq(r, 4, 3, None);
        } else {
            self.dashed_rect(r, self.t.border_strong);
        }
        // The picker: a grid of cells, 4 × 3 of them lit as the pointer drags.
        let pick = Rect::from_min_size(self.at(108.0, 22.0), vec2(60.0, 60.0));
        self.p.rect_filled(pick, 2.0, self.t.menu);
        self.p.rect_stroke(pick, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        let g = pick.shrink(4.0);
        let lit_cols = (1.0 + 2.0 * k).round() as usize;
        let lit_rows = (1.0 + 3.0 * k).round() as usize;
        for i in 0..6 {
            for j in 0..6 {
                let c = self.cell(g, 6, 6, i, j).shrink(0.8);
                // Picked from the right edge (RTL).
                let lit = i < lit_rows && j >= 6 - lit_cols;
                self.p.rect_filled(c, 0.0, if lit { self.t.accent.linear_multiply(0.5) } else { Color32::WHITE });
                self.p.rect_stroke(c, 0.0, self.stroke(if lit { self.t.accent } else { self.t.border }), egui::StrokeKind::Inside);
            }
        }
        self.label(pos2(pick.center().x, pick.max.y - 2.0), "٤ × ٣", 6.5, self.grey(), Align2::CENTER_BOTTOM);
    }
    pub(super) fn ms_insert_chart(&self) {
        let k = self.reveal(0.2, 0.8);
        let a = self.ms_slide_big();
        self.ms_title_body(a, 0);
        let r = Rect::from_center_size(a.center() + vec2(0.0, 6.0), vec2(80.0, 44.0));
        self.p.rect_filled(r, 1.0, Color32::WHITE);
        self.p.rect_stroke(r, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        let bw = (r.width() - 8.0) / 5.0;
        for (i, h) in [0.5, 0.8, 0.35, 0.95, 0.6].iter().enumerate() {
            let x = r.min.x + 4.0 + i as f32 * bw + 2.0;
            let top = r.max.y - 4.0 - (r.height() - 8.0) * h * k;
            self.p.rect_filled(
                Rect::from_min_max(pos2(x, top), pos2(x + bw - 4.0, r.max.y - 4.0)),
                1.0,
                if i % 2 == 0 { self.t.accent } else { self.t.accent.linear_multiply(0.5) },
            );
        }
    }
    pub(super) fn ms_insert_picture(&self) {
        let k = self.reveal(0.1, 0.7);
        self.ms_folder(self.at(30.0, 62.0));
        self.ms_picture(Rect::from_center_size(self.at(30.0, 60.0), vec2(26.0, 16.0)), Pic::Normal);
        let a = self.ms_slide(66.0, 20.0, 100.0);
        self.ms_title_body(a, 0);
        let target = Rect::from_center_size(a.center() + vec2(0.0, 6.0), vec2(50.0, 30.0));
        self.arrow_right(self.at(52.0, 52.0), self.at(52.0 + 10.0 * k, 52.0));
        if k >= 1.0 {
            self.ms_picture(target, Pic::Normal);
            self.handles(target);
        } else {
            self.dashed_rect(target, self.t.border_strong);
        }
    }
    pub(super) fn ms_photo_album(&self, edit: bool) {
        let k = self.reveal(0.2, 0.7);
        let looks = [Pic::Normal, Pic::City, Pic::Normal];
        for (i, look) in looks.iter().enumerate() {
            let x = 10.0 + i as f32 * 54.0;
            let a = self.ms_slide(x, 26.0, 48.0);
            let pr = Rect::from_min_max(pos2(a.min.x + 2.0, a.min.y + 2.0), pos2(a.max.x - 2.0, a.max.y - 6.0));
            let look = if edit && k >= 1.0 { looks.get((i + 1) % 3).copied().unwrap_or(Pic::Normal) } else { *look };
            self.ms_picture(pr, look);
            self.line(pos2(a.center().x - 8.0, a.max.y - 2.0), pos2(a.center().x + 8.0, a.max.y - 2.0), self.grey(), 1.5);
            self.label(self.at(x + 24.0, 60.0), &ms_num(i + 1), 7.0, self.grey(), Align2::CENTER_CENTER);
        }
        if edit {
            // Reordering: an arrow swapping the first two pictures.
            self.arrow_left(self.at(64.0 + 6.0 * k, 16.0), self.at(58.0 - 6.0 * k, 16.0));
            self.pencil(self.at(150.0, 86.0), self.at(166.0, 70.0));
        } else {
            // The pictures come in from the folder.
            self.ms_folder(self.at(88.0, 84.0));
            self.arrow_up(self.at(88.0, 70.0 - 6.0 * k), self.at(88.0, 62.0 - 6.0 * k));
        }
    }
    pub(super) fn ms_insert_audio(&self) {
        let k = self.reveal(0.2, 0.8);
        let a = self.ms_slide_big();
        self.ms_title_body(a, 2);
        let c = a.center() + vec2(0.0, 12.0);
        self.speaker(c);
        for i in 0..3 {
            if self.reveal(0.3 + i as f32 * 0.15, 0.45 + i as f32 * 0.15) > 0.0 {
                self.p.rect_filled(Rect::from_center_size(c + vec2(22.0 + i as f32 * 6.0, 0.0), vec2(3.0, 6.0 + i as f32 * 5.0)), 1.0, self.t.accent);
            }
        }
        let bar = Rect::from_min_size(pos2(c.x - 30.0, c.y + 16.0), vec2(60.0, 3.0));
        self.p.rect_filled(bar, 1.0, self.t.border);
        self.p.rect_filled(Rect::from_min_size(bar.min, vec2(60.0 * k, 3.0)), 1.0, self.t.accent);
    }
    pub(super) fn ms_insert_video(&self) {
        let k = self.reveal(0.2, 0.8);
        let a = self.ms_slide_big();
        self.ms_title_body(a, 0);
        let r = Rect::from_center_size(a.center() + vec2(0.0, 6.0), vec2(76.0, 44.0));
        self.ms_picture(r, Pic::City);
        self.p.rect_filled(Rect::from_min_max(pos2(r.min.x, r.max.y - 7.0), r.max), 0.0, Color32::from_black_alpha(120));
        self.p.rect_filled(
            Rect::from_min_max(pos2(r.min.x + 10.0, r.max.y - 4.0), pos2(r.min.x + 10.0 + (r.width() - 20.0) * k, r.max.y - 2.0)),
            1.0,
            Color32::WHITE,
        );
        self.p.circle_filled(r.center(), 9.0, Color32::from_black_alpha(140));
        self.ms_play(r.center() + vec2(1.0, 0.0), 5.0, Color32::WHITE);
    }

    // ---- Format ----

    pub(super) fn ms_font_dialog(&self) {
        let inner = self.dialog("Font", 2);
        let k = self.reveal(0.3, 0.7);
        let preview = Rect::from_min_max(pos2(inner.min.x, inner.min.y + 30.0), inner.max);
        self.p.rect_filled(preview, 1.0, Color32::WHITE);
        self.p.rect_stroke(preview, 1.0, self.stroke(self.t.border), egui::StrokeKind::Inside);
        self.glyph(preview.center(), "أبجد", 10.0 + 3.0 * k, if k > 0.0 { self.accent() } else { self.ink() });
    }
    pub(super) fn ms_paragraph_dialog(&self) {
        let inner = self.dialog("Paragraph", 1);
        let k = self.reveal(0.3, 0.7);
        for i in 0..4 {
            let r = Rect::from_min_size(pos2(inner.min.x + i as f32 * 14.0, inner.min.y + 16.0), vec2(12.0, 10.0));
            self.p.rect_filled(r, 1.0, if i == 3 { self.t.checked } else { self.t.menu });
            self.p.rect_stroke(r, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
            for j in 0..3 {
                let y = r.min.y + 2.5 + j as f32 * 2.5;
                let w = if j == 1 { 4.0 } else { 7.0 };
                let (x0, x1) = match i {
                    0 => (r.min.x + 2.0, r.min.x + 2.0 + w),
                    1 => (r.center().x - w / 2.0, r.center().x + w / 2.0),
                    2 => (r.max.x - 2.0 - w, r.max.x - 2.0),
                    _ => (r.min.x + 2.0, r.max.x - 2.0),
                };
                self.line(pos2(x0, y), pos2(x1, y), self.t.text, 1.0);
            }
        }
        let preview = Rect::from_min_max(pos2(inner.min.x, inner.min.y + 30.0), inner.max);
        self.p.rect_filled(preview, 1.0, Color32::WHITE);
        self.text_lines(preview.shrink(2.0), true, &[preview.width() * 0.9, preview.width() * 0.8], 4.0 + 3.0 * k, self.grey());
    }
    pub(super) fn ms_text_style(&self, s: TextStyle) {
        let words = |d: &Draw<'_>, a: Rect, styled: bool| {
            d.ms_title_body(a, 0);
            let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 10.0), a.max);
            d.paragraph(body, Dir::Rtl, 3, d.grey());
            // The second line's first word (on the right) is the selected one.
            let y = body.min.y + 14.0;
            let w = Rect::from_min_max(pos2(body.max.x - 26.0, y - 3.0), pos2(body.max.x, y + 3.0));
            d.p.rect_filled(w.expand2(vec2(1.0, 1.5)), 0.0, Color32::WHITE);
            if !styled {
                d.selection(w.expand2(vec2(1.0, 1.5)));
                d.line(pos2(w.min.x, y), pos2(w.max.x, y), d.grey(), 1.5);
                return;
            }
            match s {
                TextStyle::Bold => d.line(pos2(w.min.x, y), pos2(w.max.x, y), d.accent(), 3.5),
                TextStyle::Italic => {
                    let pts =
                        vec![pos2(w.min.x + 2.0, y - 2.0), pos2(w.max.x + 2.0, y - 2.0), pos2(w.max.x - 2.0, y + 2.0), pos2(w.min.x - 2.0, y + 2.0)];
                    d.p.add(Shape::convex_polygon(pts, d.accent(), Stroke::NONE));
                }
                TextStyle::Underline => {
                    d.line(pos2(w.min.x, y), pos2(w.max.x, y), d.accent(), 1.5);
                    d.line(pos2(w.min.x, y + 3.5), pos2(w.max.x, y + 3.5), d.accent(), 1.0);
                }
            }
        };
        self.ms_before_after(|d, a| words(d, a, false), |d, a| words(d, a, true));
    }
    pub(super) fn ms_text_align(&self, al: TextAlign) {
        let before = match al {
            TextAlign::Right => TextAlign::Left,
            _ => TextAlign::Right,
        };
        self.ms_before_after(
            |d, a| {
                d.ms_title_body(a, 0);
                d.ms_text_box_aligned(Rect::from_min_max(pos2(a.min.x + 4.0, a.min.y + 8.0), a.max), before, d.grey());
            },
            |d, a| {
                d.ms_title_body(a, 0);
                d.ms_text_box_aligned(Rect::from_min_max(pos2(a.min.x + 4.0, a.min.y + 8.0), a.max), al, d.accent());
            },
        );
    }
    pub(super) fn ms_size_position(&self) {
        let a = self.ms_slide(4.0, 22.0, 80.0);
        self.ms_title_body(a, 0);
        let r = Rect::from_center_size(a.center() + vec2(0.0, 5.0), vec2(36.0, 20.0));
        self.shape(r);
        self.measure_h(pos2(r.min.x, r.max.y + 5.0), pos2(r.max.x, r.max.y + 5.0));
        self.measure_v(pos2(r.max.x + 5.0, r.min.y), pos2(r.max.x + 5.0, r.max.y));
        let dlg = Rect::from_min_size(self.at(92.0, 14.0), vec2(80.0, 76.0));
        self.p.rect_filled(dlg, 3.0, self.t.menu);
        self.p.rect_stroke(dlg, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.p.rect_filled(Rect::from_min_size(dlg.min, vec2(dlg.width(), 12.0)), 3.0, self.t.hover);
        self.label(pos2(dlg.max.x - 5.0, dlg.min.y + 6.0), "الحجم", 7.0, self.t.text, Align2::RIGHT_CENTER);
        let k = self.reveal(0.3, 0.7);
        for (i, (name, v)) in [("العرض", "١٠ سم"), ("الارتفاع", "٥ سم"), ("الدوران", "٠°")].iter().enumerate() {
            let y = dlg.min.y + 18.0 + i as f32 * 16.0;
            self.label(pos2(dlg.max.x - 5.0, y + 5.0), name, 6.5, self.t.text, Align2::RIGHT_CENTER);
            let f = Rect::from_min_size(pos2(dlg.min.x + 5.0, y), vec2(34.0, 10.0));
            self.p.rect_filled(f, 1.0, self.t.input);
            self.p.rect_stroke(f, 1.0, self.stroke(if i == 0 && k > 0.0 { self.t.accent } else { self.t.input_border }), egui::StrokeKind::Inside);
            self.label(f.center(), v, 6.5, self.t.text, Align2::CENTER_CENTER);
        }
    }
    pub(super) fn ms_rtl(&self) {
        self.ms_before_after(
            |d, a| {
                d.ms_title_body(a, 0);
                let r = Rect::from_min_max(pos2(a.min.x + 4.0, a.min.y + 8.0), a.max);
                d.ms_text_box_aligned(r, TextAlign::Left, d.grey());
                d.arrow_right(pos2(r.min.x + 4.0, r.max.y - 5.0), pos2(r.min.x + 18.0, r.max.y - 5.0));
            },
            |d, a| {
                d.ms_title_body(a, 0);
                let r = Rect::from_min_max(pos2(a.min.x + 4.0, a.min.y + 8.0), a.max);
                d.ms_text_box_aligned(r, TextAlign::Right, d.accent());
                d.arrow_left(pos2(r.max.x - 4.0, r.max.y - 5.0), pos2(r.max.x - 18.0, r.max.y - 5.0));
            },
        );
    }

    // ---- Slide Show and Help ----

    pub(super) fn ms_hide_slide(&self) {
        let k = self.reveal(0.3, 0.7);
        let rects = self.ms_strip(3, 44.0, 30.0, Some(1));
        if let Some(r) = rects.get(1) {
            if k > 0.0 {
                self.p.rect_filled(r.shrink(1.5), 2.0, Color32::from_white_alpha(170));
                let n = pos2(r.center().x, r.max.y + 7.0);
                self.line(n + vec2(-5.0, -4.0), n + vec2(5.0, 4.0), self.red(), 1.5);
                self.line(n + vec2(-5.0, 4.0), n + vec2(5.0, -4.0), self.red(), 1.5);
            }
            // The show skips it: 1 → 3.
            if let (Some(a), Some(b)) = (rects.first(), rects.get(2)) {
                let y = a.min.y - 8.0;
                self.arrow_right(pos2(a.center().x, y), pos2(a.center().x + (b.center().x - a.center().x) * k, y));
            }
        }
    }
    pub(super) fn ms_setup_show(&self) {
        let inner = self.dialog("Set Up Show", 0);
        let k = self.reveal(0.3, 0.7);
        for (i, s) in ["ملء الشاشة", "في نافذة", "تكرار حتى الإيقاف"].iter().enumerate() {
            let y = inner.min.y + 5.0 + i as f32 * 12.0;
            let on = (i == 0 || i == 2) && k > 0.0;
            if i < 2 {
                self.p.circle_stroke(pos2(inner.max.x - 5.0, y), 3.0, Stroke::new(1.0, self.t.text_dim));
                if on {
                    self.p.circle_filled(pos2(inner.max.x - 5.0, y), 1.8, self.t.accent);
                }
            } else {
                let b = Rect::from_center_size(pos2(inner.max.x - 5.0, y), vec2(7.0, 7.0));
                self.p.rect_stroke(b, 1.0, self.stroke(self.t.text_dim), egui::StrokeKind::Inside);
                if on {
                    self.line(b.min + vec2(1.5, 3.5), b.min + vec2(3.0, 5.5), self.accent(), 1.5);
                    self.line(b.min + vec2(3.0, 5.5), b.min + vec2(6.0, 1.5), self.accent(), 1.5);
                }
            }
            self.label(pos2(inner.max.x - 12.0, y), s, 7.0, self.t.text, Align2::RIGHT_CENTER);
        }
        let screen = Rect::from_min_size(pos2(inner.min.x + 2.0, inner.min.y + 2.0), vec2(26.0, 16.0));
        self.p.rect_filled(screen, 1.0, Color32::BLACK);
        self.line(pos2(screen.center().x, screen.max.y), pos2(screen.center().x, screen.max.y + 4.0), self.t.text, 1.5);
        self.line(pos2(screen.center().x - 6.0, screen.max.y + 4.0), pos2(screen.center().x + 6.0, screen.max.y + 4.0), self.t.text, 1.5);
    }
    pub(super) fn ms_about(&self) {
        let inner = self.dialog("About", 0);
        let logo = Rect::from_min_size(pos2(inner.max.x - 22.0, inner.min.y), vec2(22.0, 22.0));
        self.p.rect_filled(logo, 4.0, self.t.accent);
        self.ms_play(logo.center() + vec2(1.0, 0.0), 6.0, Color32::WHITE);
        self.label(pos2(logo.min.x - 6.0, inner.min.y + 6.0), "DeckCraft", 8.0, self.t.text, Align2::RIGHT_CENTER);
        self.label(pos2(logo.min.x - 6.0, inner.min.y + 17.0), "الإصدار ١٫٠", 7.0, self.grey(), Align2::RIGHT_CENTER);
        self.text_lines(
            Rect::from_min_max(pos2(inner.min.x, inner.min.y + 26.0), inner.max),
            true,
            &[inner.width() * 0.8, inner.width() * 0.6],
            8.0,
            self.grey(),
        );
    }
    pub(super) fn ms_palette(&self) {
        let k = self.reveal(0.2, 0.7);
        let w = self.ms_window(Rect::from_min_size(self.at(4.0, 4.0), vec2(168.0, 96.0)));
        let a = self.ms_slide_rect(Rect::from_center_size(w.center() + vec2(0.0, 8.0), vec2(120.0, 120.0 * SLIDE_RATIO)));
        self.ms_title_body(a, 3);
        let box_ = Rect::from_min_size(pos2(w.min.x + 24.0, w.min.y + 2.0), vec2(120.0, 54.0));
        self.p.rect_filled(box_, 3.0, self.t.menu);
        self.p.rect_stroke(box_, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        let f = Rect::from_min_size(box_.min + vec2(4.0, 4.0), vec2(box_.width() - 8.0, 11.0));
        self.p.rect_filled(f, 2.0, self.t.input);
        self.p.rect_stroke(f, 2.0, self.stroke(self.t.input_border), egui::StrokeKind::Inside);
        self.magnifier(pos2(f.min.x + 7.0, f.center().y - 2.0));
        let typed = ["", "إخ", "إخفا", "إخفاء"];
        let text = typed.get((k * 3.0).round() as usize).copied().unwrap_or("إخفاء");
        self.label(pos2(f.max.x - 4.0, f.center().y), text, 7.5, self.t.text, Align2::RIGHT_CENTER);
        for (i, s) in ["إخفاء الشريحة", "إخفاء المسطرة", "إخفاء الشبكة"].iter().enumerate() {
            let row = Rect::from_min_size(pos2(f.min.x, f.max.y + 2.0 + i as f32 * 11.0), vec2(f.width(), 10.0));
            if i == 0 && k > 0.0 {
                self.p.rect_filled(row, 2.0, self.t.checked);
            }
            self.label(pos2(row.max.x - 4.0, row.center().y), s, 7.0, self.t.text, Align2::RIGHT_CENTER);
        }
    }
}
