#![allow(missing_docs)]

use uinput_sys::*;

enum_from_primitive! {
	#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
	#[repr(i32)]
	pub enum Button {
		Left            = BTN_LEFT,
		Right           = BTN_RIGHT,
		Middle          = BTN_MIDDLE,
		Side            = BTN_SIDE,
		Extra           = BTN_EXTRA,
		Forward         = BTN_FORWARD,
		Back            = BTN_BACK,
		Task            = BTN_TASK,
	}
}

#[cfg_attr(rustfmt, rustfmt_skip)]
enum_serde!(Button {
	Left,
	Right,
	Middle,
	Side,
	Extra,
	Forward,
	Back,
	Task,
});

enum_from_primitive! {
	#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
	#[repr(i32)]
	pub enum Key {
	    Reserved         = KEY_RESERVED,
		Esc              = KEY_ESC,
		_1               = KEY_1,
		_2               = KEY_2,
		_3               = KEY_3,
		_4               = KEY_4,
		_5               = KEY_5,
		_6               = KEY_6,
		_7               = KEY_7,
		_8               = KEY_8,
		_9               = KEY_9,
		_0               = KEY_10,
		Minus            = KEY_MINUS,
		Equal            = KEY_EQUAL,
		BackSpace        = KEY_BACKSPACE,
		Tab              = KEY_TAB,
		Q                = KEY_Q,
		W                = KEY_W,
		E                = KEY_E,
		R                = KEY_R,
		T                = KEY_T,
		Y                = KEY_Y,
		U                = KEY_U,
		I                = KEY_I,
		O                = KEY_O,
		P                = KEY_P,
		LeftBrace        = KEY_LEFTBRACE,
		RightBrace       = KEY_RIGHTBRACE,
		Enter            = KEY_ENTER,
		LeftControl      = KEY_LEFTCTRL,
		A                = KEY_A,
		S                = KEY_S,
		D                = KEY_D,
		F                = KEY_F,
		G                = KEY_G,
		H                = KEY_H,
		J                = KEY_J,
		K                = KEY_K,
		L                = KEY_L,
		SemiColon        = KEY_SEMICOLON,
		Apostrophe       = KEY_APOSTROPHE,
		Grave            = KEY_GRAVE,
		LeftShift        = KEY_LEFTSHIFT,
		BackSlash        = KEY_BACKSLASH,
		Z                = KEY_Z,
		X                = KEY_X,
		C                = KEY_C,
		V                = KEY_V,
		B                = KEY_B,
		N                = KEY_N,
		M                = KEY_M,
		Comma            = KEY_COMMA,
		Dot              = KEY_DOT,
		Slash            = KEY_SLASH,
		RightShift       = KEY_RIGHTSHIFT,
		LeftAlt          = KEY_LEFTALT,
		Space            = KEY_SPACE,
		CapsLock         = KEY_CAPSLOCK,
		F1               = KEY_F1,
		F2               = KEY_F2,
		F3               = KEY_F3,
		F4               = KEY_F4,
		F5               = KEY_F5,
		F6               = KEY_F6,
		F7               = KEY_F7,
		F8               = KEY_F8,
		F9               = KEY_F9,
		F10              = KEY_F10,
		NumLock          = KEY_NUMLOCK,
		ScrollLock       = KEY_SCROLLLOCK,
		F11              = KEY_F11,
		F12              = KEY_F12,
		RightControl     = KEY_RIGHTCTRL,
		SysRq            = KEY_SYSRQ,
		RightAlt         = KEY_RIGHTALT,
		LineFeed         = KEY_LINEFEED,
		Home             = KEY_HOME,
		Up               = KEY_UP,
		PageUp           = KEY_PAGEUP,
		Left             = KEY_LEFT,
		Right            = KEY_RIGHT,
		End              = KEY_END,
		Down             = KEY_DOWN,
		PageDown         = KEY_PAGEDOWN,
		Insert           = KEY_INSERT,
		Delete           = KEY_DELETE,
		LeftMeta         = KEY_LEFTMETA,
		RightMeta        = KEY_RIGHTMETA,
		ScrollUp         = KEY_SCROLLUP,
		ScrollDown       = KEY_SCROLLDOWN,
		F13              = KEY_F13,
		F14              = KEY_F14,
		F15              = KEY_F15,
		F16              = KEY_F16,
		F17              = KEY_F17,
		F18              = KEY_F18,
		F19              = KEY_F19,
		F20              = KEY_F20,
		F21              = KEY_F21,
		F22              = KEY_F22,
		F23              = KEY_F23,
		F24              = KEY_F24,

		KP7 			 = KEY_KP7,
		KP8 			 = KEY_KP8,
		KP9 			 = KEY_KP9,
		KPMinus 		 = KEY_KPMINUS,
		KP4 			 = KEY_KP4,
		KP5 			 = KEY_KP5,
		KP6 			 = KEY_KP6,
		KPPlus 			 = KEY_KPPLUS,
		KP1 			 = KEY_KP1,
		KP2 			 = KEY_KP2,
		KP3 			 = KEY_KP3,
		KP0 			 = KEY_KP0,
		KPDot 			 = KEY_KPDOT,

		Zenkakuhankaku 	 = KEY_ZENKAKUHANKAKU,
		_102ND 			 = KEY_102ND,
		RO 				 = KEY_RO,
		Katakana 		 = KEY_KATAKANA,
		Hiragana 		 = KEY_HIRAGANA,
		Henkan 			 = KEY_HENKAN,
		KatakanaHiragana = KEY_KATAKANAHIRAGANA,
		Muhenkan 		 = KEY_MUHENKAN,
		KPJPComma 		 = KEY_KPJPCOMMA,
		KPEnter 		 = KEY_KPENTER,

		KPSlash 		 = KEY_KPSLASH,

		Macro 			 = KEY_MACRO,
		Mute 			 = KEY_MUTE,
		VolumeDown 		 = KEY_VOLUMEDOWN,
		VolumeUp 		 = KEY_VOLUMEUP,
		Power 			 = KEY_POWER, /* SC System Power Down */
		KPEqual 		 = KEY_KPEQUAL,
		KPPlusMinus 	 = KEY_KPPLUSMINUS,
		Pause 			 = KEY_PAUSE,
		Scale 			 = KEY_SCALE, /* AL Compiz Scale : c_int = Expose */

		KPComma 		 = KEY_KPCOMMA,
		Hangeul			 = KEY_HANGEUL,
		Hanja 			 = KEY_HANJA,
		Yen 			 = KEY_YEN,

		Compose 		 = KEY_COMPOSE,

		Stop 			 = KEY_STOP, /* AC Stop */
		Again 			 = KEY_AGAIN,
		Props 			 = KEY_PROPS, /* AC Properties */
		Undo 			 = KEY_UNDO, /* AC Undo */
		Front 			 = KEY_FRONT,
		Copy 			 = KEY_COPY, /* AC Copy */
		Open 			 = KEY_OPEN, /* AC Open */
		Paste 			 = KEY_PASTE, /* AC Paste */
		Find 			 = KEY_FIND, /* AC Search */
		Cut 			 = KEY_CUT, /* AC Cut */
		Help 			 = KEY_HELP, /* AL Integrated Help Center */
		Menu 			 = KEY_MENU, /* Menu : c_int = show menu */
		Calc 			 = KEY_CALC, /* AL Calculator */
		Setup 			 = KEY_SETUP,
		Sleep 			 = KEY_SLEEP, /* SC System Sleep */
		Wakeup 			 = KEY_WAKEUP, /* System Wake Up */
		File 			 = KEY_FILE, /* AL Local Machine Browser */
		Sendfile 		 = KEY_SENDFILE,
		DeleteFile 		 = KEY_DELETEFILE,
		Xfer 			 = KEY_XFER,
		Prog1 			 = KEY_PROG1,
		Prog2 			 = KEY_PROG2,
		WWW 			 = KEY_WWW, /* AL Internet Browser */
		MSDos 			 = KEY_MSDOS,
		ScreenLock 		 = KEY_COFFEE, /* AL Terminal Lock/Screensaver */
		RotateDisplay 	 = KEY_ROTATE_DISPLAY, /* Display orientation for e.g. tablets */
		CycleWindows 	 = KEY_CYCLEWINDOWS,
		Mail 			 = KEY_MAIL,
		Bookmarks 		 = KEY_BOOKMARKS, /* AC Bookmarks */
		Computer 		 = KEY_COMPUTER,
		Back 			 = KEY_BACK, /* AC Back */
		Forward 		 = KEY_FORWARD, /* AC Forward */
		CloseCD 		 = KEY_CLOSECD,
		EjectCD 		 = KEY_EJECTCD,
		EjectCloseCD 	 = KEY_EJECTCLOSECD,
		NextSong 		 = KEY_NEXTSONG,
		PlayPause 		 = KEY_PLAYPAUSE,
		PreviousSong 	 = KEY_PREVIOUSSONG,
		StopCD 			 = KEY_STOPCD,
		Record 			 = KEY_RECORD,
		Rewind 			 = KEY_REWIND,
		Phone 			 = KEY_PHONE, /* Media Select Telephone */
		Iso 			 = KEY_ISO,
		Config 			 = KEY_CONFIG, /* AL Consumer Control Configuration */
		HomePage 		 = KEY_HOMEPAGE, /* AC Home */
		Refresh 		 = KEY_REFRESH, /* AC Refresh */
		Exit 			 = KEY_EXIT, /* AC Exit */
		Move 			 = KEY_MOVE,
		Edit 			 = KEY_EDIT,
		KPLeftParen 	 = KEY_KPLEFTPAREN,
		KPRightParen 	 = KEY_KPRIGHTPAREN,
		New 			 = KEY_NEW, /* AC New */
		Redo 			 = KEY_REDO, /* AC Redo/Repeat */

		PlayCD 			 = KEY_PLAYCD,
		PauseCD 		 = KEY_PAUSECD,
		Prog3 			 = KEY_PROG3,
		Prog4 			 = KEY_PROG4,
		Dashboard 		 = KEY_DASHBOARD, /* AL Dashboard */
		Suspend 		 = KEY_SUSPEND,
		Close 			 = KEY_CLOSE, /* AC Close */
		Play 			 = KEY_PLAY,
		FastForward 	 = KEY_FASTFORWARD,
		BassBoost 		 = KEY_BASSBOOST,
		Print 			 = KEY_PRINT, /* AC Print */
		HP 				 = KEY_HP,
		Camera 			 = KEY_CAMERA,
		Sound 			 = KEY_SOUND,
		Question 		 = KEY_QUESTION,
		Email 			 = KEY_EMAIL,
		Chat 			 = KEY_CHAT,
		Search 			 = KEY_SEARCH,
		Connect 		 = KEY_CONNECT,
		Finance 		 = KEY_FINANCE, /* AL Checkbook/Finance */
		Sport 			 = KEY_SPORT,
		Shop 			 = KEY_SHOP,
		AltErase 		 = KEY_ALTERASE,
		Cancel 			 = KEY_CANCEL, /* AC Cancel */
		BrightnessDown 	 = KEY_BRIGHTNESSDOWN,
		BrightnessUp 	 = KEY_BRIGHTNESSUP,
		Media 			 = KEY_MEDIA,

		SwitchVideoMode  = KEY_SWITCHVIDEOMODE,
		/* Cycle between available video outputs (Monitor/LCD/TV-out/etc) */
		KBDIllumToggle 	 = KEY_KBDILLUMTOGGLE,
		KBDIllumDown 	 = KEY_KBDILLUMDOWN,
		KBDIllumUp 		 = KEY_KBDILLUMUP,

		Send 			 = KEY_SEND, /* AC Send */
		Reply 			 = KEY_REPLY, /* AC Reply */
		ForwardMail 	 = KEY_FORWARDMAIL, /* AC Forward Msg */
		Save 			 = KEY_SAVE, /* AC Save */
		Documents 		 = KEY_DOCUMENTS,

		Battery 		 = KEY_BATTERY,

		Bluetooth 		 = KEY_BLUETOOTH,
		Wlan 			 = KEY_WLAN,
		Uwb 			 = KEY_UWB,

		Unknown 		 = KEY_UNKNOWN,

		VideoNext 		 = KEY_VIDEO_NEXT, /* drive next video source */
		VideoPrev 		 = KEY_VIDEO_PREV, /* drive previous video source */
		BrightnessCycle  = KEY_BRIGHTNESS_CYCLE, /* brightness up, after max is min */
		BrightnessAuto 	 = KEY_BRIGHTNESS_AUTO, /* Set Auto Brightness:
		manual brightness control is off, rely on ambient */
		DisplayOff 		 = KEY_DISPLAY_OFF, /* display device to off state */

		WWan 			 = KEY_WWAN, /* Wireless WAN : c_int = LTE, UMTS, GSM, etc. */
		RFKill 			 = KEY_RFKILL, /* Key that controls all radios */

		MicMute 		 = KEY_MICMUTE, /* Mute / unmute the microphone */
	}
}

#[cfg_attr(rustfmt, rustfmt_skip)]
enum_serde!(Key {
	Reserved,
	Esc,
	_1,
	_2,
	_3,
	_4,
	_5,
	_6,
	_7,
	_8,
	_9,
	_0,
	Minus,
	Equal,
	BackSpace,
	Tab,
	Q,
	W,
	E,
	R,
	T,
	Y,
	U,
	I,
	O,
	P,
	LeftBrace,
	RightBrace,
	Enter,
	LeftControl,
	A,
	S,
	D,
	F,
	G,
	H,
	J,
	K,
	L,
	SemiColon,
	Apostrophe,
	Grave,
	LeftShift,
	BackSlash,
	Z,
	X,
	C,
	V,
	B,
	N,
	M,
	Comma,
	Dot,
	Slash,
	RightShift,
	LeftAlt,
	Space,
	CapsLock,
	F1,
	F2,
	F3,
	F4,
	F5,
	F6,
	F7,
	F8,
	F9,
	F10,
	NumLock,
	ScrollLock,
	F11,
	F12,
	RightControl,
	SysRq,
	RightAlt,
	LineFeed,
	Home,
	Up,
	PageUp,
	Left,
	Right,
	End,
	Down,
	PageDown,
	Insert,
	Delete,
	LeftMeta,
	RightMeta,
	ScrollUp,
	ScrollDown,
	F13,
	F14,
	F15,
	F16,
	F17,
	F18,
	F19,
	F20,
	F21,
	F22,
	F23,
	F24,
	KP7,
	KP8,
	KP9,
	KPMinus,
	KP4,
	KP5,
	KP6,
	KPPlus,
	KP1,
	KP2,
	KP3,
	KP0,
	KPDot,
	Zenkakuhankaku,
	_102ND,
	RO,
	Katakana,
	Hiragana,
	Henkan,
	KatakanaHiragana,
	Muhenkan,
	KPJPComma,
	KPEnter,
	KPSlash,
	Macro,
	Mute,
	VolumeDown,
	VolumeUp,
	Power, /* SC System Power Down */
	KPEqual,
	KPPlusMinus,
	Pause,
	Scale, /* AL Compiz Scale : c_int = Expose */
	KPComma,
	Hangeul,
	Hanja,
	Yen,
	Compose,
	Stop, /* AC Stop */
	Again,
	Props, /* AC Properties */
	Undo, /* AC Undo */
	Front,
	Copy, /* AC Copy */
	Open, /* AC Open */
	Paste, /* AC Paste */
	Find, /* AC Search */
	Cut, /* AC Cut */
	Help, /* AL Integrated Help Center */
	Menu, /* Menu : c_int = show menu */
	Calc, /* AL Calculator */
	Setup,
	Sleep, /* SC System Sleep */
	Wakeup, /* System Wake Up */
	File, /* AL Local Machine Browser */
	Sendfile,
	DeleteFile,
	Xfer,
	Prog1,
	Prog2,
	WWW, /* AL Internet Browser */
	MSDos,
	ScreenLock,
	RotateDisplay, /* Display orientation for e.g. tablets */
	CycleWindows,
	Mail,
	Bookmarks, /* AC Bookmarks */
	Computer,
	Back, /* AC Back */
	Forward, /* AC Forward */
	CloseCD,
	EjectCD,
	EjectCloseCD,
	NextSong,
	PlayPause,
	PreviousSong,
	StopCD,
	Record,
	Rewind,
	Phone, /* Media Select Telephone */
	Iso,
	Config, /* AL Consumer Control Configuration */
	HomePage, /* AC Home */
	Refresh, /* AC Refresh */
	Exit, /* AC Exit */
	Move,
	Edit,
	KPLeftParen,
	KPRightParen,
	New, /* AC New */
	Redo, /* AC Redo/Repeat */
	PlayCD,
	PauseCD,
	Prog3,
	Prog4,
	Dashboard, /* AL Dashboard */
	Suspend,
	Close, /* AC Close */
	Play,
	FastForward,
	BassBoost,
	Print, /* AC Print */
	HP,
	Camera,
	Sound,
	Question,
	Email,
	Chat,
	Search,
	Connect,
	Finance, /* AL Checkbook/Finance */
	Sport,
	Shop,
	AltErase,
	Cancel, /* AC Cancel */
	BrightnessDown,
	BrightnessUp,
	Media,
	SwitchVideoMode, /* Cycle between available video outputs (Monitor/LCD/TV-out/etc) */
	KBDIllumToggle,
	KBDIllumDown,
	KBDIllumUp,
	Send, /* AC Send */
	Reply, /* AC Reply */
	ForwardMail, /* AC Forward Msg */
	Save, /* AC Save */
	Documents,
	Battery,
	Bluetooth,
	Wlan,
	Uwb,
	Unknown,
	VideoNext, /* drive next video source */
	VideoPrev, /* drive previous video source */
	BrightnessCycle, /* brightness up, after max is min */
	BrightnessAuto, /* Set Auto Brightness: manual brightness control is off, rely on ambient */
	DisplayOff, /* display device to off state */
	WWan, /* Wireless WAN : c_int = LTE, UMTS, GSM, etc. */
	RFKill, /* Key that controls all radios */
	MicMute, /* Mute / unmute the microphone */
});
