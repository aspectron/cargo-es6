const ASPECTRON_FLOW_UX_FLOW_UX_434 : &'static str = r###"
/*
* Flow-UX: flow-ux.js
* version: 1.0.0
*/











































































"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_TERMINAL_431 : &'static str = r###"




import '../resources/extern/colors/extend-string-prototypes.js';

import '../resources/extern/xterm/lib/xterm.js';
import '../resources/extern/xterm-addon-fit/lib/xterm-addon-fit.js';
import '../resources/extern/xterm-addon-web-links/lib/xterm-addon-web-links.js';

// import '/node_modules/xterm/lib/xterm.js';
// import '/node_modules/xterm-addon-fit/lib/xterm-addon-fit.js';
// import '/node_modules/xterm-addon-web-links/lib/xterm-addon-web-links.js';

/**
* @class FlowTerminal
* @extends BaseElement
*
* @prop {String} [background=rgba(0,0,0,0.0)] background color
* @prop {String} [foreground=#000000] foreground color
* @prop {Number} [font-size=20] font size in number
* @prop {String} [font-family=Consolas, 'Ubuntu Mono', courier-new, courier, monospace] text font-family 
* @prop {Boolean} [noscroll=false] set true to make overflow as hidden 
* @prop {Boolean} [fixed=false] set true to disable "`" and "Esc" keys which toggle terminal 
* @prop {Boolean} [noinput=false] set true to make terminal readonly
*
* @cssvar {color} --flow-terminal-bg-color terminal background color
* @cssvar {color} --flow-terminal-color terminal foreground color
* @cssvar {color} --flow-terminal-cursor terminal cursor color
* @cssvar {color} --flow-terminal-selection terminal selection color
*
* @example
* <flow-terminal id="terminal" background="#000" foreground="#FFF"></flow-terminal>
* @example
* document.querySelector("#terminal").prompt();
*/
export class FlowTerminal extends BaseElement {
	/**
	* @member {String} [background=rgba(0,0,0,0.0)] background color
	* @memberof XTerminal#
	*/

	/**
	* @member {String} [foreground=#000000] foreground color
	* @memberof XTerminal#
	*/

	/**
	* @member {Number} [font-size=20] font size in number
	* @memberof XTerminal#
	*/

	/**
	* @member {String} [font-family=Consolas, 'Ubuntu Mono', courier-new, courier, monospace] text font-family 
	* @memberof XTerminal#
	*/

	/**
	* @member {Boolean} [noscroll=false] set true to make overflow as hidden 
	* @memberof XTerminal#
	*/

	/**
	* @member {Boolean} [fixed=false] set true to disable "`" and "Esc" keys which toggle terminal 
	* @memberof XTerminal#
	*/

	/**
	* @member {Boolean} [noinput=false] set true to make terminal readonly 
	* @memberof XTerminal#
	*/

	static get properties() {
		return {
			background : { type : String },
			foreground : { type : String },
			'font-size' : { type : Number },
			'font-family' : { type : String },
			noscroll : { type : Boolean },
			fixed : { type : Boolean },
			noinput : { type : Boolean },
			resident : { type : Number },
			/*data: { type: Array }*/
		};
	}
	static get styles() {
		return [ScrollbarStyle, css`
			:host {
				/*background-color: rgba(0,0,0,0.0);*/
				/*color: black;*/
				min-height:100px;
				min-width:100px;
				display:flex;
				flex-direction:column;
				box-sizing:border-box;
			}
			:host(.hidden){visibility:hidden}
			#terminal .terminal{height:100%;}
			
			#terminal{flex:1;height: 100%;overflow: hidden;}
			/******* imported from xterm module **********/
			.xterm {
			    font-feature-settings: "liga" 0;
			    position: relative;
			    user-select: none;
			    -ms-user-select: none;
			    -webkit-user-select: none;
			    font-size:20px;
			    letter-spacing:0;
			}
			.xterm.focus,
			.xterm:focus {
			    outline: none;
			}
			.xterm .xterm-helpers {
			    position: absolute;
			    top: 0;
			    /**
			     * The z-index of the helpers must be higher than the canvases in order for
			     * IMEs to appear on top.
			     */
			    z-index: 5;
			}
			.xterm .xterm-helper-textarea {
			    /*
			     * HACK: to fix IE's blinking cursor
			     * Move textarea out of the screen to the far left, so that the cursor is not visible.
			     */
			    position: absolute;
			    opacity: 0;
			    left: -9999em;
			    top: 0;
			    width: 0;
			    height: 0;
			    z-index: -5;
			    /** Prevent wrapping so the IME appears against the textarea at the correct position */
			    white-space: nowrap;
			    overflow: hidden;
			    resize: none;
			    font-size:20px;
			    letter-spacing:0;
			}
			.xterm .composition-view {
			    /* TODO: Composition position got messed up somewhere */
			    background: #000;
			    color: #FFF;
			    display: none;
			    position: absolute;
			    white-space: nowrap;
			    z-index: 1;
			}
			.xterm .composition-view.active {
			    display: block;
			}
			.xterm .xterm-viewport {
			    /* On OS X this is required in order for the scroll bar to appear fully opaque */
			    background-color: #000;
			    overflow-y: scroll;
			    cursor: default;
			    position: absolute;
			    right: 0;
			    left: 0;
			    top: 0;
			    bottom: 0;
			}
			.xterm .xterm-screen {
			    position: relative;
			}
			.xterm .xterm-screen canvas {
			    position: absolute;
			    left: 0;
			    top: 0;
			}
			.xterm .xterm-scroll-area {
			    visibility: hidden;
			}
			.xterm-char-measure-element {
			    display: inline-block;
			    visibility: hidden;
			    position: absolute;
			    top: 0;
			    left: -9999em;
			    line-height: normal;
			    letter-spacing:normal;
			    _width:9px;
			    overflow:hidden
			}
			.xterm {
			    cursor: text;
			}
			.xterm.enable-mouse-events {
			    /* When mouse events are enabled (eg. tmux), revert to the standard pointer cursor */
			    cursor: default;
			}
			.xterm.xterm-cursor-pointer {
			    cursor: pointer;
			}
			.xterm.column-select.focus {
			    /* Column selection mode */
			    cursor: crosshair;
			}
			.xterm .xterm-accessibility,
			.xterm .xterm-message {
			    position: absolute;
			    left: 0;
			    top: 0;
			    bottom: 0;
			    right: 0;
			    z-index: 10;
			    color: transparent;
			}
			.xterm .live-region {
			    position: absolute;
			    left: -9999px;
			    width: 1px;
			    height: 1px;
			    overflow: hidden;
			}
			.xterm-dim {
			    opacity: 0.5;
			}
			.xterm-underline {
			    text-decoration: underline;
			}
		`];
	}

	constructor() {
		// Must call superconstructor first.
		super();

		this.resident = 0;
		this.residentBuffers = [];
		this.residentBuffersLength = 0;
	}

	/**
	* Define a template for the new element by implementing LitElement's
	* `render` function. `render` must return a lit-html TemplateResult.
	*/
	render() {
		return html`<div id="colorClc"></div><div id="terminal"></div><div id="clipboard"></div>`;
	}

	/**
	* Initilize terminalHolder element then calls initTerminal(), initClipboard()
	*/
	firstUpdated() {
		this.terminalHolder = this.renderRoot.getElementById('terminal');
		this.colorClc = this.renderRoot.getElementById("colorClc");
		this.initTerminal();
		let clipboardHolder = this.renderRoot.getElementById('clipboard');
		this.initClipboard(this.terminalHolder, clipboardHolder);
		this._updateBGColorChange = this.updateBGColorChange.bind(this);
		document.body.addEventListener("flow-theme-changed", this._updateBGColorChange)
		this._updateBGColorChange();
	}

	getVarColor(varName, defaults, style){
		style = style || getComputedStyle(this);
		this.colorClc.style.color = style.getPropertyValue(varName) || defaults;
		return getComputedStyle(this.colorClc).color;
	}

	updateBGColorChange(){
		let style = getComputedStyle(this);
		let theme = this.term.getOption("theme") || this.termTheme;
		//console.log('%cOld Theme', `color:${theme.foreground};background-color:${theme.background}`, theme)
		
		let background = this.getVarColor('--flow-terminal-bg-color', theme.background, style);
		let foreground = this.getVarColor('--flow-terminal-color', theme.foreground, style);
		let cursor = this.getVarColor('--flow-terminal-cursor', theme.cursor || foreground, style);
		let selection = this.getVarColor('--flow-terminal-selection', theme.selection || foreground, style);

		theme.background = background;
		theme.foreground = foreground;
		theme.cursor = cursor;
		theme.selection = selection;

		//console.log('%cNew Theme', `color:${foreground};background-color:${background}`, theme)
		this.term.setOption('theme', theme)
		this.term._core._setTheme(theme)
	}

	removeBgColorListerner(){
		if(!this._updateBGColorChange)
			return
		document.body.removeEventListener("flow-theme-changed", this._updateBGColorChange)
	}

	/**
	* Initilize paste event handling process
	* @param {HTMLElement} eventEl terminalHolder element
	* @param {HTMLElement} holder textarea parent node, where new textarea will be created for paste event
	*/
	initClipboard(eventEl, holder){
		this.term.attachCustomKeyEventHandler((e)=>{
			if(e.type != 'keydown')
				return;
			this.onClipboardKeyDown(e);
		});
	}
	onClipboardKeyDown(e){
		if(e.key == "v" && (e.metaKey || e.ctrlKey)){
			//e.preventDefault();
			navigator.clipboard.readText().then((text) => {
				this.onClipboardPaste(text);
			})
		}
		else
		if(e.key == "c" && (e.metaKey || e.ctrlKey)){
			//e.preventDefault();
			let text = this.term.getSelection();
			if(text) {
				console.log("copying text to clipboard:",text);
				navigator.clipboard.writeText(text);
			}
		}
	}
	onClipboardPaste(value){
		this.term.focus();
		this.pasteText(value);
	}
	pasteText(text){
		if(this.running) {
			this.term.write(text);
		} else {
			let t = this.buffer.split('');
			t.splice(this.cursorX,0,text);
			this.buffer = t.join('');
			this.trail(this.cursorX, { rewind : true, offset : text.length });
			this.cursorX+=text.length;
		}
	}
	updated(changedProperties) {
		changedProperties.forEach((oldValue, propName) => {
			//this.log(`${propName} changed. oldValue: ${oldValue}, new: ${this[propName]}`);
			//if(propName == "data")
			//	this.updateGraph(this.data);
		});
	}

	trail (x, options) {
		const { rewind, eraseLast, offset } = options;
		let tail = this.buffer.substring(x)+(eraseLast?' ':'');
		this.term.write(tail);
		for(let i = 0; rewind && i < tail.length-(offset||0); i++)
			this.term.write('\b');
	}


	initTerminal(){

		/*
		this.log("INIT TERMINAL:", {
			fontSize: this['font-size'] || 20,
			background: this.background || 'rgba(0,0,0,0.0)',
			//background: 'rgba(255,255,255,0.75)',
			foreground: this.foreground || '#000000',
			noscroll : this.noscroll
		});
		*/

		let term = new Terminal({
			allowTransparency: true,
			fontFamily: this['font-family'] || 'Consolas, Ubuntu Mono, courier-new, courier, monospace',
			fontSize: this['font-size'] || 20,
			cursorBlink : true,
			theme: {
				background: this.background || 'rgba(0,0,0,0.0)',
				foreground: this.foreground || '#000000',
				cursor: this.cursor || this.foreground || "#FFF"
			}
		});
    	this.term = term;
    	this.termTheme = term.getOption("theme") || {}

    	window.terms = window.terms || [];
    	window.terms.push(term);

    	//this.log("termtermterm:theme", this.termTheme)
    	this.addons = {
	    	weblinks : new WebLinksAddon.WebLinksAddon((event,uri) => {
				this.handleLink(event,uri);
			}),
	    	fit : new FitAddon.FitAddon()
	    };

		//$(this.terminalHolder).on("keydown", e=>{
		this.terminalHolder.addEventListener("keydown", e=>{
				//this.log("e.key", e.key)
			if(e.ctrlKey || e.metaKey) {
				if(e.key == "+" || e.key == "="){
					this.setFontSizeDelta(1, e);
				}else if(e.key == "-" || e.key == "_"){
					this.setFontSizeDelta(-1, e);
				}
			}
		});

	    Object.entries(this.addons).forEach((e,i) => { 
	    	let [k,addon] = e; term.loadAddon(addon); 
	    	let instance = term._addonManager._addons[i];
	    	this.addons[k] = instance;
	    });
		//	https://github.com/xtermjs/xterm.js/tree/master/addons/xterm-addon-fit	
		//	https://github.com/xtermjs/xterm.js/tree/master/addons/xterm-addon-attach	
		//	term.loadAddon(new AttachAddon.AttachAddon(websocket));

		//term.open(document.getElementById("terminalx"));
		term.open(this.terminalHolder)
		this.buffer = '';
		this.cursorX = 0;
		this.history = [ ];
		this.historyIdx = 0;
		term.onResize(()=>{
			var size = {rows: term.rows, cols:term.cols};
			this.fire("terminal-resize", size);
			this.log('terminal-resize', size)
		})


		const keys = {
			Tab : (e, key) => {
				// TODO - completion handler
			},
			Backspace : (e, key) => {
				if(this.cursorX == 0)
					return;
				term.write('\b');
				this.cursorX--;
				let t = this.buffer.split('');
				t.splice(this.cursorX,1);
				this.buffer = t.join('');
				this.trail(this.cursorX, { rewind : true, eraseLast : true });
			},
			Delete : (e, key) => {
				let t = this.buffer.split('');
				t.splice(this.cursorX,1);
				this.buffer = t.join('');
				this.trail(this.cursorX, { rewind : true, eraseLast : true });
			},
			Inject : (e,key) => {
				let t = this.buffer.split('');
				t.splice(this.cursorX,0,key);
				this.buffer = t.join('');
				this.trail(this.cursorX, { rewind : true, offset : 1 });
				this.cursorX++;
			},
			Enter : async (e) => {
				e.stopPropagation();
				let { buffer, history } = this;
				let { length } = history;

				term.write('\r\n');
				this.buffer = '';
				this.cursorX = 0;

				if(buffer) {
					if(!length || history[length-1])
						this.historyIdx = length;
					else
						this.historyIdx = length-1;
					this.history[this.historyIdx] = buffer;
					this.historyIdx++;

					this.running = true;
					await this.digest(buffer);
					this.running = false;
				}

				this.prompt();
			},
			Escape : async (e) => { },
			PageDown : async (e) => { },
			PageUp : async (e) => { },
			Home : async (e) => { 
				let l = this.cursorX;
				term.write(`\x1B[${l}D`);
				this.cursorX = 0;
			},
			End : async (e) => { 
				let l = this.buffer.length - this.cursorX;
				term.write(`\x1B[${l}C`);
				this.cursorX = this.buffer.length;
			},
			ArrowLeft : (e, key) => {
				if(this.cursorX == 0)
					return;
				this.cursorX--;
				term.write(key);
			},
			ArrowRight : (e, key) => {
				if(this.cursorX < this.buffer.length) {
					this.cursorX++;
					term.write(key);
				}
			},
			ArrowUp : () => {
				if(this.historyIdx == 0)
					return;
				this.history[this.historyIdx] = this.buffer;
				this.historyIdx--;
				this.buffer = this.history[this.historyIdx] || '';
				this.term.write(`\x1B[2K\r${this.prompt_}${this.buffer}`);
				this.cursorX = this.buffer.length;
			},
			ArrowDown : () => {
				if(this.historyIdx >= this.history.length)
					return;

				this.history[this.historyIdx] = this.buffer;
				this.historyIdx++;
				this.buffer = this.history[this.historyIdx] || '';
				this.term.write(`\x1B[2K\r${this.prompt_}${this.buffer}`);
				this.cursorX = this.buffer.length;
			},
		}

		term.onKey((e_) => {

			if(this.noinput)
				return;

			const e = e_.domEvent;
			const termKey = e_.key;
			const { key, keyCode } = e;
			if(!this.fixed && (key == "`" || key == "Escape"))
				return this.toggleTerminal();

			if(this.stdinSink)
				return this.stdinSink({termKey,key,keyCode});

			const printable = !e.altKey && !e.ctrlKey && !e.metaKey && !key.match(/^F\d+$/);

			if(keys[key]) {
				keys[key](e,termKey);
			} else 
			if (printable) {
				keys.Inject(e,termKey);
			}

			//console.log('cursorX:',this.cursorX,'buffer:',this.buffer,'key:',key,'tk:',termKey);
			//console.log('x:',term._core.buffer.x,'plen:',this.promptLength);
			//console.log('keys:',key,keyCode,term._core.buffer);
			//console.log(this.buffer);
		});
		if(window.ResizeObserver){
			this.resizeObserver = new ResizeObserver(e => {
				this.updateTerminalSize();
			});
			this.resizeObserver.observe(this.terminalHolder);
		}
		this.setFontSize(this['font-size'] || this.getFontSize());

		if(this.noscroll) {
			let xtv = this.shadowRoot.querySelector(".xterm-viewport");
			xtv.style.overflow = 'hidden';
		}
	}
	get prompt_() {
		return `${this.promptPrefix?this.promptPrefix()+' ':''}$ `;
	}
	/**
	* prompt the terminal
	*/
	prompt(){
		this.cursorX = 0;
		this.buffer = '';

		let prompt = this.prompt_;
		this.term.write("\r\n"+prompt);
		this.promptLength = ("\r\n"+prompt).length;
	}

	enableResidentMode(length) {
		this.resident = length;
	}

	flushResidentBuffers() {
		while(this.residentBuffers.length)
			this.term.write(this.residentBuffers.shift());
	}

	writeToResidentBuffers(text) {
		if(!this.resident)
			return this.term.write(text);

		this.residentBuffers.push(text);
		this.residentBuffersLength += text.length;
		while(this.residentBuffersLength > this.resident && this.residentBuffers.length > 1) {
			let tip = this.residentBuffers.shift();
			this.residentBuffersLength -= tip.length;
		}
	}

	writeln(...args) {

		if(!args.length)
			args = [''];

		if(this.running) {
			this.term.write(args.join(' ')+'\r\n');
		}
		else {
			this.term.write(`\x1B[2K\r`+args.join(' ')+'\r\n');
			let prompt = `${this.prompt_}${this.buffer}`;
			this.term.write(prompt);
			let l = this.buffer.length-this.cursorX;
			while(l--)
				this.term.write('\b');
		}
	}
	write(...args){
		this.writeln(...args)
	}
	updateTerminalSize() {
		let term = this.term;
		//this.log("updateTerminalSize", this, this.term)
		//if(!$(this).width() || !term)
		if(!this.offsetWidth || !term)
			return
		let charSizeService = term._core._charSizeService
		if(charSizeService && !charSizeService.hasValidSize){
			charSizeService.measure()
			//if(term._core._renderService)
			//	term._core._renderService._updateDimensions();
		}
		let addon = this.addons.fit.instance;
		let dimensions = addon.proposeDimensions()
		addon.fit();
		//this.addons.fit.instance.fit();
	}
	/**
	* toggle terminal css class `hidden`
	*/
	toggleTerminal(){
		this.classList.toggle("hidden");
	}
	async digest(cmd) {
		return new Promise( async (resolve, reject) => {
			if(!cmd) {
				resolve();
				return;
			}

			if(cmd == 'history') {
				[...new Set(this.history)].forEach(t => this.write(t));
				resolve();
				return;
			}

			if(cmd == 'history reset') {
				this.history.forEach(t => this.write(t));
				resolve();
				return;
			}
			if(!this.sink)
				return resolve();

			this.sink.digest(cmd).then((resp)=>{
				if(resp && typeof(resp) == 'string')
					this.write(resp);
				resolve();
			},(err)=>{
				this.write((err||'unknown digest error').toString().brightRed);
				resolve();
			})
		})
	}
	registerSink(sink) {
		if(!sink.digest || !sink.complete)
			throw new Error("Missing digest() or complete() in the terminal sink");
		this.sink = sink;
	}

	captureStdin(sink) {
		this.stdinSink = sink;
	}

	releaseStdin(sink) {
		delete this.stdinSink;
	}

	isCaptured() {
		return !!this.stdinSink;
	}

	getSize() {
		return { rows: this.term.rows, cols: this.term.cols };
	}

	setFontSizeDelta(delta, e){
		this.setFontSize(this.getFontSize()+delta);
		if(e && e.preventDefault){
			e.preventDefault();
			e.stopPropagation();
		}
	}
	/**
	* Set font size and update terminal size
	* @param {number} [size=20] font size should be >= 7
	*/
	setFontSize(size){
		if(!size)
			size = 20;
		else
		if(size < 7)
			size = 7;
		this.setSetting("fontSize", size);
		this.term.setOption("fontSize", size)
		this.updateTerminalSize();
	}
	getFontSize(size){
		return parseFloat(this.getSetting("fontSize", size || 20));
	}
	increaseFontSize(){
		this.setFontSizeDelta(1);
	}
	decreaseFontSize(){
		this.setFontSizeDelta(-1);
	}
	getStorageKeyPrefix(){
		return (this.id || this.dataKey || "xterm")+"_";
	}
	getSetting(name, defaults){
		let key = this.getStorageKeyPrefix();
		let ls = window.localStorage || {};
		return ls[key+name] || defaults;
	}
	setSetting(name, value){
		let key = this.getStorageKeyPrefix();
		let ls = window.localStorage || {};
		ls[key+name] = value;
	}
	clear() {
		this.term.write(`\x1B[2J\x1B[H`);
	}
	handleLink(event, link) {
		this.linkHandler?.(event,link);
	}
	registerLinkHandler(fn) {
		this.linkHandler = fn;
	}
}

FlowTerminal.define("flow-terminal")

"###;

const ASPECTRON_FLOW_UX_SRC_SVG_414 : &'static str = r###"
export class Segment {
	constructor(path, begin, end, circular) {
		this.path = path;
		this.reset();
		this.begin = typeof begin !== 'undefined' ? this.valueOf(begin) : 0;
		this.end = typeof end !== 'undefined' ? this.valueOf(end) : this.length;
		this.circular = typeof circular !== 'undefined' ? circular : false;
		this.timer = null;
		this.animationTimer = null;
		this.draw(this.begin, this.end, 0, {circular: this.circular});
	}

	reset(){
		this.length = this.path.getTotalLength();
		this.path.style.strokeDashoffset = this.length * 2;
	}

	draw(begin, end, duration, options){
		this.circular = options && options.hasOwnProperty('circular') ? options.circular : false;
		if(duration){
			this.stop();

			var that = this;
			var delay = options && options.hasOwnProperty('delay') ? parseFloat(options.delay) * 1000 : 0;
			if(delay){
				delete options.delay;
				this.timer = setTimeout(function(){
					that.draw(begin, end, duration, options);
				}, delay);
				return this.timer;
			}

			this.startTime = new Date();
			this.initBegin = this.begin;
			this.initEnd = this.end;
			this.finalBegin = this.valueOf(begin);
			this.finalEnd = this.valueOf(end);
			this.pausedTime = 0;
			this.duration = duration;
			this.easing = options && options.hasOwnProperty('easing') ? options.easing : null;
			this.update = options && options.hasOwnProperty('update') ? options.update : null;
			this.callback = options && options.hasOwnProperty('callback') ? options.callback : null;

			this.animationTimer = requestAnimationFrame(this.play.bind(this));
		}else{
			this.path.style.strokeDasharray = this.strokeDasharray(begin, end);
		}
	}

	play() {
		var now = new Date();
		if (this.pausedTime) {
			this.startTime.setMilliseconds(this.startTime.getMilliseconds() + (now - this.pausedTime));
			this.pausedTime = 0;
		}
		var elapsed = (now - this.startTime) / 1000;
		var time = (elapsed / parseFloat(this.duration));
		var t = time;

		if(typeof this.easing === 'function'){
			t = this.easing(t);
		}

		if(time > 1){
			t = 1;
			this.stop();
		}else{
			this.animationTimer = requestAnimationFrame(this.play.bind(this));
		}

		this.drawStep(t);

		if(time > 1 && typeof this.callback === 'function'){
			return this.callback.call(this);
		}
	}

	pause() {
		if (this.animationTimer) {
			this.stop();
			this.pausedTime = new Date();
		}
	}

	resume() {
		if (!this.animationTimer) {
			this.animationTimer = requestAnimationFrame(this.play.bind(this));
		}
	}

	stop(){
		cancelAnimationFrame(this.animationTimer);
		this.animationTimer = null;
		clearTimeout(this.timer);
		this.timer = null;
	}

	drawStep(t) {
		this.begin = this.initBegin + (this.finalBegin - this.initBegin) * t;
		this.end = this.initEnd + (this.finalEnd - this.initEnd) * t;

		this.begin = this.begin < 0 && !this.circular ? 0 : this.begin;
		this.begin = this.begin > this.length && !this.circular ? this.length : this.begin;
		this.end = this.end < 0 && !this.circular ? 0 : this.end;
		this.end = this.end > this.length && !this.circular ? this.length : this.end;

		if (this.end - this.begin <= this.length && this.end - this.begin > 0) {
			this.draw(this.begin, this.end, 0, {circular: this.circular});
		} else {
			if (this.circular && this.end - this.begin > this.length) {
				this.draw(0, this.length, 0, {circular: this.circular});
			} else {
				this.draw(this.begin + (this.end - this.begin), this.end - (this.end - this.begin), 0, {circular: this.circular});
			}
		}

		if(typeof this.update === 'function'){
			this.update(this);
		}
	}

	strokeDasharray(begin, end){
		this.begin = this.valueOf(begin);
		this.end = this.valueOf(end);
		if(this.circular){
			var division = this.begin > this.end || (this.begin < 0 && this.begin < this.length * -1)
				? parseInt(this.begin / parseInt(this.length)) : parseInt(this.end / parseInt(this.length));
			if(division !== 0){
				this.begin = this.begin - this.length * division;
				this.end = this.end - this.length * division;
			}
		}
		if(this.end > this.length){
			var plus = this.end - this.length;
			return [this.length, this.length, plus, this.begin - plus, this.end - this.begin].join(' ');
		}
		if(this.begin < 0){
			var minus = this.length + this.begin;
			if(this.end < 0){
				return [this.length, this.length + this.begin, this.end - this.begin, minus - this.end, this.end - this.begin, this.length].join(' ');
			}else{
				return [this.length, this.length + this.begin, this.end - this.begin, minus - this.end, this.length].join(' ');
			}
		}
		return [this.length, this.length + this.begin, this.end - this.begin].join(' ');
	}

	valueOf(input){
		var val = parseFloat(input);
		if(typeof input === 'string' || input instanceof String){
			if(~input.indexOf('%')){
				var arr;
				if(~input.indexOf('+')){
					arr = input.split('+');
					val = this.percent(arr[0]) + parseFloat(arr[1]);
				}else if(~input.indexOf('-')){
					arr = input.split('-');
					if(arr.length === 3){
						val = -this.percent(arr[1]) - parseFloat(arr[2]);
					}else{
						val = arr[0] ? this.percent(arr[0]) - parseFloat(arr[1]) : -this.percent(arr[1]);
					}
				}else{
					val = this.percent(input);
				}
			}
		}
		return val;
	}

	percent(value){
		return parseFloat(value) / 100 * this.length;
	}
}
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_ANCHOR_362 : &'static str = r###"

const flowAnchors = [];

/**
* @class FlowAnchor
* @extends BaseElement
* @property {String} [for]
* @property {String} [type]
* @example
*   <flow-anchor>Button 1</flow-anchor>
*
*
*/
export {flowAnchors}
export class FlowAnchor extends BaseElement {
	static get properties() {
		return {
			for : { type : String },
			type : { type : String }
		}
	}

	static get styles() {
		return css`
			:host {
				display : block;
			}
		`;
	}

	constructor() {
		super();
		flowAnchors.push(this);
	}

	render() {
		return html`<slot></slot>`;
	}
}

FlowAnchor.define('flow-anchor');
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_ASYNC_396 : &'static str = r###"


const deferred = () => {
    //let methods = {};
	let resolve;
	let reject;
    const p = new Promise((resolve_, reject_) => {
		resolve = resolve_;
		reject = reject_;
        //methods = { resolve, reject };
    });
	p.resolve = resolve;
	p.reject = reject;
	return p;
//    return Object.assign(p, methods);
}

export class AsyncQueue {
	constructor(opt) {
		this.pending = [];
        this.processed = 0;
        this.inflight = 0;
		this.signal = deferred();
		this.done = false;
		this.max = opt?.max || 0;
	}
	[Symbol.asyncIterator]() { return this.iterator(); }
	post(v) {
		if(this.done)
			return;
		if(this.max) {
			while(this.pending.length >= this.max)
				this.pending.shift();
		}
		this.pending.push(v);
		this.signal.resolve();
	}
	stop(err) {
		this.err = err;
		this.abort = true;
		this.done = true;
		if(!this.inflight) {
			this.signal.resolve();
		}
	}
	clear() {
		this.pending = [];
		if(this.inflight) {
			this.abort = true;
			this.reset_ = true;
		}
	}
    get length() {
        return this.pending.length+this.inflight;
    }
	async *iterator() {

		if(this.done) {
			this.done = false;
			if(!this.pending.length)
				this.signal = deferred();
		}

		while(true) {
			if(this.pending.length === 0) {
				await this.signal;
			}
			if (this.err)
				throw this.err;

			const pending = this.pending;
			this.inflight = pending.length;
			this.pending = [];
			let processed = 0;
			for (; processed < pending.length && !this.abort; processed++) {
                this.processed++;
                yield pending[processed];
				this.inflight--;
			}


			if(this.reset_) {
				this.abort = false;
				this.reset_ = false;
				pending.length = 0;
			}

			if(this.done) {
				this.abort = false;
				const incoming = this.pending.length;
				if(incoming)
					this.pending = processed ? pending.slice(processed).concat(this.pending) : pending.concat(this.pending);
				else
					this.pending = processed ? pending.slice(processed) : pending;
				this.inflight = 0;
				break;
			}
			else if (this.pending.length === 0) {
				this.inflight = 0;
				pending.length = 0;
				this.pending = pending;
				this.signal = deferred();
			}
		}
	}
}

export class AsyncQueueSubscriber {
	constructor(manager, subject) {
		this.uid = UID();
		this.queue = new AsyncQueue();
		this.manager = manager;
		this.subject = subject;
		this.events = { };
//		this.ready = false;
//		this.connectHandlers = [];
	}

	[Symbol.asyncIterator]() { return this.queue.iterator(); }


	subscribe(subject) {
		this.unsubscribe();
		this.subject = subject;
		this.manager.subscribe(subject, this);
	}

	unsubscribe() {
		this.manager.remove(this);
		let { subject, uid, queue } = this;
		queue.clear();
		//queue.stop();

		this.event('unsubscribe');
		// for(const handler of this.events.unsubscribe||[])
		// 	handler();
	}

	resubscribe() {
		this.manager.subscribe(this.subject, this);
	}

	close() {
		this.unsubscribe();
		//queue.clear();
		this.queue.stop();
	}

	stop(){
		this.queue.stop();
	}

	// changeSubject(subject) {
	// 	this.manager.changeSubject(this, subject);
	// }

	// connect(connectHandler) {

	// }

	on(event, handler) {
		if(!this.events[event])
			this.events[event] = [];
		this.events[event].push(handler);
	}

	event(name, subject) {
		//console.log('EVENT:', name, subject);
		dpc(()=>{
			for(const handler of this.events[name]||[])
				handler(subject);
		})
	}
}

export class AsyncQueueSubscriberMap {
	constructor() {
		this.map = new Map();
	}

	subscribe(subject, subscriber = null) {
		let subscribers = this.map.get(subject);
		if(!subscribers) {
			subscribers = new Map();
			this.map.set(subject,subscribers);
		}
		//let queue = new AsyncQueue();
		//let subscriber = 
		if(!subscriber)
			subscriber = new AsyncQueueSubscriber(this, subject);
		else
			subscriber.manager = this;
		subscribers.set(subscriber.uid,subscriber);
		subscriber.event('subscribe', subject);
		return subscriber;
	}

	remove(subscriber) {
		let { subject, uid } = subscriber;
		let subscribers = this.map.get(subject);
		if(subscribers) {
			subscribers.delete(uid);
			if(!subscribers.size)
				this.map.delete(subject);
		} else {
			// console.trace('WARNING - no previous subscription for subject',subject,'uid:',uid);
			console.log('note','no previous subscription for subject',subject);
		}
		
		// let subscriber_ = this.subscribers.get(subscriber.uid);
		// if(subscriber_) {
		// 	this.subscribers.delete(subscriber.uid);
		// }
	}

	post(subject, msg) {
		let subscribers = this.map.get(subject);
		if(subscribers && subscribers.size)
			subscribers.forEach((subscriber)=>{
				subscriber.queue.post(msg);
			});
	}

	shutdown() {
		this.map.forEach((subscribers) => {
			subscribers.forEach(subscriber => {
				const { uid, queue } = subscriber;
				// ???????????
				queue.stop();
				queue.clear();
			});
		});
		this.map.clear();
	}

	forEach(iter) {
		this.map.forEach((subscribers, subject)=>{
			subscribers.forEach((subscriber)=>{
				iter(subscriber);
			})
		});
	}

	/*
	changeSubject(subscriber, newSubject) {
		const { uid, subject } = subscriber;
		let subscribers = this.map.get(subject);
		if(subscribers)
			subscribers.delete(uid);
		subscribers = this.map.get(newSubject);
		if(!subscribers) {
			subscribers = new Map();
			this.map.set(newSubject, subscribers);
		}
		subscribers.set(subscriber.uid, subscriber);
	}
	*/
}

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_CAPTION_BAR_399 : &'static str = r###"








/**
 *
 * @export
 * @class FlowCaptionBar
 * @extends {BaseElement}
 * @property {String} [icon] [FaIcon]{@link FaIcon} css classes i.e `fal fa-chart-network`
 * @property {Array.<FlowTabConfig>} [tabs] [FlowTabs.tabs]{@link FlowTabs#tabs}
 * @property {String} [active-tab] [FlowTabs.active]{@link FlowTabs#active}
 * @property {String} [logo] url to the logo image
 * @property {String} [version] text containig version information
 * @property {String} [caption] window caption text
 * @prop {ActiveDisplay} target-display *blocked/active* display type for target elements (default: block)
 *
 * @example
 * <flow-caption-bar icon="fal fa-chart-network">FLOW</flow-caption-bar>
 * <flow-caption-bar logo="/resources/images/logo.png"></flow-caption-bar>
 * 
 * JavaScript
 * initCaption(){
 *		let caption = document.querySelector('flow-caption-bar');
		caption.close = ()=>{ ... };
		caption.logo = `/resources/images/logo-${this.theme}-bg.png`;
		caption.version = '1.0.0';
		caption.tabs = [{
			title : "Tab1",
			id : "tab1",
			cls: "tab1"
		},{
			title : "Tab2",
			id : "tab2"
		},{
			title : "Tab3",
			id : "tab3",
			disable: true,
			section: 'filter'
		}];

		caption["active"] = "tab1";
	}
 * @cssvar {font-family} [--flow-caption-bar-font-family=var(--flow-font-family, "Julius Sans One")]
 * @cssvar {font-weight} [--flow-caption-bar-font-weight=var(--flow-font-weight, bold)]
 * @cssvar {font-size} [--flow-caption-bar-font-size=var(--flow-font-size, 1.5rem)]
 * @cssvar {color} [--flow-caption-bar-primary-color=var(--flow-primary-color, rgba(0,151,115,1))]
 * @cssvar {color} [--flow-primary-color=rgba(0,151,115,1)]
 * @cssvar {margin} [--flow-caption-tabs-margin=0px]
 * @cssvar {background-color} [--flow-caption-logo-bg-color=var(--flow-background-color,#232323)]
 */
export class FlowCaptionBar extends BaseElement {
	static get properties() {
		return {
			caption : { type : String },
			version : { type : String },
			icon : { type : String },
			logo : { type : String },
			tabs : { type : Array},
			"active-tab" : { type : String },
			"target-display":{type: String, value:"block"}
		}
	}

	constructor() {
		super();
		this.tabs = null;
		this['target-display'] = 'block';
		if(typeof nw != 'undefined' && typeof nw.Window != 'undefined'){
			this.window = nw.Window.get();
			this.window.on("maximize", ()=>{
				//this.log("maximize")
				this.window._maximize = true;
				this.updateMaxIcon();
			});
			this.window.on("restore", ()=>{
				//this.log("restore")
				this.window._maximize = false;
				this.updateMaxIcon();
			})
			this.window.on("resize", ()=>{
				//this.log("resize")
				this.window._maximize = false;
				this.updateMaxIcon();
			})
		}
		else
			this.window = window;
	}

	static get styles() {
		return [this.svgStyle, css`
			/*div { border: 1px solid red; }*/
			:host {
				width:var(--flow-caption-bar-width, 100vw);
				user-select:none;
			}
			.host {
				width:var(--flow-caption-bar-host-width, calc(100vw - 8px));
				min-height: 28px;
				/* background-color: green; */
				display: flex;
				flex-direction: row;
				font-family: var(--flow-caption-bar-font-family, var(--flow-font-family, "Julius Sans One"));
				font-weight: var(--flow-caption-bar-font-weight, var(--flow-font-weight, bold));
				font-size: var(--flow-caption-bar-font-size, var(--flow-font-size, 1.5rem));
				color: var(--flow-caption-bar-primary-color, var(--flow-primary-color, rgba(0,151,115,1.0)));
				padding: 4px;
				/*border-bottom: 1px solid rgba(0,151,115,0.5);*/
			}

			.caption {
				-webkit-app-region: drag;
				padding-left: 8px;
				padding-top: 2px;
				white-space: nowrap;
				overflow: hidden;
				text-overflow: ellipsis;
				/*border: 1px solid red;*/
			}

			.tabs {
				flex: 1;
				margin:var(--flow-caption-tabs-margin, 0px);
				z-index:3;
			}
			svg.icon{
				cursor:pointer;
			}
			fa-icon,
			svg.icon:hove{
				margin: 0px 4px 0px 4px;
				--fa-icon-size: 28px;
				--fa-icon-color: var(--flow-primary-color, rgba(0,151,115,1.0));
			}

			fa-icon:hover,
			svg.icon:hove{
				filter: brightness(0.8);
			}
			fa-icon:active {
				transform: translate(1px,1px);	
			}

			.logo {
				left: 0px;
				top: 0px;
				width:52px;
				height:52px;
				background-position: center;
				background-size: contain;
				background-repeat: no-repeat;
				z-index: 4;
				background-color:var(--flow-caption-logo-bg-color,var(--flow-background-color,#232323));
				margin-left: -4px;
				padding: 4px 8px 4px 4px;
			}
			.caption{display:flex;flex-direction:column;}
			.version {
				font-size: 12px;
				/*border: 1px solid red;*/
				/*margin: 10px 8px 4px 4px;*/
				padding: 4px 8px 2px 4px;
				opacity: 0.75;
				font-family: "Consolas";
				display: flex;
				flex-direction: column;
			}
			.language-icon{position:relative;margin-right:10px;}
			.language-icon svg{cursor:pointer}
			.language-icon:after{
				content:"";
				position:absolute;
				top:12px;
				right:-5px;
				width:0px;
				height:0px;
				border:5px solid transparent;
				border-top:5px solid var(--flow-primary-color);
			}
			.logo-icon,.logo,.caption{-webkit-app-region:drag;cursor:move;}

			.caption-content { height: min-content;display:flex;flex-direction:row;  }
			*[flex] { flex: 1; }
			
		`];
	}

	render() {
		let tabs = this.tabs? this.tabs.slice(0):[];
		this.maxicon = (this.window._maximize ? 'window-restore':'window-maximize');
		return html`
			<div class='host'>
				${
					!this.icon?'':html
					`<svg class="icon" @click="${this.onIconClick}">
						<use href="${this.iconPath(this.icon)}"></use>
					</svg>`
				}
				${
					!this.logo?'':html
					`<div class="logo" @click="${this.onLogoClick}"
						style="background-image:url(${this.logo});"></div>`
				}
				<div class="caption">
					<div class="caption-content">
						<div class="title">
							${this.caption||html`<slot></slot>`}
						</div>
						<div class="version">
							<div flex></div>
							<div>
								${this.version?`v${this.version}`:html`<slot name='version'></slot>`}
							</div>
						</div>
					</div>
					<div flex>
					</div>
				</div>
				<div class="tabs">
				<flow-tabs active=${this['active-tab']} .tabs=${tabs} part="tabs"
					target-display="${this['target-display']}">
					<slot name="flow-tabs"></slot>
				</flow-tabs>
				</div>

				<div class="language-icon">
					<svg class="icon" @click="${this.onLangClick}">
						<use href="${this.iconPath('language')}"></use>
					</svg>
				</div>
				<svg class="icon" @click="${this.minimize}">
					<use href="${this.iconPath('window-minimize')}"></use>
				</svg>
				<svg class="icon" @click="${this.toggleMaximize}">
					<use href="${this.iconPath(this.maxicon)}"></use>
				</svg>
				<svg class="icon" @click="${this.close}">
					<use href="${this.iconPath('times-circle')}"></use>
				</svg>
			</div>
		`;
	}
	requestTabsUpdate(){
		this.shadowRoot.querySelector("flow-tabs").requestUpdate();
	}

	onIconClick(e){
		this.fire("flow-caption-icon-clicked", {e})
	}
	onLogoClick(e){
		this.fire("flow-caption-logo-clicked", {e})
	}
	onLangClick(e){
		this.openI18nDialog(e.target);
	}
	openI18nDialog(target){
		FlowI18nDialog.open(target);
	}
	updateMaxIcon(){
		this.maxicon = this.window._maximize?'window-restore':'window-maximize';
		this.update();
	}
	toggleMaximize(){
		if(this.window._maximize){
			this.window.restore();
		}else{
			this.window.maximize()
		}
	}
	minimize(e) { this.window.minimize(); }
	maximize(e) { this.window.maximize(); }
	close(e) { this.window.close(); }
}

FlowCaptionBar.define('flow-caption-bar');
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_CHECKBOX_STYLED_425 : &'static str = r###"


export class FlowCheckboxStyled extends BaseElement {
	static get styles() {
		return css`
		.ext-cross:before, .checkbox__checker:before, .checkbox__cross:before, .checkbox__ok:before, .ext-cross:after, .checkbox__checker:after, .checkbox__cross:after, .checkbox__ok:after {
			content: "";
			display: block;
			position: absolute;
			width: 14px;
			height: 2px;
			margin: 0 auto;
			top: 20px;
			left: 0;
			right: 0;
			background-color: #bf1e1e;
			border-radius: 5px;
			transition-duration: .3s;
		}
		.ext-cross:before, .checkbox__checker:before, .checkbox__cross:before, .checkbox__ok:before {
			-webkit-transform: rotateZ(45deg);
			transform: rotateZ(45deg);
		}
		.ext-cross:after, .checkbox__checker:after, .checkbox__cross:after, .checkbox__ok:after {
			-webkit-transform: rotateZ(-45deg);
			transform: rotateZ(-45deg);
		}

		.ext-ok:before, .checkbox__toggle:checked + .checkbox__checker:before, .checkbox__ok:before, .ext-ok:after, .checkbox__toggle:checked + .checkbox__checker:after, .checkbox__ok:after {
			background-color: #0cb018;
		}
		.ext-ok:before, .checkbox__toggle:checked + .checkbox__checker:before, .checkbox__ok:before {
			width: 6px;
			top: 23px;
			left: -7px;
		}
		.ext-ok:after, .checkbox__toggle:checked + .checkbox__checker:after, .checkbox__ok:after {
			width: 12px;
			left: 5px;
		}

		.checkbox {
			width: 100px;
			margin: 0 auto 30px auto;
		}
		:host([slot="input"]) .checkbox{
			margin:0px;
		}
		.checkbox__container {
			display: block;
			position: relative;
			height: 42px;
			cursor: pointer;
		}
		.checkbox__toggle {
			display: none;
		}
		:host(:not(.size-animate)) .checkbox__toggle:checked + .checkbox__checker {
			left: calc(100% - 43px);
			-webkit-transform: rotateZ(360deg);
			transform: rotateZ(360deg);
		}
		:host(.size-animate) .checkbox__toggle+.checkbox__checker{
			animation:animate1 .3s;
		}
		:host(.size-animate) .checkbox__toggle:checked + .checkbox__checker {
			left: calc(100% - 43px);
			/*-webkit-transform: rotateZ(360deg);
			transform: rotateZ(360deg);*/
			animation:animate .3s;
		}
		
		@keyframes animate1{
			0%{-webkit-transform:scale(1) rotateZ(360deg);transform:scale(1) rotateZ(360deg)}
			50%{-webkit-transform:scale(0.3) rotateZ(180deg);transform:scale(0.3) rotateZ(180deg)}
			100%{-webkit-transform:scale(1) rotateZ(0deg);transform:scale(1) rotateZ(0deg)}
		}
		@keyframes animate{
			0%{-webkit-transform:scale(1) rotateZ(0deg);transform:scale(1) rotateZ(0deg)}
			50%{-webkit-transform:scale(0.3) rotateZ(180deg);transform:scale(0.3) rotateZ(180deg)}
			100%{-webkit-transform:scale(1) rotateZ(360deg);transform:scale(1) rotateZ(360deg)}
		}

		.checkbox__checker, .checkbox__cross, .checkbox__ok {
			display: block;
			position: absolute;
			height: 43px;
			width: 43px;
			top: -1px;
			left: 0px;
			z-index: 1;
		}
		.checkbox__checker {
			border-radius: 50%;
			background-color: #fff;
			box-shadow: 0px 2px 6px rgba(0, 0, 0, 0.5);
			transition: .3s;
			z-index: 2;
		}
		.checkbox__checker:before, .checkbox__checker:after {
			transition-duration: .3s;
		}
		.checkbox__cross:before, .checkbox__cross:after, .checkbox__ok:before, .checkbox__ok:after {
			background-color: #ddd;
		}
		.checkbox__ok {
			left: calc(100% - 43px);
		}
		.checkbox__txt-left, .checkbox__txt-right {
			display: block;
			position: absolute;
			width: 42px;
			top: 15px;
			text-align: center;
			color: #fff;
			font-size: 12px;
			z-index: 1;
		}
		.checkbox__txt-right {
			right: 0px;
		}
		.checkbox__bg {
			position: absolute;
			top: 0;
			left: 0;
			fill: #aaa;
			width: 100%;
			height: 100%;
		}
		`;
	}
	render(){
		return html`
		<div class="checkbox">
			<label class="checkbox__container">
				<input class="checkbox__toggle" type="checkbox" @change="${this.onChange}">
				<span class="checkbox__checker"></span>
				<span class="checkbox__cross"></span>
				<span class="checkbox__ok"></span>
				<svg class="checkbox__bg" space="preserve" __style="enable-background:new 0 0 110 43.76;" version="1.1" viewBox="0 0 110 43.76">
					<path class="shape" d="M88.256,43.76c12.188,0,21.88-9.796,21.88-21.88S100.247,0,88.256,0c-15.745,0-20.67,12.281-33.257,12.281,S38.16,0,21.731,0C9.622,0-0.149,9.796-0.149,21.88s9.672,21.88,21.88,21.88c17.519,0,20.67-13.384,33.263-13.384,S72.784,43.76,88.256,43.76z"></path>
				</svg>
			</label>
		</div>
		`;
	}
	onChange(e){
		this.fire("changed", {checked: e.target.checked})
	}
}

FlowCheckboxStyled.define('flow-checkbox-styled');
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_CHECKBOX_364 : &'static str = r###"


/**
 * @export
 * @class FlowCheckbox
 * @extends {BaseElement}
 * @prop {Boolean} checked is ckecbox checked?
 * 
 * @example
 * <flow-checkbox></flow-checkbox>
 *
 * @cssvar {color} [--flow-checkbox-color=var(--flow-border-color, rgba(0,0,0,.54))]
 * @cssvar {background-color} [--flow-checkbox-bg=var(--flow-input-bg, inherit))]
 * @cssvar {background|border-color} [--flow-checkbox-checked-color=var(--flow-border-color, var(--flow-primary-color, #3f51b5))]
 * @cssvar {background-color} [--flow-checkbox-bg=var(--flow-input-bg, inherit)]
 * @cssvar {width} [--flow-checkbox-outer-width=24px]
 * @cssvar {height} [--flow-checkbox-outer-height=24px]
 * @cssvar {margin} [--flow-checkbox-outer-margin=0px 10px 0px 0px]
 * @cssvar {border} [--flow-checkbox-outer-border=2px solid rgba(0,0,0,.54)]
 * @cssvar {border-color} [--flow-checkbox-color=var(--flow-border-color, rgba(0,0,0,.54))]
 * @cssvar {background-color} [--flow-checkbox-bg=var(--flow-input-bg, inherit)]
 * @cssvar {height} [--flow-checkbox-outline-height=100%]
 * @cssvar {width} [--flow-checkbox-outline-width=100%]
 * @cssvar {border-color} [--flow-checkbox-checked-color=var(--flow-border-color, var(--flow-primary-color, #3f51b5))]
 * @cssvar {background-color} [--flow-checkbox-checked-bg=var(--flow-input-bg, inherit)]		 
 *  
 */
export class FlowCheckbox extends BaseElement {
	static get properties() {
		return {
			checked:{type:Boolean, reflect: true},
			readonly:{type:Boolean, reflect: true},
			name:{type:String},
			inputValue:{type:String}
		}
	}
	static get styles() {
		return css`
			:host{
				display:inline-block;
				margin-right: var(--flow-checkbox-margin-right, 10px);
			}
			:host(.block){
				display: block;
				width: max-content;
				margin-bottom: var(--flow-checkbox-margin-bottom);
			}
			:host(:not([disabled]):not([readonly])) .checkbox{
				cursor:pointer;
			}
			.checkbox{
				display:flex;align-items:center;
			    user-select:none;position:relative;
			}
			.checkbox-input{
			    position:absolute;
			    opacity:0;
			    z-index:0;
				top:0px;
			}
			.checkbox-outer{
				position:relative;
			    top:0px;
			    left:0;
			    display:inline-block;
			    box-sizing:border-box;
			    width: var(--flow-checkbox-outer-width,24px);
			    height: var(--flow-checkbox-outer-height,24px);
			    margin: var(--flow-checkbox-outer-margin, 0px 10px 0px 0px);
			    cursor:pointer;
			    overflow:hidden;
			    border: var(--flow-checkbox-outer-border,2px solid rgba(0,0,0,.54));
			    border-color:var(--flow-checkbox-color, var(--flow-border-color, rgba(0,0,0,.54)));
			    border-radius:2px;
			    z-index:2;
			    background-color:var(--flow-checkbox-bg, var(--flow-input-bg, inherit));
			    transition-duration: .28s;
			    transition-timing-function: cubic-bezier(.4,0,.2,1);
			    transition-property: background;
			}
			.outline{
				position: absolute;
			    top: 0;
			    left: 0;
			    height: var(--flow-checkbox-outline-height, 100%);
			    width: var(--flow-checkbox-outline-width,100%);
			    -webkit-mask: url("data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiIHN0YW5kYWxvbmU9Im5vIj8+CjxzdmcKICAgeG1sbnM6ZGM9Imh0dHA6Ly9wdXJsLm9yZy9kYy9lbGVtZW50cy8xLjEvIgogICB4bWxuczpjYz0iaHR0cDovL2NyZWF0aXZlY29tbW9ucy5vcmcvbnMjIgogICB4bWxuczpyZGY9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkvMDIvMjItcmRmLXN5bnRheC1ucyMiCiAgIHhtbG5zOnN2Zz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciCiAgIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIKICAgdmVyc2lvbj0iMS4xIgogICB2aWV3Qm94PSIwIDAgMSAxIgogICBwcmVzZXJ2ZUFzcGVjdFJhdGlvPSJ4TWluWU1pbiBtZWV0Ij4KICA8ZGVmcz4KICAgIDxjbGlwUGF0aCBpZD0iY2xpcCI+CiAgICAgIDxwYXRoCiAgICAgICAgIGQ9Ik0gMCwwIDAsMSAxLDEgMSwwIDAsMCB6IE0gMC44NTM0Mzc1LDAuMTY3MTg3NSAwLjk1OTY4NzUsMC4yNzMxMjUgMC40MjkzNzUsMC44MDM0Mzc1IDAuMzIzMTI1LDAuOTA5Njg3NSAwLjIxNzE4NzUsMC44MDM0Mzc1IDAuMDQwMzEyNSwwLjYyNjg3NSAwLjE0NjU2MjUsMC41MjA2MjUgMC4zMjMxMjUsMC42OTc1IDAuODUzNDM3NSwwLjE2NzE4NzUgeiIKICAgICAgICAgc3R5bGU9ImZpbGw6I2ZmZmZmZjtmaWxsLW9wYWNpdHk6MTtzdHJva2U6bm9uZSIgLz4KICAgIDwvY2xpcFBhdGg+CiAgICA8bWFzayBpZD0ibWFzayIgbWFza1VuaXRzPSJvYmplY3RCb3VuZGluZ0JveCIgbWFza0NvbnRlbnRVbml0cz0ib2JqZWN0Qm91bmRpbmdCb3giPgogICAgICA8cGF0aAogICAgICAgICBkPSJNIDAsMCAwLDEgMSwxIDEsMCAwLDAgeiBNIDAuODUzNDM3NSwwLjE2NzE4NzUgMC45NTk2ODc1LDAuMjczMTI1IDAuNDI5Mzc1LDAuODAzNDM3NSAwLjMyMzEyNSwwLjkwOTY4NzUgMC4yMTcxODc1LDAuODAzNDM3NSAwLjA0MDMxMjUsMC42MjY4NzUgMC4xNDY1NjI1LDAuNTIwNjI1IDAuMzIzMTI1LDAuNjk3NSAwLjg1MzQzNzUsMC4xNjcxODc1IHoiCiAgICAgICAgIHN0eWxlPSJmaWxsOiNmZmZmZmY7ZmlsbC1vcGFjaXR5OjE7c3Ryb2tlOm5vbmUiIC8+CiAgICA8L21hc2s+CiAgPC9kZWZzPgogIDxyZWN0CiAgICAgd2lkdGg9IjEiCiAgICAgaGVpZ2h0PSIxIgogICAgIHg9IjAiCiAgICAgeT0iMCIKICAgICBjbGlwLXBhdGg9InVybCgjY2xpcCkiCiAgICAgc3R5bGU9ImZpbGw6IzAwMDAwMDtmaWxsLW9wYWNpdHk6MTtzdHJva2U6bm9uZSIgLz4KPC9zdmc+Cg==");
			    mask: url("data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiIHN0YW5kYWxvbmU9Im5vIj8+CjxzdmcKICAgeG1sbnM6ZGM9Imh0dHA6Ly9wdXJsLm9yZy9kYy9lbGVtZW50cy8xLjEvIgogICB4bWxuczpjYz0iaHR0cDovL2NyZWF0aXZlY29tbW9ucy5vcmcvbnMjIgogICB4bWxuczpyZGY9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkvMDIvMjItcmRmLXN5bnRheC1ucyMiCiAgIHhtbG5zOnN2Zz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciCiAgIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIKICAgdmVyc2lvbj0iMS4xIgogICB2aWV3Qm94PSIwIDAgMSAxIgogICBwcmVzZXJ2ZUFzcGVjdFJhdGlvPSJ4TWluWU1pbiBtZWV0Ij4KICA8ZGVmcz4KICAgIDxjbGlwUGF0aCBpZD0iY2xpcCI+CiAgICAgIDxwYXRoCiAgICAgICAgIGQ9Ik0gMCwwIDAsMSAxLDEgMSwwIDAsMCB6IE0gMC44NTM0Mzc1LDAuMTY3MTg3NSAwLjk1OTY4NzUsMC4yNzMxMjUgMC40MjkzNzUsMC44MDM0Mzc1IDAuMzIzMTI1LDAuOTA5Njg3NSAwLjIxNzE4NzUsMC44MDM0Mzc1IDAuMDQwMzEyNSwwLjYyNjg3NSAwLjE0NjU2MjUsMC41MjA2MjUgMC4zMjMxMjUsMC42OTc1IDAuODUzNDM3NSwwLjE2NzE4NzUgeiIKICAgICAgICAgc3R5bGU9ImZpbGw6I2ZmZmZmZjtmaWxsLW9wYWNpdHk6MTtzdHJva2U6bm9uZSIgLz4KICAgIDwvY2xpcFBhdGg+CiAgICA8bWFzayBpZD0ibWFzayIgbWFza1VuaXRzPSJvYmplY3RCb3VuZGluZ0JveCIgbWFza0NvbnRlbnRVbml0cz0ib2JqZWN0Qm91bmRpbmdCb3giPgogICAgICA8cGF0aAogICAgICAgICBkPSJNIDAsMCAwLDEgMSwxIDEsMCAwLDAgeiBNIDAuODUzNDM3NSwwLjE2NzE4NzUgMC45NTk2ODc1LDAuMjczMTI1IDAuNDI5Mzc1LDAuODAzNDM3NSAwLjMyMzEyNSwwLjkwOTY4NzUgMC4yMTcxODc1LDAuODAzNDM3NSAwLjA0MDMxMjUsMC42MjY4NzUgMC4xNDY1NjI1LDAuNTIwNjI1IDAuMzIzMTI1LDAuNjk3NSAwLjg1MzQzNzUsMC4xNjcxODc1IHoiCiAgICAgICAgIHN0eWxlPSJmaWxsOiNmZmZmZmY7ZmlsbC1vcGFjaXR5OjE7c3Ryb2tlOm5vbmUiIC8+CiAgICA8L21hc2s+CiAgPC9kZWZzPgogIDxyZWN0CiAgICAgd2lkdGg9IjEiCiAgICAgaGVpZ2h0PSIxIgogICAgIHg9IjAiCiAgICAgeT0iMCIKICAgICBjbGlwLXBhdGg9InVybCgjY2xpcCkiCiAgICAgc3R5bGU9ImZpbGw6IzAwMDAwMDtmaWxsLW9wYWNpdHk6MTtzdHJva2U6bm9uZSIgLz4KPC9zdmc+Cg==");
			    background: 0 0;
			    transition-duration: .28s;
			    transition-timing-function: cubic-bezier(.4,0,.2,1);
			    transition-property: background;
			}
			.checkbox-input:checked+.checkbox-outer{
				border: 2px solid #3f51b5;
				border-color:var(--flow-checkbox-checked-color, var(--flow-border-color, var(--flow-primary-color, #3f51b5)));
			}
			.checkbox-input:checked+.checkbox-outer .outline{
				background:var(--flow-checkbox-checked-color, var(--flow-border-color, var(--flow-primary-color, #3f51b5))) url("data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiIHN0YW5kYWxvbmU9Im5vIj8+CjxzdmcKICAgeG1sbnM6ZGM9Imh0dHA6Ly9wdXJsLm9yZy9kYy9lbGVtZW50cy8xLjEvIgogICB4bWxuczpjYz0iaHR0cDovL2NyZWF0aXZlY29tbW9ucy5vcmcvbnMjIgogICB4bWxuczpyZGY9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkvMDIvMjItcmRmLXN5bnRheC1ucyMiCiAgIHhtbG5zOnN2Zz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciCiAgIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIKICAgdmVyc2lvbj0iMS4xIgogICB2aWV3Qm94PSIwIDAgMSAxIgogICBwcmVzZXJ2ZUFzcGVjdFJhdGlvPSJ4TWluWU1pbiBtZWV0Ij4KICA8cGF0aAogICAgIGQ9Ik0gMC4wNDAzODA1OSwwLjYyNjc3NjcgMC4xNDY0NDY2MSwwLjUyMDcxMDY4IDAuNDI5Mjg5MzIsMC44MDM1NTMzOSAwLjMyMzIyMzMsMC45MDk2MTk0MSB6IE0gMC4yMTcxNTcyOSwwLjgwMzU1MzM5IDAuODUzNTUzMzksMC4xNjcxNTcyOSAwLjk1OTYxOTQxLDAuMjczMjIzMyAwLjMyMzIyMzMsMC45MDk2MTk0MSB6IgogICAgIGlkPSJyZWN0Mzc4MCIKICAgICBzdHlsZT0iZmlsbDojZmZmZmZmO2ZpbGwtb3BhY2l0eToxO3N0cm9rZTpub25lIiAvPgo8L3N2Zz4K");
			}
			.checkbox-input:checked+.checkbox-outer{
				background-color:var(--flow-checkbox-checked-bg, var(--flow-input-bg, inherit));
			}

		`;
	}
	render(){
		return html`
		<label class="checkbox">
			<input class="checkbox-input" type="checkbox" @change="${this.onChange}" 
				?disabled=${this.readonly} .checked="${this.checked}"
				.name="${this.name}" .value="${this.inputValue||'ON'}">
			<div class="checkbox-outer"><div class="outline"></div></div>
			<slot></slot>
		</label>
		`;
	}
	get value(){
		return !!this.checked;
	}
	set value(checked){
		this.setChecked(checked)
	}
	onChange(e){
		this.setChecked(e.target.checked);
	}
	toggle(){
		this.setChecked(!this.checked)
	}
	setChecked(checked){
		let lastChecked = this.checked;
		this.checked = !!checked;
		if(lastChecked != this.checked)
			this.fireChangeEvent();
	}
	fireChangeEvent(){
		this.fire("changed", {checked: this.checked, name:this.name}, {bubbles:true})
	}
}

FlowCheckbox.define('flow-checkbox');
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_RADIO_424 : &'static str = r###"


/**
 * @export
 * @class FlowRadio
 * @extends {BaseElement}
 * @prop {Boolean} checked is ckecbox checked?
 * 
 * @example
 * <flow-radio></flow-radio>
 *
 * @cssvar {color} [--flow-radio-color=var(--flow-border-color, rgba(0,0,0,.54))]
 * @cssvar {background-color} [--flow-radio-bg=var(--flow-input-bg, inherit))]
 * @cssvar {background|border-color} [--flow-radio-checked-color=var(--flow-border-color, var(--flow-primary-color, #3f51b5))]
 * @cssvar {background-color} [--flow-radio-bg=var(--flow-input-bg, inherit)]
 * @cssvar {width} [--flow-radio-outer-width=24px]
 * @cssvar {height} [--flow-radio-outer-height=24px]
 * @cssvar {margin} [--flow-radio-outer-margin=0px 10px 0px 0px]
 * @cssvar {border} [--flow-radio-outer-border=2px solid rgba(0,0,0,.54)]
 * @cssvar {border-color} [--flow-radio-color=var(--flow-border-color, rgba(0,0,0,.54))]
 * @cssvar {background-color} [--flow-radio-bg=var(--flow-input-bg, inherit)]
 * @cssvar {height} [--flow-radio-outline-height=100%]
 * @cssvar {width} [--flow-radio-outline-width=100%]
 * @cssvar {border-color} [--flow-radio-checked-color=var(--flow-border-color, var(--flow-primary-color, #3f51b5))]
 * @cssvar {background-color} [--flow-radio-checked-bg=var(--flow-input-bg, inherit)]		 
 *  
 */
export class FlowRadio extends BaseElement {
	static get properties() {
		return {
			checked:{type:Boolean, reflect: true},
			readonly:{type:Boolean, reflect: true},
			name:{type:String},
			inputValue:{type:String}
		}
	}
	static get styles() {
        return css`
			:host{
				display:inline-block;
				margin-right: var(--flow-radio-margin-right, 10px);
			}
			:host(.block){
				display: block;
				width: max-content;
				margin-bottom: var(--flow-radio-margin-bottom);
			}
			:host(:not([disabled]):not([readonly])) .radio{
				cursor:pointer;
			}
			.radio{
				display:flex;align-items:center;
			    user-select:none;position:relative;
			}
			.radio-input{
			    position:absolute;
			    opacity:0;
			    z-index:0;
				top:0px;
			}
			.radio-outer{
				position:relative;
			    top:0px;
			    left:0;
			    display:inline-block;
			    box-sizing:border-box;
			    width: var(--flow-radio-outer-width,24px);
			    height: var(--flow-radio-outer-height,24px);
			    margin: var(--flow-radio-outer-margin, 0px 10px 0px 0px);
			    cursor:pointer;
			    overflow:hidden;
			    border: var(--flow-radio-outer-border, 2px solid rgba(0,0,0,.54));
			    border-color:var(--flow-radio-color, var(--flow-border-color, rgba(0,0,0,.54)));
			    border-radius:50%;
			    z-index:2;
			    background-color:var(--flow-radio-bg, var(--flow-input-bg, inherit));
			    transition-duration: .28s;
			    transition-timing-function: cubic-bezier(.4,0,.2,1);
			    transition-property: background;
			}
			.outline{
				position: absolute;
			    top: 15%;
			    left: 15%;
			    height: var(--flow-radio-outline-height, 70%);
			    width: var(--flow-radio-outline-width, 70%);
                border-radius:50%;
                background: 0 0;
			    transition-duration: .28s;
			    transition-timing-function: cubic-bezier(.4,0,.2,1);
			    transition-property: background;
			}
			.radio-input:checked+.radio-outer{
				border: 2px solid #3f51b5;
				border-color:var(--flow-radio-checked-color, var(--flow-border-color, var(--flow-primary-color, #3f51b5)));
			}
			.radio-input:checked+.radio-outer .outline{
				background:var(--flow-radio-checked-color, var(--flow-border-color, var(--flow-primary-color, #3f51b5)));
			}
			.radio-input:checked+.radio-outer{
				background-color:var(--flow-radio-checked-bg, var(--flow-input-bg, inherit));
			}

		`;
	}
	constructor(){
		super();
		this.checked = false;
		this.name = this.name||"radio-"+Date.now();
	}
	render(){
        let {name} = this;
		return html`
		<label class="radio">
			<input class="radio-input" type="radio" @change="${this.onChange}" 
				?disabled=${this.readonly} ?checked="${this.checked}"
				.name="${name}" .value="${this.inputValue||'ON'}">
			<div class="radio-outer"><div class="outline"></div></div>
			<slot></slot>
		</label>
		`;
	}
	get value(){
		return !!this.checked;
	}
	set value(checked){
		this.setChecked(checked)
	}
	onChange(e){
		this.setChecked(e.target.checked);
	}
	toggle(){
		this.setChecked(!this.checked)
	}
	setChecked(checked){
		let lastChecked = this.checked;
		this.checked = !!checked;
		if(lastChecked != this.checked)
			this.fireChangeEvent();
	}
	fireChangeEvent(){
		this.fire("changed", {
			checked: this.checked,
			name:this.name,
			value:this.inputValue||'ON'
		}, {bubbles:true});

		if(this.checked)
			this.fire("flow-radio-checked", {
				name: this.name,
				el:this
			}, {}, window);
	}
	firstUpdated(...args){
		super.firstUpdated(...args);
		this.registerListener("flow-radio-checked", (e)=>{
			let {name, el} = e.detail||{};
			if(name == this.name && el!=this){
				//console.log("el, name", el, name)
				this.setChecked(false);
			}
		});
	}
	updated(updates){
		super.updated(updates);
		if (updates.has("checked")){
			this.fireChangeEvent();
		}
	}
}

FlowRadio.define('flow-radio');
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_RADIO_BTN_394 : &'static str = r###"


/**
 * @export
 * @class FlowRadioBtn
 * @extends {FlowToggleBtn}
 * @prop {Boolean} active is btn active?
 * @prop {Boolean} inputValue is value for multiple radio btn
 * 
 * @example
 * <flow-radio-btn inputvalue="xyz"></flow-radio-btn>
 * 
 */
export class FlowRadioBtn extends FlowToggleBtn {
	constructor(){
		super();
        this.radio = true;
	}
}

FlowRadioBtn.define('flow-radio-btn');
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_RADIO_BTNS_356 : &'static str = r###"


/**
 * @export
 * @class FlowRadioBtns
 * @extends {BaseElement}
 * 
 * @example
 * 
 * <flow-radio-btns selected="btn2">
 *	<flow-radio-btn inputvalue="btn1">Btn 1</flow-radio-btn>
 *	<flow-radio-btn inputvalue="btn2">Btn 2</flow-radio-btn>
 *	<flow-radio-btn inputvalue="btn3">Btn 3</flow-radio-btn>
 * </flow-radio-btns>
 *
 * 
 */
export class FlowRadioBtns extends FlowMenu {
	constructor(){
		super();
		this.valueAttr = "inputvalue";
        this.selector = "flow-radio-btn";
	}
    connectedCallback(){
        this.classList.add("flow-btn-group");
        super.connectedCallback();
    }
	updateList(){
		this.list.forEach(item=>{
			let value = item.getAttribute(this.valueAttr)
			item.setActive(this.isSelected(value));
		});
	}
}

FlowRadioBtns.define('flow-radio-btns');
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_RADIOS_373 : &'static str = r###"


/**
 * @export
 * @class FlowRadios
 * @extends {BaseElement}
 * 
 * @example
 * 
 * <flow-radios selected="btn2">
 *	<flow-radio inputvalue="btn1">Btn 1</flow-radio>
 *	<flow-radio inputvalue="btn2">Btn 2</flow-radio>
 *	<flow-radio inputvalue="btn3">Btn 3</flow-radio>
 * </flow-radios>
 *
 * 
 */
export class FlowRadios extends FlowMenu {
	constructor(){
		super();
		this.valueAttr = "inputvalue";
		this.selector = "flow-radio";
		this.name = this.name||"radio-"+Date.now();
	}
	updateList(){
		this.list.forEach(item=>{
			let value = item.getAttribute(this.valueAttr)
			item.setChecked(this.isSelected(value));
		});
	}
	onSlotChange(){
		this.list.forEach(item=>{
			if (!item.getAttribute("name")){
				item.setAttribute("name", this.name)
			}
			item.onclick = ()=>{};//<--- iphone issue
		});
		this.updateList();
	}
}

FlowRadios.define('flow-radios');
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_TOGGLE_BTN_372 : &'static str = r###"



/**
 * @export
 * @class FlowToggleBtn
 * @extends {FlowBtn}
 * @prop {Boolean} active is btn active?
 * @prop {Boolean} radio is btn a radio control?, will be effective using name property
 * @prop {Boolean} inputValue is value for multiple radio btn
 * 
 * @example
 * <flow-toggle-btn></flow-toggle-btn>
 * 
 */
export class FlowToggleBtn extends FlowBtn {
	static get properties() {
		return {
			active:{type:Boolean, reflect: true},
            radio:{type:Boolean, reflect: true},
			readonly:{type:Boolean, reflect: true},
			name:{type:String},
			inputValue:{type:String}
		}
	}
    static get styles(){
		return [FlowBtn.styles, css`
            :host([active]){
				background-color:var(--flow-btn-active-bg-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-color:var(--flow-btn-active-border-color, var(--flow-primary-color, rgba(0,151,115,1)));
				color:var(--flow-btn-active-invert-color, var(--flow-primary-invert-color, #FFF));
				--fa-icon-color:var(--flow-btn-active-invert-color, var(--flow-primary-invert-color, #FFF));
			}
			:host([active]:not([disabled]):hover){
				background-color:var(--flow-btn-hover-active-bg-color, var(--flow-btn-hover-border-color, var(--flow-primary-color, rgba(0,151,115,1))));
				border-color:var(--flow-btn-hover-active-border-color, var(--flow-btn-hover-border-color, var(--flow-primary-color, rgba(0,151,115,1))));
                color: var(--flow-btn-hover-active-color, var(--flow-btn-hover-color, inherit));
			}
			:host([active][radio]){
				cursor:default;
			}
        `]
    }
	constructor(){
		super();
		this.active = false;
	}
	get value(){
		return !!this.active;
	}
	set value(active){
		this.setActive(active)
	}
	click(e){
        super.click(e);
		this.toggle();
	}
	toggle(){
        if(this.radio && this.active)//radio btn cant be deactivated
            return
		this.setActive(!this.active)
	}
	setActive(active){
		let lastActive = this.active;
		this.active = !!active;
		if(lastActive != this.active)
			this.fireChangeEvent();
	}
	fireChangeEvent(){
		this.fire("changed", {
			active: this.active,
			name:this.name,
			value:this.inputValue||'ON'
		}, {bubbles:true});

		if(this.active && this.radio)
			this.fire("flow-toogle-btn-active", {
				name: this.name,
				el:this
			}, {}, window);
	}
	firstUpdated(...args){
		super.firstUpdated(...args);
        if(this.radio){
            this.registerListener("flow-toogle-btn-active", (e)=>{
                let {name, el} = e.detail||{};
                if(name == this.name && el!=this){
                    //console.log("el, name", el, name)
                    this.setActive(false);
                }
            });
        }
	}
}

FlowToggleBtn.define('flow-toggle-btn');
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_BTN_GROUP_412 : &'static str = r###"


/**
 * @export
 * @class FlowBtnGroup
 * @extends {BaseElement}
 * 
 * @example
 * 
 * <flow-btn-group>
 *	<flow-btn>Btn 1</flow-btn>
 *	<flow-btn>Btn 2</flow-btn>
 *	<flow-btn>Btn 3</flow-btn>
 * </flow-btn-group>

 * <flow-btn-group vertical>
 *	<flow-btn>Btn 1</flow-btn>
 *	<flow-btn>Btn 2</flow-btn>
 *	<flow-btn>Btn 3</flow-btn>
 * </flow-btn-group>
 *
 * 
 */
export class FlowBtnGroup extends BaseElement {
	createRenderRoot(){
		return this;
	}
}

FlowBtnGroup.define('flow-btn-group');
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_CIRCULAR_SHAPES_397 : &'static str = r###"


export class FlowCircularShapes extends BaseElement {
	constructor() {
		super();
		this.ident = Math.round((Math.random()*1e16)).toString(16);
		this.template = document.createElement('template');
	}

	generate() {
		this.targets = [...this.querySelectorAll('.square')];

		let rects = this.targets.map(t => t.getBoundingClientRect());
		if(!rects.length)
			return;
		let rect = { left : rects[0].left, top : rects[0].top, right : rects[0].right, bottom : rects[0].bottom  };
		rects.forEach(r => {
			if(r.left < rect.left)
				rect.left = r.left;
			if(r.top < rect.top)
				rect.top = r.top;
			if(r.right > rect.right)
				rect.right = r.right;
			if(r.bottom > rect.bottom)
				rect.bottom = r.bottom;
		})

		//console.log("SHAPES RECT:",this.targets,rects,rect)

		let path = '';

		let svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${this.width} ${this.height}">
			<rect x="${rect.left}" y="${rect.top}" width="${rect.right-rect.left}" height="${rect.bottom-rect.top}"
				fill="none" stroke="red" />
				<!--text x="0" y="0">
				HELLO WORLD
				</text-->
				${path}
			</svg>`;

		this.template.innerHTML = svg;
		this.svg = this.template.content.firstChild.cloneNode(true);
	}

	static get styles() {
		return css`
			.wrapper {
				position: relative;
				overflow:hidden;
			}

			.container {
				position: absolute;
				left : 0;
				top : 0;
			}

			.svg {
				position: absolute;
				left: 0;
				top: 0;
			}
		`;
	}
	render() {
		this.generate();
		return html`
			<style>
				.wrapper {
					height: ${this.height || 0};
				}
			</style>
			<div class="wrapper">
				<div class="container">
				</div>
				<div class="svg">
					${this.svg || ''}
				</div>
			</div>
		`;
	}

	firstUpdated() {
		this.generate();
		this.requestUpdate();
	}
}

FlowCircularShapes.define('flow-circular-shapes');
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_FORM_REGION_429 : &'static str = r###"


export class FlowFormRegion extends BaseElement {

	static get styles() {
		return css`
		`;
	}

	render() {
		return html`
			<div class='region'>
			icon
			field
			question region
			</div>
		`;
	}
}

FlowFormRegion.define('flow-form-region');
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_SVG_TEST_433 : &'static str = r###"


export class FlowSVGTest extends BaseElement {

	static get properties() {
		return {
			checked : { type : Boolean }
		}
	}
	static get styles() {
		return css`
			:host{display:inline-block}
		`
	}

	constructor() {
		super();
		let template = document.createElement('template');

		this.ident = 'path-'+Math.round((Math.random()*1e16)).toString(16);
		//html = html.trim(); // Never return a text node of whitespace as the result
		template.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 250 250">
			<path
				style="fill:none;stroke:#000000;stroke-width:20px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
				d="M 157.04983,38.414322 H 2.3523133 V 239.82002 H 203.01068 V 84.748842"
				id="path3748"
				inkscape:connector-curvature="0" />

			<path  id="${this.ident}" 
				style="fill:none;stroke:#000000;stroke-width:20px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
				d="M 40.839859,122.86272 83.81139,165.83425 247.66371,1.9819364"
				id="path3750"
				inkscape:connector-curvature="0" />
		</svg>`;
		// <path id="${this.ident}" fill="#FFFFFF" stroke="#000000" stroke-width="4" stroke-miterlimit="10" d="M66.039,133.545c0,0-21-57,18-67s49-4,65,8
		//    s30,41,53,27s66,4,58,32s-5,44,18,57s22,46,0,45s-54-40-68-16s-40,88-83,48s11-61-11-80s-79-7-70-41
		//    C46.039,146.545,53.039,128.545,66.039,133.545z"/>
		//<path d="M256 8C119.033 8 8 119.033 8 256s111.033 248 248 248 248-111.033 248-248S392.967 8 256 8zm0 48c110.532 0 200 89.451 200 200 0 110.532-89.451 200-200 200-110.532 0-200-89.451-200-200 0-110.532 89.451-200 200-200m140.204 130.267l-22.536-22.718c-4.667-4.705-12.265-4.736-16.97-.068L215.346 303.697l-59.792-60.277c-4.667-4.705-12.265-4.736-16.97-.069l-22.719 22.536c-4.705 4.667-4.736 12.265-.068 16.971l90.781 91.516c4.667 4.705 12.265 4.736 16.97.068l172.589-171.204c4.704-4.668 4.734-12.266.067-16.971z"/>
		//	    return template.content.childNodes;//firstChild.cloneNode(true);

		//		this.viewBox = template.content.
		// <path id="${this.ident}" xfill="#FFFFFF" stroke="#000000" stroke-width="4" stroke-miterlimit="10" d="M43.542,8.812c-0.465-0.3-1.083-0.166-1.383,0.298L21.718,40.816L7.671,28.11c-0.41-0.372-1.042-0.339-1.413,0.07
		// 	c-0.371,0.41-0.339,1.042,0.071,1.412L21.25,43.089c0.185,0.168,0.424,0.259,0.671,0.259c0.043,0,0.087-0.003,0.131-0.009
		// 	c0.291-0.038,0.551-0.203,0.709-0.449l21.08-32.696C44.14,9.729,44.006,9.111,43.542,8.812z"/>

		this.svg = template.content.firstChild.cloneNode(true);
		this.svg.style.overflow = 'visible';

		this.viewBox = this.svg.getAttribute("viewBox");

		this.path = this.svg.querySelector("path");
		this.length = this.path.getTotalLength();
		//console.log("PATH:",this.path,this.length);

		//this.path.style.strokeDashoffset = this.length*2;

		//this.segment = new Segment(this.path);

	}

	createRenderRoot() {
		return this;
	}

	render() {
		let style = this.checked ? 
			html`
				#${this.ident} {
					stroke-dasharray: ${this.length};
					stroke-dashoffset: ${this.length};
					animation: dash 0.4s ease-in-out forwards;
					display: block;
				}

			`
			:
			html`
				#${this.ident} {
					/*transform: scale(2,2) translate(0,-5%);*/
					transform-origin:center;
					opacity : 0.0;
				}
			`;

		return html`

		<style>
		.wrapper {
			padding: 4px;
			/*border:1px solid red;*/
		}
		#${this.ident} {
			transition: all 0.2s ease;
			trantition-property: transform, opacity;
		}
		${style}
		@keyframes dash {
			to {
				stroke-dashoffset: 0;
			}
		}		
		</style>

		<span class="wrapper" @click=${this.toggle}>
		${this.svg}
		</span>`;

		//<svg viewBox="${this.viewBox}">${this.path}</svg>`;

		/*
		return html`

		<style>
			svg {
				width : auto;
				height : 100%;
				fill:red;
			}
		</style>

		<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
		<path d="M256 8C119.033 8 8 119.033 8 256s111.033 248 248 248 248-111.033 248-248S392.967 8 256 8zm0 48c110.532 0 200 89.451 200 200 0 110.532-89.451 200-200 200-110.532 0-200-89.451-200-200 0-110.532 89.451-200 200-200m140.204 130.267l-22.536-22.718c-4.667-4.705-12.265-4.736-16.97-.068L215.346 303.697l-59.792-60.277c-4.667-4.705-12.265-4.736-16.97-.069l-22.719 22.536c-4.705 4.667-4.736 12.265-.068 16.971l90.781 91.516c4.667 4.705 12.265 4.736 16.97.068l172.589-171.204c4.704-4.668 4.734-12.266.067-16.971z"/></svg>

		`;

		*/		
	}

	toggle() {
		this.checked = !this.checked;
		console.log("checked:",this.checked);
	}

	updated() {/*
		console.log("DRAWING SEGMENT",this.innerHTML);
				let path = this.querySelector("path");
				console.log("path:",path)
				 let segment = new Segment(path);

				segment.draw("25%","75% - 10", 5);
				*/
		//this.segment.draw("20%","70% - 10", 5);
	}
}

FlowSVGTest.define('flow-svg-test')
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_TAB_347 : &'static str = r###"

/**
* @typedef {Object} FlowTabConfig Plain Object with following properties
* @prop {String} title text to display inside of tab
* @prop {String} id tab id
*/

/**
* @class FlowTab
* @extends BaseElement
* @export FlowTab
*
* @prop {Boolean} active is tab active?
*
* @cssvar {font-family} [--flow-tab-font-family=var(--flow-font-family, "Julius Sans One")]
* @cssvar {font-family} [--flow-tab-font-weight=var(--flow-font-weight, bold)]
* @cssvar {font-family} [--flow-tab-font-size=var(--flow-font-size, 1.5rem)]
* @cssvar {color} [--flow-primary-color=rgba(0,151,115,1)]
* @cssvar {stroke} [--flow-tab-border-color=var(--flow-border-color, --flow-primary-color)]
* @cssvar {css-unit} [--flow-tab-wrapper-max-width=150px]
* @cssvar {max-width} [--flow-tab-wrapper-max-width=150px]
*
* @example
*   <flow-tab id="tab1">TAB 1</flow-tab>
*
*/

export class FlowTab extends BaseElement {
	static get properties() {
		return {
			active : { type : Boolean, reflect:true },
			//handler : { type : Object }
			//caption : { type : String }
		}
	}

	static get styles(){
		return css`
		:host{
			display:flex;
			align-items:center;
			position:relative;
			pointer-events:none;
			color: var(--flow-primary-color, rgba(0,151,115,1.0));
			font-family: var(--flow-tab-font-family, var(--flow-font-family, "Julius Sans One"));
			font-weight: var(--flow-tab-font-weight, var(--flow-font-weight, bold));
			font-size: var(--flow-tab-font-size, var(--flow-font-size, 1.5rem));
		}
		:host(:not([disabled])){
			cursor:pointer;
		}
		.wrapper {
			padding:0px 40px;
			position: relative;
			pointer-events:none;
			display:flex;
			flex-direction:column;
			align-items:center;
			white-space: nowrap;
		    max-width: var(--flow-tab-wrapper-max-width, 150px);
		    overflow: hidden;
		    text-overflow: ellipsis;
		}
		.text{
			max-width:100%;box-sizing:border-box;overflow: hidden;
		    text-overflow: ellipsis;
		}
		.background {
			position:absolute;
			left: 0px;
			top: 0px;
			width:100%;
			height:100%;
			pointer-events:none;
		}

		svg {
			position: absolute;
			left: 0px;
			top: 0px;
			z-index:0;
			pointer-events:none;
			overflow:visible;
		}

		path {
			pointer-events:auto;
			-webkit-app-region:no-drag;
			stroke: var(--flow-tab-border-color, var(--flow-border-color, --flow-primary-color));
		}
		`;
	}

	constructor() {
		super();
		this.caption = this.firstChild ? this.firstChild.cloneNode(true) : '';
		this.ident = Math.round((Math.random()*1e16)).toString(16);
		//this.template = document.createElement('template');
	}

	generate() {
		let rect = this.getBoundingClientRect();
		if(!rect.width)
			return '';

		let width = rect.width;
		let height = rect.height;

		let margin = 50;

		/*let path = `<path id="path-${this.ident}"
   			style="fill:url(#gradient-${this.ident});stroke-width:2px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
   			d="`; //"// 0,${height} `;*/

   		let path = '';

   		if(window.flowConfig && window.flowConfig.flowTab){
			let atanRange = 20;
			let atanMax = Math.atan(atanRange*0.5);
			let minY = ( 1.0 - (Math.atan(atanRange * 0.5) + atanMax) / (atanMax*2)) * (height-1);
			for(let x = 0; x < width; x++) {
				let y = 0;
				if(x < margin * 1) {
					let v = (x / margin - 0.5) * atanRange;
					y = ( 1.0 - (Math.atan(v) + atanMax) / (atanMax*2)) * (height-1);// * 1.05 - 5;// - 0.75;
				}
				else
				if(x > width-margin*1) {
					let v = ((width - x) / margin - 0.5) * atanRange;
					y = ( 1.0 - (Math.atan(v) + atanMax) / (atanMax*2)) * (height-1);// * 1.05 - 5;// - 0.75;
				}
				else {
					y = minY;
				}

				if(x == 0) {
					path += `M -3480,${height+1} -3480,${y} `;
				}else if(x > width-2) {
					path += `L 3480 ${y} L 3480 ${height+1} `;
				}else{
					path += `L ${x} ${y} `;
				}
			}
		}else{
			//margin = 20;//margin*0.1;
			let y = height;
			let halfM = margin/2;
			path += `M -3480,${y+40}`;
			path += `L -3480 ${y}`;
			path += `L 0,${y}`;
			path += `C ${halfM},${y} 0,0 ${halfM},0`;
			path += `L ${width-halfM},0`;
			path += `C ${width},0 ${width-halfM},${y} ${width},${y}`;
			path += `L 3480 ${y} L 3480 ${y+40}`;
		}

		//path += `z`;
		//path += `" />`;//"

		let color = {
			top : 'var(--flow-tab-bg-top, #fefefe)',
			bottom : 'var(--flow-tab-bg-bottom, #EEE)'
		}

		if(this.active) {
			color.top = 'var(--flow-tab-active-bg-top, #fefefe)';
			color.bottom = 'var(--flow-tab-active-bg-bottom, #fff)';
		}

		//html = html.trim(); // Never return a text node of whitespace as the result
		return svg`<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${width} ${height}">
		<linearGradient id="gradient-${this.ident}"  x1="0" x2="0" y1="0" y2="1">
			<stop class="stop1-${this.ident}" offset="0%" style="stop-color: ${color.top}"/>
			<stop class="stop3-${this.ident}" offset="100%" style="stop-color: ${color.bottom}"/>
		</linearGradient>
		<path id="path-${this.ident}"
   			style="fill:url(#gradient-${this.ident});stroke-width:2px;
   			stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
   			d="${path}" />
		</svg>`;

		//let template = document.createElement('template');
		//template.innerHTML = svg;

		//this.svg = template.content.firstChild.cloneNode(true);

		//this.svgViewBox = this.svg.getAttribute("viewBox");
		//this.svgPath = this.svg.querySelector("path");
		//this.svgPathLength = this.svgPath.getTotalLength();
	}

	render() {
		let svg = this.generate();

		return html`
		<div class='background' @click=${this.click}>${svg}</div>
		<div class="wrapper"><div class="text"><slot></slot></div></div>
		`;
	}

	click() {
		let event = new CustomEvent('flow-tab-select', {
			detail : this,
			bubbles : true
		});
		this.dispatchEvent(event);
	}

	onWindowResize(){
		if(this.isConnected){
    		this.requestUpdate();
		}
    }

	connectedCallback(){
    	super.connectedCallback();
    	//this._onWindowResize = this._onWindowResize || this.onWindowResize.bind(this);
		//window.addEventListener("resize", this._onWindowResize)

    	if(!this.__resizeObserver){
    		//this._onWindowResize = this._onWindowResize || this.onWindowResize.bind(this);
    		//window.__resizeObservers = window.__resizeObservers || {}; 
    		//window.__resizeObservers[this.ident] = this._onWindowResize;
	    	this.__resizeObserver = new ResizeObserver(()=>{
	    		this.onWindowResize();
			});
			this.__resizeObserver.observe(this);
	    }
    }

    disconnectedCallback() {
		if(this.__resizeObserver){
			this.__resizeObserver.unobserve(this);
			this.__resizeObserver.disconnect();
			this.__resizeObserver = null;
			//observer.observe(document.createElement("div"));
		}
		//parts.delete(this.renderRoot)
		super.disconnectedCallback();
		
		//if(this._onWindowResize)
		//	window.removeEventListener("resize", this._onWindowResize)
	}
}

FlowTab.define('flow-tab');
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_TABS_421 : &'static str = r###"




/**
* @class FlowTabs
* @extends BaseElement
* @listens flow-tab-select
* @export FlowTabs
*
* @prop {String} active Active tab id
* @prop {Array.<FlowTabConfig>} tabs alternative to slotted tabs for defining tabs
* @prop {ActiveDisplay} target-display *blocked/active* display type for target elements (default: block)
*
* @cssvar {margin} [--flow-tabs-margin=8px 28px 0px 28px]
*
* @example
* <flow-tabs active="tab1">
*   <flow-tab id="tab1">TAB 1</flow-tab>
*   <flow-tab id="tab2">TAB 2</flow-tab>
*   <flow-tab id="tab3">TAB 3</flow-tab>
*   <flow-tab id="tab4">TAB 4</flow-tab>
* </flow-tabs>
*/
export class FlowTabs extends BaseElement {
	/**
	* @member {String} active Active tab id
	* @memberof FlowTabs#
	*/

	/**
	* @member {Array.<FlowTabConfig>} tabs alternative to slotted tabs for defining tabs
	* @memberof FlowTabs#
	*/

	/**
	* @member {ActiveDisplay} target-display *blocked/active* display type for target elements (default: block)
	* @memberof FlowTabs#
	*/


	static get properties() {
		return {
			active : { type : String },
			tabs : { type : Array },
			'target-display':{type: String, value:"block"}
		}
	}
	

	/**
	* [CSSResult]{@link https://lit-element.polymer-project.org/api/classes/_lib_css_tag_.cssresult.html} object
	*/
	static get styles() {
		return css`
			:host{
				flex:1;
				display:flex;
				flex-direction:row;
				flex-wrap:wrap;
				justify-content:flex-end;
				margin:var(--flow-tabs-margin, 8px 28px 0px 28px);
				-webkit-app-region:drag;
			}
			.tabs-outer{
				width:100%;
				display:flex;
				position:relative;
			}
			.tab-items{
				flex:1;
				display:flex;
				flex-direction:row;
				flex-wrap:wrap;
				justify-content:flex-end;
			}
			.proxy{
				position:absolute;
				left:0px;
				top:0px;
				width:100%;
				visibility: hidden;
			}
			.proxy.v{
				visibility:visible;
				z-index:100;
				opacity:1
			}
			flow-tab, ::slotted(flow-tab) {
				/*//min-width: var(--flow-tab-min-width);
				//max-width: var(--flow-tab-max-width);
				//width: var(--flow-tab-width);
				//min-width: fill-available;*/
				height: 38px;
				margin-left: -20px;
				margin-right: -20px;
				margin-top:-8px;
				-webkit-app-region:no-drag;
				
			}
			.line-break{
				width:100%;
				background-color:#F00;
			    padding:0px;
			    border:0px;
			    height:0px;
			}
			flow-tab[hidden]{display:none}
		`;
	}

	constructor() {
		super();
		this.ident = Math.round((Math.random()*1e16)).toString(16);
		this.tabs = null;
		this['target-display'] = "block";
	}

	onWindowResize(){
		if(this._timeoutId)
			clearTimeout(this._timeoutId);
		this._timeoutId = setTimeout(()=>{
			this._timeoutId = null
			if(this.active)
				this.updateRows();
		}, 100);
	}

	render() {
		let tabs = this._tabs || this.tabs || [];
		return html`
		<slot></slot>
		<div class="tabs-outer">
			<div class="tab-items proxy">
			${
				(this.tabs || []).filter(t => !t.disable).map(t=>{
					if(t.sep)
						return html`<div class="line-break"></div>`;
					return html`<flow-tab data-id="${t.id}" class="${t.cls||''}" part="${t.part||''}">${this.renderTab(t)}</flow-tab>`
				})
			}
			</div>
			<div class="tab-items front">
			${
				tabs.map(t=>{
					if(t.sep)
						return html`<div class="line-break"></div>`;
					if(t.disable)
						return html`<flow-tab id='${t.id}' hidden="1"></flow-tab>`;
					return html`<flow-tab id='${t.id}' class="${t.cls||''}" part="${t.part||''}"
						style="z-index:${t.zIndex || (t.id==this.active? 2:1)}">${this.renderTab(t)}</flow-tab>`
				})
			}
			</div>
		</div>
		`;
	}
	updated(changed) {
		//this.log("UPDATING", this.active, changed)
		if(changed.has("tabs") && this._tabs){
			let lastHash = this.lastTabHash;
			let tabs = this.tabs.map(t=>{
				return Object.assign({}, t, {zIndex:undefined});
			})
			this.lastTabHash = JSON.stringify(tabs)
			if(lastHash != this.lastTabHash){
				//this.log("updated:\n", lastHash+"\n-----------\n", this.lastTabHash)
				this._tabs = null;
				this._lastHash = null;
				return this.requestUpdate("tabs");
			}
		}
		//let root = this.tabs ? this.shadowRoot : this;
		let root = this.tabs? this.shadowRoot.querySelector(".tab-items.front"):this;
		if(!root)
			return

		let tabs = [...root.querySelectorAll("flow-tab")];

		//console.log("tabs", tabs, this.tabs, this._tabs)

		let i = tabs.length;
		if(!i){
			this.log("Tab not found")
			return false;
		}

		let activeIsSet = false;
		let activeIndex = -1;
		tabs.forEach((tab, index) => {
			let target = document.querySelector(`tab-content[for="${tab.id}"]`);
			if(!tab.getAttribute('hidden') && tab.id == this.active) {
				tab.active = true;
				activeIsSet = true;
				activeIndex = index;
				//tab.style['z-index'] = 2;
				if(target){
					target.style.display = target.getAttribute('data-active-display') || this['target-display'];
				}
			}
			else {
				tab.active = false;
				//tab.style['z-index'] = 1;
				target && (target.style.display = 'none');
			}
		});

		if(!this.tabs){
			let zIndex = activeIndex>-1 ? tabs.slice(activeIndex).length+2 : 2;
			tabs.forEach((t, i)=>{
				if(activeIndex >-1 && i >= activeIndex){
					t.style['z-index'] = zIndex--;
				}else{
					t.style['z-index'] = 1;
				}
			})
		}


		if(!activeIsSet)
			this.active = tabs[0].id;

		//console.log("changed", changed)

		//if(changed.has("tabs") || changed.has("active")){
			/*
			this.addFillerTabs();
			setTimeout(()=>{
				this.addFillerTabs();
			}, 10)
			this.changeTabRows(activeTab);
			if(changed.get("active") == 'undefined'){
				setTimeout(()=>{
					this.changeTabRows();
				}, 15)
			}
			*/
			//console.log("updateRows", this.active)
			if(this.active && this.tabs){
				this.updateRows();
				setTimeout(()=>{
					this.updateRows();
					setTimeout(()=>{
						this.updateRows();
					}, 1000)
				}, 15)
			}
		//}
	}

	hashTabs(tabs){
		return JSON.stringify(tabs);
	}

	updateRows(){
		let container = this.tabs ? this.shadowRoot : this;
		container = container.querySelector(".tab-items.proxy");
		let activeTab = container && container.querySelector(`[data-id=${this.active}]`);
		if(!activeTab){
			this.log("activeTab is null", activeTab)
			return
		}
		let offsetTop = activeTab.offsetTop;
		let bottomOffset = container.querySelector("flow-tab:last-child").offsetTop;
		let map = {};

		this.tabs.forEach(t=>{
			map[t.id] = t;
		})

		let tabs = [];
		let lastRowTabs = [];
		let lastTop = -1, id;
		container.querySelectorAll("flow-tab").forEach((tab, index)=>{
			id = tab.getAttribute("data-id");
			//this.log(tab.id, tab.offsetTop, offsetTop, bottomOffset)
			if(tab.offsetTop == offsetTop && tab.offsetTop != bottomOffset){
				lastRowTabs.push(map[id]);
				return true;
			}

			if(lastTop != -1 && tab.offsetTop != lastTop){
				tabs.push({sep:1, id:"sep"});
			}
			tabs.push(map[id]);

			lastTop = tab.offsetTop;
		});

		if(lastRowTabs.length)
			lastRowTabs.unshift({sep:1, id:"sep"})

		let _tabs = [...tabs, ...lastRowTabs];
		let index = _tabs.findIndex(t=>t.id == this.active);
		let zIndex = index>-1 ? _tabs.slice(index).length+2 : 2;
		_tabs.forEach((t, i)=>{
			if(index >-1 && i >= index){
				t.zIndex = zIndex--;
			}else{
				t.zIndex = 1;
			}
		})
		let hash1 =  this.hashTabs(_tabs);
		//let hash2 =  this.hashTabs(this.tabs);

		//this.log("updateRows:_tabs\n", _tabs.map(t=>t.id+"::"+t.zIndex).join("\n"))

		
		if(/*hash1 != hash2 ||*/this._lastHash != hash1){
			//this.log("updateRows\n", this._lastHash+"\n----\n", hash1+"\n")
			this._lastHash = hash1;
			this._tabs = _tabs;
			this.requestUpdate("_tabs")
		}else{
			//this.log('updateRows:hash1==hash2', hash1)
		}
	}

	changeTabRows(activeTab){
		let container = this.tabs ? this.shadowRoot : this;
		activeTab = activeTab || container.querySelector("flow-tab[active]");
		if(!activeTab)
			return
		let offsetTop = activeTab.offsetTop;
		let bottomOffset = container.querySelector("flow-tab:last-child").offsetTop
		let tabs = [];
		let lastRowTabs = [];
		Array.from(container.querySelectorAll("flow-tab")).filter(tab=>{
			//console.log(tab.id, tab.offsetTop, offsetTop, bottomOffset)
			if(tab.offsetTop == offsetTop && tab.offsetTop != bottomOffset){
				lastRowTabs.push(tab);
				return true;
			}
			tabs.push(tab);
		}).map(tab=>{
			tab.remove();
			return tab;
		}).forEach(tab=>{
			container.append(tab);
		})
		/*
		let _tabs = [...tabs, ...lastRowTabs];
		let tabStr = _tabs.map(t=>t.id||"filler").join(",");
		if(this.tabStr == tabStr)
			return
		this.tabStr = tabStr;
		this._tabs = _tabs;
		*/
	}

	addFillerTabs() {
		window.xxxxflowTabs = this;
		let container = this.tabs ? this.shadowRoot : this;
		container.querySelectorAll(".filler-tab").forEach(e=>{
			e.remove();
		});
	
		let lastTop = -1, fillerTab, fillerTabW;
		let isEndJustified = true;
		let minTabWidth = 50;
		container.querySelectorAll("flow-tab").forEach((e, i)=>{
			//console.log("e.offsetTop, lastTop", e.offsetTop, lastTop)
			if(e.offsetTop == lastTop)
				return
			if((isEndJustified && i==0) || lastTop !== -1){
				fillerTab = document.createElement("flow-tab");
				fillerTab.className = 'filler-tab';
				fillerTab.style.backgroundColor = `rgb(${lastTop}, ${lastTop}, ${lastTop})`;
				container.insertBefore(fillerTab, e);
				fillerTab.style.minWidth = minTabWidth+"px";
				fillerTabW = fillerTab.getBoundingClientRect().width;
				fillerTab.style.minWidth = Math.max(fillerTabW, minTabWidth)+"px";
				//console.log("fillerTab.offsetTop != lastTop", fillerTab.offsetTop , lastTop)
				//console.log("fillerTab.width:", fillerTabW, fillerTab.getBoundingClientRect().width, Math.max(fillerTabW, minTabWidth))
				if(isEndJustified){
					if(fillerTab.offsetTop != e.offsetTop){
						//console.log("fillerTab.remove", fillerTab.offsetTop, e.offsetTop)
						fillerTab.remove();
					}
				}else if(fillerTab.offsetTop != lastTop)
					fillerTab.remove();
			}
			lastTop = e.offsetTop;
		})
		if(!isEndJustified){
			fillerTab = document.createElement("flow-tab");
			fillerTab.className = 'filler-tab';
			container.append(fillerTab);
			fillerTab.style.minWidth = minTabWidth+"px";
			fillerTab.style.minWidth = Math.max(fillerTab.getBoundingClientRect().width, minTabWidth)+"px";
			//console.log("fillerTab.offsetTop != lastTop", fillerTab.offsetTop , lastTop)
			if(fillerTab.offsetTop != lastTop)
				fillerTab.remove();
		}

		/*
		container.querySelectorAll(".filler-tab").forEach((e, i)=>{
			//e.style.minWidth = e.getBoundingClientRect().width+"px";
			//e.classList.remove("flex")
		})
		*/
	}

	renderTab(t) {
		let text = t.render ? t.render() : t.html||t.title||t.caption;
		if(typeof(text) == 'string'){
			//this.log("t", t.id, text)
			return html`<flow-i18n .text="${text}" text2="${text}"></flow-i18n>`;
		}
		return text;
	}

	connectedCallback(){
    	super.connectedCallback();
    	this._onWindowResize = this._onWindowResize || this.onWindowResize.bind(this);
		window.addEventListener("resize", this._onWindowResize)
		//console.log("this.renderRoot", this.renderRoot)
		this.renderRoot.addEventListener('flow-tab-select', (e) => {
			//console.log("eee", e)
			this.active = e.detail.id;
		})
    }

    disconnectedCallback() {
		super.disconnectedCallback();
		if(this._onWindowResize)
			window.removeEventListener("resize", this._onWindowResize)
		if(this._timeoutId){
			clearTimeout(this._timeoutId);
		}
	}
}

FlowTabs.define('flow-tabs');
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_TEST_ELEMENT_393 : &'static str = r###"


export class FlowTestElement extends BaseElement {

	static get properties() {
		return {
			mood: {type: String}
		}
	}

	static get styles() {
		return css`.mood { color: green; }`;
	}

	constructor() {
		super();

		this.content = "123";

		// let slots = this.shadowRoot.querySelectorAll("slot");
		// slots[1].addEventListener('slotchange', function(e) {
		//   let nodes = slots[1].assignedNodes();
		//   console.log('Element in Slot "' + slots[1].name + '" changed to "' + nodes[0].outerHTML + '".');
		// });		
	}

	createRenderRoot() {
		// console.log("S1",this.querySelector("slot"));
		// console.log(this.innerHTML);

		this.tpl_ = this.innerHTML;

		// this.display_ = this.style.display;

		// this.style.display = "none";

		return this;

		let root;

		// can't use querySelector because it would break with nesting because we don't
		// have shadow DOM boundaries anymore. We don't know what's the content
		// of this element, vs the content of child elements. 🤷‍♂️

		// for (const child of this.childElements) {
		// 	if (child.matches('div.content') {
		// 		root = child;
		// 		break;
		// 	}
		// }

		if (root === undefined) {
			//root = document.createTextNode('');
			root = document.createElement('template');
			//root.className = 'i18n';
			this.appendChild(root);

			// https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver
			/*let observer = new MutationObserver((mutationList, observer) => {
				console.log('A child node has been added or removed.',root.innerHTML);


				let slots = this.querySelectorAll("slot");
				observer.observe(slots[0],{childList: true});


			})

			observer.observe(root, { childList : true })
			*/

			/*
				let slots = this.querySelectorAll("slot");
				//let nodes = slot[0].assignedNodes();
				//console.log("slot nodes",nodes)
				slots[0].addEventListener('slotchange', function(e) {
				  let nodes = slots[1].assignedNodes();
				  console.log('Element in Slot "' + slots[1].name + '" changed to "' + nodes[0].outerHTML + '".');
				});		
			*/
		}

		return root;
	}

	htmlToElement(html) {
		let template = document.createElement('template');
		html = html.trim(); // Never return a text node of whitespace as the result
		template.innerHTML = `<span>${html}</span>`;
		//return template.content.childNodes;//firstChild.cloneNode(true);
		return template.content.firstChild.cloneNode(true);
	}


	render() {
		//return html`YAY`;
		//console.log("THIS:",this);
		//return html`Web Components are <span class="mood">${this.mood}</span>!`;
		//return html`${this.tpl_}`;//<slot></slot>`;
		//let H = this._T(this.tpl_);//<slot></slot>`;
		//console.log("RETURNING:",H)
		let H = this._T(this.tpl_);
		//console.log("H:",H);
		return html`${H}`;
		//return H;//html`${H}`;
	}  

	afterRender() {
		//console.log("AFTER RENDER")
	}

	updated() {
		//console.log("UPDATED!", this.innerHTML);
		//this.style.display = this.display_;
		//this.innerHTML = this.innerHTML.replace("WORKED","HELLO")
	}

	_T(html) {
		//console.log("replacing",html);
		html = html.replace("WORKED","W_O_R_K_E_D");
		return this.htmlToElement(html);
		//return this.htmlToElement("HELLO <i>WORLD</i> W<b>W</b>WW");
	}
}

FlowTestElement.define('flow-test-element');

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_TEXT_AREA_354 : &'static str = r###"



/**
 * @export
 * @class FlowTextArea
 * @extends {BaseElement}
 * 
 * @property {Number} [height] height in number
 * @property {Boolean} [autocomplete=false] textarea attribute autocomplete
 * @property {Boolean} [autofocus=false] set true to autofocus textarea
 * @property {Boolean} [autosize=false] set true to autosize textarea
 * @property {Boolean} [disabled=false] set true to disable textarea
 * @property {String} [placeholder=''] textarea placeholder text
 * @property {Boolean} [readonly=false] set true to make textarea as readonly
 * @property {Number} [maxlength] set maxlength of textarea
 * 
 * @example
 * <flow-textarea></flow-textarea>
 *
 * @cssvar {border} [--flow-border-color=var(--flow-primary-color, #3f51b5)]
 * @cssvar {background} [--flow-textarea-bg=var(--flow-input-bg, #fafafa)]
 * @cssvar {color} [--flow-textarea-color=var(--flow-input-color, inherit)]
 */
export class FlowTextArea extends BaseElement {
	static get properties() {
		return {
			value:{type:String},
			height : { type : Number },
			autosize : { type : Boolean },
			autocomplete : { type : Boolean },
			autofocus : { type : Boolean },
			disabled : { type : Boolean, reflect : true },
			placeholder : { type : String },
			readonly : { type : Boolean },
			label:{type:String},
			maxlength : { type : Number }
		}
	}

	static get styles() {
		return css`
		:host {
			display:var(--flow-input-display, inline-block);
			vertical-align:middle;
			font-family:var(--flow-font-family, "Julius Sans One");
			font-weight:var(--flow-font-weight, bold);
			width:var(--flow-input-width, 100%);
			min-width:var(--flow-input-min-width, 100px);
			max-width:var(--flow-input-max-width, 500px);
			margin:var(--flow-input-margin, 5px 0px);
			font-size:0px;
		}
		label{
			font-size:var(--flow-input-label-font-size, 0.7rem);
			padding:var(--flow-input-label-padding,2px 5px);
			border: var(--flow-input-label-border, 2px) solid  var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1)));
			border-radius:var(--flow-input-label-border-radius, 8px);
			margin-left: var(--flow-input-label-margin-left,10px);
			margin-right: var(--flow-input-label-margin-right,20px);
			z-index: var(--flow-input-label-z-index, 1);
			position: var(--flow-input-label-position, relative);
			background-color:var(--flow-input-bg, inherit);
		}
		textarea{

			width:var(--flow-input-input-width, 100%);
			box-sizing:border-box;
			height:var(--flow-input-height);
			border: var(--flow-input-border, 2px) solid var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1)));
			border-radius:var(--flow-input-border-radius, 8px);
			margin:0px;
			padding:var(--flow-input-padding, 10px);
			font-size:var(--flow-input-font-size, 1rem);
			font-weight:var(--flow-input-font-weight, 400);
			font-family:var(--flow-input-font-family);
			line-height:var(--flow-input-line-height, 1.2);
			box-shadow:var(--flow-input-box-shadow);
			text-align:var(--flow-input-text-align);
			min-width:var(--flow-input-input-min-width, 10px);
			letter-spacing:var(--flow-input-letter-spacing, inherit);

			min-width: var(--flow-textarea-min-width, 200px);
			min-height: var(--flow-textarea-min-height, 32px);
			overflow: hidden;
			overflow-y:hidden;
			outline: none;
			resize: none;
			background-color: var(--flow-textarea-bg, var(--flow-input-bg, #fafafa));
			color: var(--flow-textarea-color, var(--flow-input-color, inherit));
			margin-top: var(--flow-input-wrapper-margin-top,-0.5rem);
		}
		textarea[has-label]{
			padding-top:var(--flow-input-with-label-input-padding-top, 15px)
		}
		.length-msg{
			font-size:var(--flow-textarea-length-msg-font-size, var(--flow-input-length-msg-font-size, 0.6rem));
		}
		`;
	}

	constructor() {
		super();

		this.ident = Math.round((Math.random()*1e16)).toString(16);

		this.height = 0;
		this.autosize = true;
		this.autocomplete = false;
		this.autofocus = false;
		this.disabled = false;
		this.placeholder = '';
		this.readonly = false;
		this.maxlength = undefined;
		this.value = this.innerHTML;
		this.innerHTML = "";
	}

	get textarea() {
		return this.renderRoot && this.renderRoot.querySelector('textarea');
	}

	get() {
		return this.textarea.value;
	}

	set(value) {
		this.textarea.value = value;
		this.change();
	}

	render() {
		//this.log("RENDERING TEXTAREA");
		let isLabel = !!this.label;
		return html`
			<label ?hidden=${!isLabel}>${this.label||""}</label>
			<textarea id="textarea-${this.ident}"
				@input="${this.change}" 
				autocomplete="${this.autocomplete}"
				?autofocus="${this.autofocus}"
				placeholder="${this.placeholder}"
				?readonly="${this.readonly}"
				?disabled="${this.disabled}"
				?has-label="${isLabel}"
				maxlength="${ifDefined(this.maxlength)}"
				.value="${this.value}"
			></textarea>
			${this.renderRemainingCharsMsg()}
		`;

	}
	renderRemainingCharsMsg(){
		if (!this.maxlength){
			return ''
		}

		let n = Math.max(0, this.maxlength - (this.textarea?.value.length||0));
		let msg = n==1? T("character left"):T("characters left");
		return html`<span class="length-msg">${n} ${msg}</span>`;
	}

	change() {
		let textarea = this.textarea;
		textarea.style.height = 'auto';
		let style = window.getComputedStyle(textarea);
		let border = parseFloat(style.getPropertyValue('border-width')) * 2;
		textarea.style.height = (textarea.scrollHeight+border)+'px';
		this.value = this.textarea.value;
		this.fire("changed", {el:this, value:this.textarea.value});
	}
}

FlowTextArea.define('flow-textarea');
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_FORM_CONTROL_363 : &'static str = r###"


/**
 * @export
 * @class FlowFormControl
 * @extends {BaseElement}
 * 
 * @property {String} [icon=light-info] icon to show in left
 * @property {String}  expandIcon
 * @property {Boolean} expandable
 * @property {Boolean} expanded

 * @cssvar {fill} [--flow-primary-color=rgba(0,151,115,1)]
 * @cssvar {width} [--flow-form-control-icon-box-width=24px]
 * @cssvar {height} [--flow-form-control-icon-box-height=24px]
 * @cssvar {margin-right} [--flow-form-control-icon-box-margin=8px]
 * @cssvar {width} [--flow-form-control-input-box-width=100px]

 * @example
 * <flow-form-control>
 *   Is Active: 
 *   <flow-checkbox-test slot="input"></flow-checkbox-test>
 * </flow-form-control>
 */
export class FlowFormControl extends BaseElement {
	static get properties() {
		return {
			icon:{type: String},
			expandIcon:{type: String},
			expandable:{type:Boolean},
			focusable:{type:Boolean},
			expanded:{type:Boolean, reflect:true}
		}
	}

	static get styles() {
		return css`
		:host{
			display:flex;
			align-items:flex-start;
			margin:10px 0px;
			flex-wrap:var(--flow-form-control-flex-wrap, wrap);
			position:var(--flow-form-control-position, initial);
		}
		.icon-box,
		.expandable-icon-box{
			width:30px;
			max-width:30px;
			text-align:center;
			align-self:var(--flow-form-control-icon-box-align-self,initial);
			position:var(--flow-form-control-icon-box-position,initial);
			top:var(--flow-form-control-icon-box-top,initial);
			bottom:var(--flow-form-control-icon-box-bottom,initial);
			left:var(--flow-form-control-icon-box-left,initial);
			right:var(--flow-form-control-icon-box-right,initial);
		}
		.expandable-icon-box svg{cursor:pointer}
		:host([icon="none"]) .icon-box{display:none}
		:host(.no-icon]) .icon-box{display:none}
		:host([no-icon]) .icon-box{display:none}

		.icon-box svg,
		.expandable-icon-box svg{
			width:var(--flow-form-control-icon-box-width,24px);
			height:var(--flow-form-control-icon-box-height,24px);
			margin-right:var(--flow-form-control-icon-box-margin,8px);
			fill:var(--flow-primary-color, rgba(0,151,115,1.0));
		}
		.title-box{
			user-select:none;line-height:24px;cursor:pointer;
		}
		:host([focusable]) .title-box:focus,
		:host([focusable]) .input-box:focus{
			display:var(--flow-form-control-focus-display, inline-block);
			outline:var(--flow-form-control-focus-outline, dotted);
		}
		.input-box{
			width: var(--flow-form-control-input-box-width,100px);
			padding: var(--flow-form-control-input-box-padding, 0px);
			flex:1;
		}
		.input{margin:5px 5px 5px 0px;}
		:host([no-help]) .info-box,
		:host([no-info]) .info-box{
			display:none;
		}
		.info-box{
			flex:1;
			max-width:300px;
			padding:var(--flow-form-control-info-box-padding, 0px 10px);
			box-sizing:border-box;
			min-width:var(--flow-form-control-info-box-min-width, initial);
			font-size:var(--flow-form-control-info-box-font-size, inherit);
		}
		.info-box ::slotted(*){
			margin:unset;
		}
		.info-box ::slotted(h4.title){
			border-bottom: 1px solid #ddd;
		    margin:0px 0px 10px 0px;
		    font-weight: bold;
		    font-size: 1.1em;
		}
		.info-box ::slotted(p){
			font-size:0.8em;
		}
		:host([expandable]:not([expanded])) .input,
		:host([expandable]:not([expanded])) .info-box ::slotted(*){
			display:none;
		}
		:host([expanded]:not([static-icon])) .expandable-icon-box > svg{
			transform:rotate(90deg)
		}
		`;
	}
	render() {
		let iconSrc = "";
		if(this.icon != "-")
			iconSrc = this.iconPath(this.icon || "info-circle");
		let icon2Src = "";
		if(this.expandIcon != "-")
			icon2Src = this.iconPath(this.expandIcon || "caret-right");
		return html`
			<div class="icon-box" @click="${this.click}"><svg><use href="${iconSrc}"></use></svg></div>
			${
				this.expandable?
				html`<div class="expandable-icon-box" 
					data-flow-expandable="toggle" @click="${this.click}"><svg>
				<use href="${icon2Src}"></use>
				</svg></div>`: ''
			}
			${
			this.focusable?
			html`<div class="input-box" tabindex="0">
				<label @click="${this.click}" class="title-box" 
					data-flow-expandable="toggle"><slot name="title"></slot></label>
				<div class="input"><slot></slot><slot name="input"></slot></div>
			</div>`:
			html`<div class="input-box">
				<label @click="${this.click}" class="title-box" 
					data-flow-expandable="toggle"><slot name="title"></slot></label>
				<div class="input"><slot></slot><slot name="input"></slot></div>
			</div>`
			}
			<div class="info-box"><slot name="info"></slot></div>
		`;

	}

	click(){
		// let target = e.target.closest("[data-flow-expandable]")
		// if(!target)
		// 	return
		let action = this.getAttribute("data-flow-expandable") || 'toggle';

		if(["toggle", "open", "close"].includes(action))
			this[action]();
	}

	firstUpdated(...args){
		super.firstUpdated(...args);
		this.label = this.renderRoot.querySelector("label.title-box");
		this.inputBox = this.renderRoot.querySelector(".input-box");
	}

	/*	

		_onClick(e){
			let target = e.target.closest("[data-flow-expandable]")
			if(!target)
				return
			let action = target.getAttribute("data-flow-expandable") || 'toggle';

			if(["toggle", "open", "close"].includes(action))
				this[action]();
		}
	*/

	focus(){
		//console.log("focus:this.label", this.label)
		if(this.inputBox){
			this.inputBox.focus()
		}else{
			super.focus();
		}
	}
	open(){
		this.expanded = true;
	}
	close(){
		this.expanded = false;
	}
	toggle(){
		this.expanded = !this.expanded;
	}
}

FlowFormControl.define('flow-form-control');
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_FORM_MIXIN_376 : &'static str = r###"
export const FlowFormValidators = {
    email : (email) => {
        const re = /^(([^<>()\[\]\\.,;:\s@"]+(\.[^<>()\[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
        return re.test(String(email).toLowerCase());
    }
}

export const FlowFormMixin = baseClass => {
	class FlowFormIface extends baseClass {



        declareSchema(prefix, schema, validators) {
            this.schema = schema;
            this.validators = validators;

            const sampleSchema = {
                'email' : { type : String, validator : FlowFormValidators.email },
                'lastname' : { type : String, length : 128 }
            }
        }

        gatherInputs(list) {
            let ctls = { };
            list.forEach(item => {
                const [id, property] = item.split('.');
                ctls[id] = this.renderRoot.getElementById(id)[property||'value'];
            });
            return ctls;
        }

        async gatherInputData(schema, resolve) {

            if(!schema)
                schema = this.schema;
            if(!schema) {
                return Promise.reject(`missing schema in ${this.constructor.name}`);
            }

            if(Array.isArray(schema)) {
                const src = schema;
                schema = { };
                src.forEach(t => { schema[t] = { type : String }; });
            }


            const data = { };
            Object.keys(schema).forEach((key) => {
                let el = this.renderRoot.getElementById(`${key}`);
                if(!el)
                    return console.error(`unknown field ${key} in data`,data,`occurred in`,this);

                let value = null;
                switch(el.tagName.toLowerCase()) {
                    case 'flow-input': {
                        value = el.value;
                    } break;

                    default: {
                        console.error(`Unknown flow form input control type ${el.tagName}`, el, 'error occurred in:',this);
                    } break;
                }

                data[key] = value;
            })

            return data;
        }

        fillInputs(data) {

            if(!schema)
                schema = this.schema;
            if(!schema) {
                return Promise.reject(`missing schema in ${this.constructor.name}`);
            }

            if(Array.isArray(schema)) {
                const src = schema;
                schema = { };
                src.forEach(t => { schema[t] = { type : String }; });
            }

            const keys = Object.keys(data);
            Object.keys(schema).forEach((key) => {
                let el = this.renderRoot.getElementById(`${key}`);
                if(!el)
                    return console.error(`unknown field ${key} in data`,data,`occurred in`,this);
                
                switch(el.tagName.toLowerCase()) {
                    case 'flow-input': {
                        el.value = value;
                    } break;

                    default: {
                        console.error(`Unknown flow form input control type ${el.tagName}`, el, 'error occurred in:',this);
                    } break;
                }

                data[field] = value;
            })

            return data;
        }

        // validateInput(data) {
        //     let fields = Object.keys(this.schema);
        //     gatherInput
        // }
	}

	return FlowFormIface;
}
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_FOLDER_INPUT_390 : &'static str = r###"



/**
* @class FlowFolderInput
* @extends BaseElement

* @property {Boolean} [disabled]
* @property {String} [btnText]
* @property {String} [value]
*
* @cssvar {font-family} [--flow-font-family="Julius Sans One"]
* @cssvar {font-weight} [--flow-font-weight=bold]
* @cssvar {background-color|border} [--flow-border-color=var(--flow-primary-color, rgba(0,151,115,1))]
* @cssvar {background|border} [--flow-border-hover-color=var(--flow-primary-color, rgba(0,151,115,1))]
* @cssvar {border-top-left-radius} [--flow-folder-input-tlbr=4px]
* @cssvar {border-bottom-left-radius} [--flow-folder-input-blbr, 4p]
* @cssvar {border-top-right-radius} [--flow-folder-input-trbr=4px]
* @cssvar {border-bottom-right-radius} [--flow-folder-input-brbr=4px]
* @cssvar {color} [--flow-border-invert-color=var(--flow-primary-invert-color, #FFF))]
* @cssvar {margin-right} [--flow-folder-input-vmr=2px]
* @cssvar {background-color} [--flow-input-bg=inherit]
* @cssvar {width} [--flow-folder-input-width=100%]
* @cssvar {min-width} [--flow-folder-input-min-width=100px]
* @cssvar {max-width} [--flow-folder-input-max-width=100%]
* @cssvar {border-top-left-radius} [--flow-folder-input-tlbr=4px]
* @cssvar {border-bottom-left-radius} [--flow-folder-input-blbr=4px);
* @example
*   <flow-folder-input></flow-folder-input>
*
*/
export class FlowFolderInput extends BaseElement {
	static get properties() {
		return {
			btnText:{type: String},
			value:{type:String},
			disabled:{type:Boolean}
		}
	}

	static get styles(){
		return css`
			:host{
				display:inline-block;
				font-family:var(--flow-font-family, "Julius Sans One");
				font-weight:var(--flow-font-weight, bold);
				width:var(--flow-folder-input-width, 100%);
				min-width:var(--flow-folder-input-min-width, 100px);
				max-width:var(--flow-folder-input-max-width, 100%);
			}
			:host(:not([disabled])) label,
			:host(:not([disabled])) label input{
				cursor:pointer;
			}
			
			.wrapper{
				display:flex;
				align-items:stretch;
				min-width:50px;
				text-align:center;
				justify-content:center;
			}
			label{
				position:relative;
				padding:5px;
				background-color:var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border: 2px solid var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1)));
				overflow:hidden;
				border-radius:8px;
				border-top-left-radius: var(--flow-folder-input-tlbr, 4px);
    			border-bottom-left-radius: var(--flow-folder-input-blbr, 4px);
    			color:var(--flow-border-invert-color, var(--flow-primary-invert-color, #FFF));
			}
			:host(:not([disabled])) label:hover{
				background-color:var(--flow-border-hover-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-color:var(--flow-border-hover-color, var(--flow-primary-color, rgba(0,151,115,1)))
			}
			label input{
			    position: absolute;
			    left: 0px;
			    top: 0px;
			    right: 0px;
			    bottom: 0px;
			    font-size: 200px;
			    height: 63px;
			    background: #F00;
			    opacity:0;
			    z-index:-1;
			}
			label .text{
				z-index:1;
			}
			.value{
				position:relative;
			    display: flex;
			    align-items: center;
			    padding: 0px 30px 0px 5px;
				box-sizing: border-box;
				margin-right:var(--flow-folder-input-vmr, 2px);
				width:150px;flex:1;
				min-height:32px;
				border: 2px solid var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-radius:8px;
				border-top-right-radius: var(--flow-folder-input-trbr, 4px);
    			border-bottom-right-radius: var(--flow-folder-input-brbr, 4px);
    			background-color:var(--flow-input-bg, inherit);
			}
			.value>span{
				overflow:hidden;text-overflow:ellipsis;flex:1;white-space:nowrap;
				text-align:left;
			}
			:host([disabled]) .value{
				padding-right:10px;
			}
			.clear-btn{
				font-style: normal;
			    font-size: 25px;
			    padding: 0px 10px 0px 10px;
			    cursor: pointer;
			    display:none;
			    position: absolute;
			    right: 0px;
			    z-index: 1;
			}
			:host(:not([disabled])) [has-value] .clear-btn{display:block;}
		`;
	}
	render() {
		return html`
		<div class="wrapper" @click=${this.onClick} ?has-value=${!!this.value}>
			<slot name="prefix"></slot>
			<div class="value">
				<span>${this.value}</span>
				<i class="clear-btn" title="Clear" @click=${this.setClear}>&times;</i>
			</div>
			<label class="btn">
				<input type="file" ?disabled=${this.disabled} nwdirectory @change=${this.onChange} />
				<div class="text"><flow-i18n text="${this.btnText || 'Select Folder'}"></flow-i18n></div>
			</label>
			<slot name="sufix"></slot>
		</div>
		`;
	}

	setClear(){
		this.setValue("");
	}

	onClick() {
		this.fire("flow-folder-input-click", {el:this})
	}

	onChange(e) {
		let value = this.shadowRoot.querySelector("input").value;
		if(!value)
			return
		this.value = value;
		this.fire("changed", {el:this, value})
	}

	setValue(value){
		this.value = value;
		this.shadowRoot.querySelector("input").value = "";
		this.fire("changed", {el:this, value:this.value})
	}
}

FlowFolderInput.define('flow-folder-input');
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_INPUT_428 : &'static str = r###"




/**
* @class FlowInput
* @extends BaseElement

* @property {Boolean} [disabled]
* @property {String} [btnText]
* @property {String} [value]
* @property {Number} [maxlength] set maxlength of input
*
*
* @cssvar {font-family} [--flow-font-family="Julius Sans One"]
* @cssvar {font-weight} [--flow-font-weight=bold]
* @cssvar {font-weight} [--flow-input-font-weight=400]
* @cssvar {font-size} [--flow-input-font-size-label=0.7rem]
* @cssvar {font-size} [--flow-input-font-size=1rem]
* @cssvar {width} [--flow-input-width=100%]
* @cssvar {min-width} [--flow-input-min-width=100px]
* @cssvar {max-width} [--flow-input-max-width=500px]
* @cssvar {height} [--flow-input-height]
* @cssvar {line-height} [--flow-input-line-height=1.2]
* @cssvar {background-color} [--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1))]
* @cssvar {background-color} [--flow-border-hover-color, var(--flow-primary-color, rgba(0,151,115,1))]
* @cssvar {background-color} [--flow-input-bg=inherit]
* @cssvar {border} [--flow-input-border-label=2px solid  var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1)))]
* @cssvar {border} [--flow-input-border=2px solid var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1))]
* @cssvar {border-top-left-radius} [--flow-input-btn-tlbr=0px]
* @cssvar {border-bottom-left-radius} [--flow-input-btn-blbr=0px]
* @cssvar {border-color} [--flow-border-hover-color, var(--flow-primary-color, rgba(0,151,115,1)]
* @cssvar {color} [--flow-border-invert-color, var(--flow-primary-invert-color, #FFF)]
* @cssvar {color} [--flow-input-color=inherit]
* @cssvar {color} [--flow-input-placeholder=#888]
* @cssvar {color} [--flow-input-invalid-color=red]
* @cssvar {padding} [--flow-input-padding-label=2px 5px]
* @cssvar {margin} [--flow-input-margin=5px 0px]
* @example
*   <flow-input></flow-input>
*
*/
export class FlowInput extends BaseElement {
	static get properties() {
		return {
			btnText:{type: String},
            value:{type:String},
            type:{type:String},
			disabled:{type:Boolean, reflect:true},
			pattern:{type:String},
			validator:{type:Function},
			placeholder:{type:String},
			label:{type:String},
			readonly:{type:Boolean},
			expression:{type:Boolean},
			maxlength:{type:Number},
			max:{type:Number},
			min:{type:Number},
			enterkeyhint:{type:String},
			"tab-index":{type:Number},
			"clear-btn":{type:Boolean, reflect:true}
		}
	}

	static get styles(){
		return css`
			:host{
				display:var(--flow-input-display, inline-block);
				vertical-align:middle;
				font-family:var(--flow-font-family, "Julius Sans One");
				font-weight:var(--flow-font-weight, bold);
				width:var(--flow-input-width, 100%);
				min-width:var(--flow-input-min-width, 100px);
				max-width:var(--flow-input-max-width, 500px);
				margin:var(--flow-input-margin, 5px 0px);
				font-size:0px;
			}
			:host(:not([disabled])) .btn{
				cursor:pointer;
			}
			
			:host(:not([apply-btn])) .btn{
				display: none;
			}
			
			.wrapper{
				display:flex;
				align-items:stretch;
				min-width_:50px;
				text-align:left;
				justify-content:center;
				margin-top:var(--flow-input-wrapper-margin-top,-0.5rem);
				height:var(--flow-input-wrapper-height);
			}
			label{
				font-size:var(--flow-input-label-font-size, 0.7rem);
				padding:var(--flow-input-label-padding,2px 5px);
				border: var(--flow-input-label-border, 2px) solid  var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-radius:var(--flow-input-label-border-radius, 8px);
				margin-left: var(--flow-input-label-margin-left,10px);
				margin-right: var(--flow-input-label-margin-right,20px);
				z-index: var(--flow-input-label-z-index, 1);
				position: var(--flow-input-label-position, relative);
				background-color:var(--flow-input-bg, inherit);
			}
			.btn{
				position:relative;
				padding:var(--flow-input-btn-padding, 5px);
				min-width:var(--flow-input-btn-min-width, 10px);
				background-color:var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border: 2px solid var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1)));
				overflow:hidden;
				border-radius:8px;
				border-top-left-radius: var(--flow-input-btn-tlbr, 0px);
    			border-bottom-left-radius: var(--flow-input-btn-blbr, 0px);
    			color:var(--flow-border-invert-color, var(--flow-primary-invert-color, #FFF));
    			display:var(--flow-input-btn-display, flex);
			    justify-content: center;
			    align-items: center;
			}
			:host(:not([disabled])) .btn:hover{
				background-color:var(--flow-border-hover-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-color:var(--flow-border-hover-color, var(--flow-primary-color, rgba(0,151,115,1)))
			}
			.input{
				width:var(--flow-input-input-width,100px);
				flex:1;box-sizing:border-box;
				height:var(--flow-input-height);
				border: var(--flow-input-border, 2px) solid var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-radius:var(--flow-input-border-radius, 8px);
    			margin:0px;
    			padding:var(--flow-input-padding,10px);
				background-color:var(--flow-input-bg, inherit);
				color:var(--flow-input-color, inherit);
				font-size:var(--flow-input-font-size, 1rem);
				font-weight:var(--flow-input-font-weight, 400);
				font-family:var(--flow-input-font-family);
				line-height:var(--flow-input-line-height, 1.2);
				box-shadow:var(--flow-input-box-shadow);
				text-align:var(--flow-input-text-align);
				min-width:var(--flow-input-input-min-width, 10px);
				letter-spacing:var(--flow-input-letter-spacing, inherit);
			}
			/*
			:host(:not([outer-border])) .input{
				box-shadow:var(--flow-input-box-shadow);
			}
			*/
			.btn .text{
				font-size:var(--flow-input-btn-font-size, 1rem);
			}

			:host([apply-btn]) .input,
			:host([sufix-btn]) .input,
			:host([suffix-btn]) .input{
			    border-right-width:0px;
				border-top-right-radius: 0px;
				border-bottom-right-radius: 0px;
			}
			:host([sufix-btn]) ::slotted([slot="sufix"]),
			:host([suffix-btn]) ::slotted([slot="suffix"]){
				border-top-left-radius: 0px;
				border-bottom-left-radius: 0px;
				margin-bottom:0px;
			}

			:host([outer-border]) .input,
			:host([clear-btn]) .input{
				border:0px;
				height:calc(var(--flow-input-height) - 4px);
				background-color:transparent;
				box-shadow:none;
			}
			:host([outer-border]) .wrapper,
			:host([clear-btn]) .wrapper{
				border:var(--flow-input-border, 2px) solid var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-radius:var(--flow-input-border-radius, 8px);
				background-color:var(--flow-input-bg, inherit);
				color:var(--flow-input-color, inherit);
				box-shadow:var(--flow-input-wrapper-box-shadow);
			}
			:host([outer-border]) .wrapper{
				box-shadow:var(--flow-input-wrapper-box-shadow, var(--flow-input-box-shadow));
			}


			.input:focus{outline:none}
			.input::-webkit-input-placeholder { color: var(--flow-input-placeholder, #888 ); }
			:host([disabled]) .value{
				padding-right:10px;
			}
			.clear-btn{margin:5px 10px;align-self:center;cursor:pointer}
			:host(.invalid) .input{color:var(--flow-input-invalid-color, red)}
			.wrapper:not([has-value]) ::slotted([hide-on-empty]),
			.wrapper:not([has-value]) .clear-btn{
				display:none
			}
			.wrapper[has-label] input{
				padding-top:var(--flow-input-with-label-input-padding-top, 15px)
			}
		`;
    }
    constructor() {
        super();
        this.type = 'text';
		this.value = '';
		this.expression = false;
    }
	render() {
		let isLabel = !!this.label;
		return html`<label ?hidden=${!isLabel}>${this.label||""}</label>
		<div class="wrapper" @click=${this.onClick} ?has-value=${!!this.value} ?has-label=${isLabel}>
			<slot name="prefix"></slot>
			<input class="input" type="${this.type}" spellcheck="false"
				placeholder="${this.placeholder || ''}"
				pattern="${ifDefined(this.pattern)}"
				maxlength="${this.maxlength||''}"
				max="${this.max||''}"
				min="${this.min||''}"
				enterkeyhint="${this.enterkeyhint||'done'}"
				tabindex="${this['tab-index']==undefined?'':this['tab-index']}"
				?readonly=${this.readonly}
				?disabled=${this.disabled}
				@change=${this.onChange}
				@input=${this.onInput}
				.value="${this.value}" />
			<div class="btn" @click=${this.onBtnClick}>
				<div class="text"><flow-i18n text="${this.btnText || 'Apply'}"></flow-i18n></div>
			</div>
			${this['clear-btn']?html`
				<fa-icon clear-input class="clear-btn"
					icon="times"></fa-icon>
			`:''}
			<slot name="sufix"></slot>
			<slot name="suffix"></slot>
		</div>
		`;
	}

	firstUpdated(...args){
		super.firstUpdated(...args)

		this.renderRoot.addEventListener("click", (e)=>{
        	this._onClick(e);
        })
	}

	setClear(){
		this.setValue("");
	}

	_onClick(e){
		if(e.target.closest("[clear-input]")){
			this.clear();
		}
	}

	onClick(e) {
		this.fire("flow-input-click", {el:this})
	}
	onBtnClick(e){
		if(this.disabled)
			return
		this.fire("btn-click", {el:this, e})
	}

	validate(value){
		let {pattern} = this;
		if(pattern){
			try{
				pattern = new RegExp(pattern)
			}catch(e){
				this.log("pattern error:", e)
				return false;
			}
			if(!pattern.test(value))
				return false;
		}
		if(typeof this.validator == 'function'){
			return this.validator(value, this);
		}
		return true;
	}

	clear(){
		let value = "";
		this.value = value;
		this.fire("changed", {el:this, value});
		this.fire("inputted", {el:this, value})
	}

	onChange(e) {
		let value = this.renderRoot.querySelector("input").value;
		value = this.parseExpressionValue(value);
		if(!this.validate(value)){
			this.classList.add("invalid")
			return
		}
		this.classList.remove("invalid")
		//this.log("value", value)

		this.value = value;
		this.fire("changed", {el:this, value})
	}
	onInput(e) {
		let value = this.renderRoot.querySelector("input").value;
		if(!this.validate(value)){
			this.classList.add("invalid")
			return
		}
		this.classList.remove("invalid")
		//this.log("value", value)

		this.value = value;
		this.fire("inputted", {el:this, value})
	}

	setValue(value){
		this.value = value;
		this.renderRoot.querySelector("input").value = "";
		this.fire("changed", {el:this, value:this.value})
	}
	getInputValue(){
		return this.renderRoot.querySelector("input").value
	}

	parseExpressionValue(value){
		if(!this.expression)
			return value;	
		value = value.replace(/,/g,'');
		value = value.replace(/\d+\.?\d*\s*[km]/ig,(v)=>{
			if(/m/i.test(v))
				v = parseFloat(v)*1_000_000;
			else if(/k/i.test(v))
				v = parseFloat(v)*1_000;
			return v;
		});
		try{
			value = eval(`(${value})`);
		}catch(ex){
			console.log(ex);
		}
		return value;
	}
}

FlowInput.define('flow-input');
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_CORE_INPUT_375 : &'static str = r###"
export class FlowCoreInput extends HTMLInputElement{
	connectedCallback(){
		this.initState();
		this.initEvents();
	}
	initState(){
		//console.log("this.dataset.allowedPattern", this.dataset.allowedPattern)
		this.__state = {
			value: this.value,
			start: this.selectionStart,
			end: this.selectionEnd,
			allowedPattern: this.dataset.allowedPattern ? RegExp(`^${this.dataset.allowedPattern}$`): null
		};
	}
	initEvents(){
		this.addEventListener('focus', this._onFocus.bind(this));
		this.addEventListener('input', this._onInput.bind(this));
		this.addEventListener('keydown', this._onKeyDown.bind(this));
	}

	getState(){
		return this.__state;
	}

	_onInput(e){
		let state = this.getState();
		if (!state.allowedPattern || state.allowedPattern.test(this.value)) {
			state.value = this.value;
		}else{
			//console.log("input FAIL:", state.allowedPattern, this.value)
			this.value = state.value;
			this.setSelectionRange(state.start, state.end);
		}
	}
	_onFocus(){
		let state = this.getState()
		state.value = this.value;
		state.start = this.selectionStart;
		state.end = this.selectionEnd
	}
	_onKeyDown(e){
		let state = this.getState();
		state.start = this.selectionStart;
		state.end = this.selectionEnd
	}
}

customElements.define('flow-core-input', FlowCoreInput, {extends:'input'});

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_EXPANDABLE_377 : &'static str = r###"


/**
 * @export
 * @class FlowExpandable
 * @extends {BaseElement}
 * @property {String} [icon=light-info] icon to show on the left of the expandable element
 * @property {Boolean} [expand] 
 * @prop {Boolean} no-info
 * @cssvar {fill} [--flow-primary-color=rgba(0,151,115,1)]


 * 
 * @example
 * <flow-expandable>
 *   <div slot="title">Is Active:</div>
 *   <flow-checkbox></flow-checkbox>
 * </flow-expandable>
 */
export class FlowExpandable extends BaseElement {
	static get properties() {
		return {
			icon:{type: String},
			expand:{type:Boolean, reflect:true},
			'no-info':{type:Boolean, reflect:true},
			caption:{type:String}
		}
	}

	static get styles() {
		return css`
		:host{
			display:flex;
			align-items:flex-start;
			margin:var(--flow-expandable-margin,10px 0px);
		}
		.icon-box{
			width:30px;
			position:var(--flow-expandable-icon-box-position,initial);
			max-width:var(--flow-expandable-icon-box-max-width,30px);
			text-align:center;
		}
		.icon-box, .title-box{
			display:flex;
			align-items:center;
			min-height:30px;
			cursor:pointer;
			user-select:none;
		}
		.title-box{
			position:relative;
		}
		.title-box:after{
			position:absolute;
			left:0px;
			top:0px;
			content:"";
			z-index:1;
			width:100%;
			height:100%;
		}
		.icon-box svg{
			width:var(--flow-expandable-icon-box-svg-width,24px);
			height:var(--flow-expandable-icon-box-svg-height,24px);
			margin-right: var(--flow-expandable-icon-box-svg-margin-right,8px);
			fill:var(--flow-primary-color, rgba(0,151,115,1.0));
		}
		.content-box{
			width:100px;
			flex:1;
			min-width: var(--flow-expandable-content-box-min-width);
		}
		.content{
			margin:var(--flow-expandable-content-margin,5px 5px 5px 10px);
			
		}

		.info-box{
			flex:1;
			max-width:300px;
			padding:0px 10px;
		}
		:host([no-help]) .info-box,
		:host([no-info]) .info-box{
			display:none
		}
		:host([no-icon]) .icon-box{
			display:none;
		}
		.info-box ::slotted(*){
			margin:unset;
		}
		.info-box ::slotted(h4.title){
			border-bottom: 1px solid #ddd;
		    margin:0px 0px 10px 0px;
		    font-weight: bold;
		    font-size: 1.1em;
		}
		.info-box ::slotted(p){
			font-size:0.8em;
		}
		:host(:not([expand])) .content{display:none}
		:host([expand]:not([static-icon])) .icon-box > svg{
			transform:rotate(90deg)
		}
		`;
	}
	render() {
		let iconSrc = "";
		if(this.icon != "-")
			iconSrc = this.iconPath(this.icon || "caret-right");
		return html`
			<div class="icon-box" data-flow-expandable="toggle" @click=${this.toggle}><svg><use href="${iconSrc}"></use></svg></div>
			<div class="content-box">
				<label class="title-box" data-flow-expandable="toggle" @click=${this.toggle}><slot name="title"></slot>${this.caption||''}</label>
				<div class="content"><slot></slot></div>
			</div>
			<div class="info-box"><slot name="info"></slot></div>
		`;
	}
//	firstUpdated(){
//		this.renderRoot.addEventListener("click", this._onClick.bind(this));
//	}

	// _onClick(e){
	// 	let target = e.target.closest("[data-flow-expandable]")
	// 	if(!target)
	// 		return
	// 	let action = target.getAttribute("data-flow-expandable") || 'toggle';

	// 	if(["toggle", "open", "close"].includes(action))
	// 		this[action]();
	// }

	open(){
		this.expand = true;
	}
	close(){
		this.expand = false;
	}
	toggle(){
		this.expand = !this.expand;
	}
}

FlowExpandable.define('flow-expandable');
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_QRCODE_384 : &'static str = r###"




/**
* @class FlowQRCode
* @extends BaseElement
* @property {String} [for]
* @property {String} [type]
* @example
*   <flow-qrcode></flow-qrcode>
*
*/

export class FlowQRCode extends BaseElement {
	static get properties() {
		return {
			for : { type : String },
			ntype : { type : Number },
			data : { type : String },
			ecl : { type : String },
			mode : { type : String },
			multibyte : { type : String },	
		}
	}

	static get styles() {
		return css`
			:host {
				display : block;
			}

			svg {
				width: 100%;
				height: 100%;
			}
			
			:host(.left-img) img{
				object-position:left;
			}
		`;
	}

	constructor() {
		super();

		this.ntype = 0;
		this.ecl = 'M';
		this.mode = 'Byte';
		this.multibyte = 'UTF-8';
		this.data = '';
	}

	render() {
		if(!window.qrcode) {
			dpc(128, () => {
				this.requestUpdate();
			});
			return 'QR';
		}
		if(!this.qr || this.data != this.data_last_) {
			this.data_last_ = this.data;
			this.qr = this.createQRCode(this.data, this.ntype, this.ecl, this.mode, this.multibyte);
		}
		return this.qr;
	}

	createQRCode(text, typeNumber,
		errorCorrectionLevel, mode, mb) {

		window.qrcode.stringToBytes = window.qrcode.stringToBytesFuncs[mb];

		var qr = qrcode(typeNumber || 4, errorCorrectionLevel || 'M');
		qr.addData(text, mode);
		qr.make();
		let cellSize = 2;
		let margin = cellSize * 4;

		let el = document.createElement('div');
		el.innerHTML = qr.createSvgTag(cellSize, margin);
		return el;
	}

}

FlowQRCode.define('flow-qrcode', [baseUrl+'resources/extern/qrcode/qrcode.js']);
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_QRCODE_SCANNER_420 : &'static str = r###"




//console.log("jsQR", jsQR)

export class FlowQRCodeScanner extends BaseElement {
	static get properties() {
		return {
			cameras:{type:Array},
			selectedCamera:{type:Object},
			qrCode:{type:String},
			errorMessage:{type:String},
			hideCode:{type:Boolean, reflect:true},
			boxColor:{type:String},
			renderAfter:{type:Number}
		}
	}

	static get styles() {
		return css`
			:host {
				display : block;
				min-width: 100px;
				min-height: 100px;
				/*max-width: 400px;*/
				/*overflow:auto;*/
				margin:auto;
				position:relative;
				/*max-height:80vh;*/
			}
			.error{
				font-size:var(--flow-qrcode-scanner-error-font-size, 0.8rem);
				color:var(--flow-qrcode-scanner-error-color, #F00);
				padding:var(--flow-qrcode-scanner-error-padding, 10px);
			}
			.wait-msg, .select-msg{
				font-size:var(--flow-qrcode-scanner-msg-font-size, 1rem);
				text-align:center;padding:10px;
			}
			.video{border:0px solid #000000;display:none;}
			.view{
				border:1px solid #000000;display:block;margin:5px auto;
				width:var(--flow-qrcode-scanner-canvas-width, 280px);
				height:var(--flow-qrcode-scanner-canvas-height, 280px);
				object-fit:contain;
			}
			.render-canvas{
				position:absolute;border:0px solid #000;display:none
			}
			.code-box{
				display:flex;
				flex-direction:var(--flow-qrcode-scanner-code-box-flex-direction, column);
				align-items:var(--flow-qrcode-scanner-code-box-align-items, center);
			}
			.code{
				border:0px;-webkit-appearance:none;outline:none;
				margin:var(--flow-qrcode-scanner-code-margin, 10px);
				overflow:hidden;text-overflow:ellipsis;
				font-size:var(--flow-qrcode-scanner-code-font-size, 1rem);
				background-color:transparent;color:var(--flow-primary-color);
				font-family:var(--flow-qrcode-scanner-code-font-family, "Exo 2");
				word-wrap:break-word;
				max-width:100%;
				width:var(--flow-qrcode-scanner-code-width, 300px);
				height:var(--flow-qrcode-scanner-code-height, auto);
				/*max-height:var(--flow-qrcode-scanner-code-max-height, 100px);*/
				resize:none;
				display:block;
			}
			:host([hidecode]) .code-box{display:none}
			.logs{width:90%;height:100px;}
			:host(:not([logs])) .logs{display:none}
			.camera-selection{
				display:flex;
			}
			:host([debug]) .render-canvas,
			:host([debug]) .video{
				display:block;position:relative;margin:auto;
			}
			/*
			.camera-selection flow-select{
				max-width:var(--flow-qrcode-scanner-s-max-width, 400px);
				--flow-dropdown-display:var(--flow-qrcode-scanner-s-display, block);
				--flow-select-width:var(--flow-qrcode-scanner-sw, var(--flow-select-width, 100%));
				--flow-select-margin:var(--flow-qrcode-scanner-sm, var(--flow-select-margin, 10px auto));
			}
			*/
		`;
	}

	constructor() {
		super();
		this.stopped = true;
		this.renderAfter = 10;
	}

	render() {
		let {cameras=[], selectedCamera='', qrCode='', errorMessage=''} = this;
		return html`
			<textarea class="logs"></textarea>
			<div class="error">${errorMessage}</div>
			${this.renderScanning()}
			${this.renderCameraSelection()}
			${this.renderCode()}
		`;
	}

	renderCameraSelection(){
		const {cameras, selectedCamera, cameraDiscovery, stopped} = this;
		if(cameraDiscovery === false || stopped)
			return '';
		if(!cameras)
			return html`<div class="wait-msg" is="i18n-div">Please wait. Getting cameras.</div>`;
		//if(selectedCamera)
		//	return html`<div>Selected cameras: ${selectedCamera.label}</div>`

		let selected = selectedCamera?.id||'';
		return html`
		<div class="camera-selection">
			<!--div class="select-msg">Select cameras</div-->
			<flow-select label="${T('Select cameras')}"
				@select="${this.onCameraSelect}" selected="${selected}">
				${cameras.map(c=>{
					return html`<flow-menu-item
						value="${c.id}">${c.label}</flow-menu-item>`
				})}
			</flow-select>
		</div>
		`
	}
	renderScanning(){
		//let {selectedCamera} = this;
		//if(!selectedCamera)
		//	return ''

		return html`
			<video class="video" width="320" height="240" autoplay></video>
			<canvas class="render-canvas"></canvas>
			<img class="view">
		`
	}

	renderCode(){
		let {qrCode} = this;
		if(!qrCode)
			return '';
		return html`
			<div class="code-box">
				<!--div class="label">QR code:</div-->
				<div class="code">QR code: ${qrCode}</div>
				<flow-btn @click="${this.clearCode}">Clear</flow-btn>
			</div>
		`;
	}
	clearCode(){
		this.setQRCode("");
	}
	setQRCode(qrCode){
		this.qrCode = qrCode;
		this.fire("changed", {code: qrCode})
	}

	firstUpdated(){
		super.firstUpdated();
		this.viewImg = this.renderRoot.querySelector(".view");
		this.init();
	}
	updated(){
		super.updated();
		this.initScanning();
	}
	_log(title, data){
		let input = this.renderRoot.querySelector(".logs");
		input.value += `\n--------------\n${title}\n`+JSON.stringify(data)
	}
	stop(){
		this.stopped = true;
		let {video} = this;
		this.closeActiveStreams(video?.srcObject)
	}
	start(){
		this.stopped = false;
		this.scanning = false;
		this.init();
	}
	getVideoElement(){
		if(this._video)
			return this._video;
		this._video = document.createElement("video");
		return this._video;
	}
	getCanvasElement(){
		if(this._canvas)
			return this._canvas;
		this._canvas = document.createElement("canvas");
		return this._canvas;
	}
	initScanning(){
		this.__render = 0;
		if(this.qrCode || this.stopped)
			return
		let canvas = this.getCanvasElement();//this.renderRoot.querySelector(".render-canvas");
		let video = this.getVideoElement()//this.renderRoot.querySelector(".video")
		let {selectedCamera} = this;
		this._log("initScanning", {canvas:!!canvas, video:!!video, selectedCamera, scanning:this.scanning})
		if(!canvas || !video || !selectedCamera)
			return

		if(this.scanning == selectedCamera.id)
			return
		this.closeActiveStreams(video.srcObject)
		this.scanning = selectedCamera.id;
		this.video = video;

		const canvasCtx = canvas.getContext('2d', {alpha: false});
		//const desiredWidth = 1280;
		//const desiredHeight = 720;

		const constraints = {
			audio: false,
			video:{deviceId: { exact: selectedCamera.id }}
			/*,
			video: {
				// the browser will try to honor this resolution,
				// but it may end up being lower.
				width: desiredWidth,
				height: desiredHeight
			}*/
		};

		// open the webcam stream
		navigator.mediaDevices.getUserMedia(constraints).then((stream) => {
			video.srcObject = stream;
			const track = stream.getVideoTracks()[0];
			const trackInfo = track.getSettings();
			//let {width, height} = trackInfo;
			// required to tell iOS safari we don't want fullscreen
			video.setAttribute("playsinline", true);
			video.play();

			let box = this.getBoundingClientRect();

			//console.log(actualSettings.width, actualSettings.height)
			canvas.width = trackInfo.width;//200;//actualSettings.width;
			canvas.height = trackInfo.height;//200;//actualSettings.height;

			canvasCtx.lineWidth = 4;
			canvasCtx.strokeStyle = this.boxColor || "#FF3B58";

			let { 
				offsetX, offsetY, width, height
			} = contain(canvas.width, canvas.height, trackInfo.width, trackInfo.height);

			offsetX = Math.floor(offsetX)
			offsetY = Math.floor(offsetY)
			width = Math.floor(width)
			height = Math.floor(height)

			requestAnimationFrame(()=>{
				this.videoTick({
					video, box:{offsetX, offsetY, width, height}, canvas, canvasCtx,
					cameraId : selectedCamera.id
				})
			});

		}).catch((e) => {
			throw e
		});
	}

	setError(error){
		this.errorMessage = error;
	}

	async init(){
		if(this.stopped)
			return
		try{
			let cameras = await this.getCameras();
			this.cameras = cameras;
			let backCameras =  cameras.filter(c=>!c.label.toLowerCase().includes("front"))
			if(backCameras.length){
				this.selectedCamera = backCameras[0];
			}else if(cameras.length){
				this.selectedCamera = cameras[0];
			}
			this.log("cameras", cameras)
		}catch(e){
			console.log("getCameras:error", e)
			this.setError(html`Camera discovery process failed.
				<br />Make sure you have given Camera permission.`)
			this.cameraDiscovery = false;
		}
	}
	closeActiveStreams(stream){
		if(!stream)
			return

		// 	alert('stopping');
		// return;
		const tracks = stream.getVideoTracks();
		for (var i = 0; i < tracks.length; i++) {
			const track = tracks[i];
			track.enabled = false;
			track.stop();
			stream.removeTrack(track);
		}
	}

	getCameras() {
		return new Promise((resolve, reject) => {
			if (navigator.mediaDevices
				&& navigator.mediaDevices.enumerateDevices
				&& navigator.mediaDevices.getUserMedia) {
				this.log("navigator.mediaDevices used");
				navigator.mediaDevices.getUserMedia({ audio: false, video: true })
				.then(stream => {
					// hacky approach to close any active stream if they are
					// active.
					stream.oninactive
						= _ => this.log("All streams closed");

					navigator.mediaDevices.enumerateDevices()
						.then(devices => {
							const results = [];
							//alert("devices:"+JSON.stringify(devices))
							for (var i = 0; i < devices.length; i++) {
								const device = devices[i];
								if (device.kind == "videoinput") {
									if(!/front/i.test(device.label) || devices.length == 1) {
										results.push({
											id: device.deviceId,
											label: device.label
										});
									}
								}
							}
							this.log(`${results.length} results found`);
							this.closeActiveStreams(stream);
							resolve(results);
						})
						.catch(err => {
							reject(`${err.name} : ${err.message}`);
						});
				})
				.catch(err => {
					reject(`${err.name} : ${err.message}`);
				})
			} else if (MediaStreamTrack && MediaStreamTrack.getSources) {
				this.log("MediaStreamTrack.getSources used");
				const callback = sourceInfos => {
					const results = [];
					for (var i = 0; i !== sourceInfos.length; ++i) {
						const sourceInfo = sourceInfos[i];
						if (sourceInfo.kind === 'video') {
							results.push({
								id: sourceInfo.id,
								label: sourceInfo.label
							});
						}
					}
					this.log(`${results.length} results found`);
					resolve(results);
				}
				MediaStreamTrack.getSources(callback);
			} else {
				this.log("unable to query supported devices.");
				reject("unable to query supported devices.");
			}
		});
	}

	stopScanning(){
		let {video} = this;
		this._log("stopScanning", 'video_srcObject:'+(!!video?.srcObject))
		if(video?.srcObject){
			this.closeActiveStreams(video.srcObject)
			video.srcObject = null;
		}
		this.scanning = false;
	}

	videoTick({video, box, canvasCtx, canvas, cameraId}) {
		if(cameraId != this.selectedCamera?.id)
			return

		let next = ()=>{
			if(this.qrCode){
				this.stopScanning()
				return
			}
			if(this.stopped)
				return;
				
			requestAnimationFrame(()=>{
				this.videoTick({video, box, canvas, canvasCtx, cameraId})
			});
		}

		this.__render++;
		if(this.__render != 1){
			if(this.__render<this.renderAfter)
				return next();

			this.__render = 0;
		}


		if (video.readyState !== video.HAVE_ENOUGH_DATA)
			return next();

		canvasCtx.fillRect(0, 0, box.width, box.height);
		canvasCtx.drawImage(video, box.offsetX, box.offsetY, box.width, box.height);
		let imageData = canvasCtx.getImageData(0, 0, box.width, box.height);

		let code = jsQR(imageData.data, imageData.width, imageData.height, {
			inversionAttempts: "dontInvert",
		});

		if (code) {
			let loc = code.location;
			this.drawLine(canvasCtx, loc.topLeftCorner, loc.topRightCorner);
			this.drawLine(canvasCtx, loc.topRightCorner, loc.bottomRightCorner);
			this.drawLine(canvasCtx, loc.bottomRightCorner, loc.bottomLeftCorner);
			this.drawLine(canvasCtx, loc.bottomLeftCorner, loc.topLeftCorner);
			this.setQRCode(code.data);
		}

		this.viewImg.src = canvas.toDataURL("image/jpeg");

		next();
		
	}

	drawLine(ctx, begin, end){
		ctx.beginPath();
		ctx.moveTo(begin.x, begin.y);
		ctx.lineTo(end.x, end.y);
		ctx.stroke();
	}

	onCameraSelect(e){
		let {selected} = e.detail;
		console.log("selected", selected)
		this.selectedCamera = this.cameras.find(c=>c.id == selected);
	}

}

FlowQRCodeScanner.define('flow-qrcode-scanner');

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_BTN_380 : &'static str = r###"




/**
* @class FlowBtn
* @extends BaseElement
* @example
*   <flow-btn>Button 1</flow-btn>
* @property {Boolean} [disabled] 
* @cssvar {font-family} [--flow-btn-font-family=var(--flow-font-family, initial)]
* @cssvar {font-weight} [--flow-btn-font-weight=var(--flow-font-weight, bold)]
* @cssvar {font-size} [--flow-btn-font-size=initial]
* @cssvar {line-height} [--flow-btn-line-height=inherit]
* @cssvar {color} [--flow-border-color=var(--flow-primary-color, rgba(0,151,115,1))]
* @cssvar {color} [--flow-btn-hover-border-color=var(--flow-primary-color, rgba(0,151,115,1))]
* @cssvar {color} [--flow-btn-primary-invert-color=var(--flow-primary-invert-color, #FFF)]
* @cssvar {color} [--flow-btn-success-color=var(--flow-btn-success-color, #FFF)]
* @cssvar {color} [--flow-btn-hover-info-color=var(--flow-btn-info-color, #FFF))]
* @cssvar {color} [--flow-btn-hover-success-color=var(--flow-btn-success-color, #FFF))]
* @cssvar {color} [--flow-btn-hover-warning-color=var(--flow-btn-warning-color, #FFF))]
* @cssvar {color} [--flow-btn-hover-danger-color=var(--flow-btn-danger-color, #FFF))]
* @cssvar {color} [--flow-btn-warning-color=var(--flow-btn-warning-color, #FFF))]
* @cssvar {color} [--flow-btn-danger-color=var(--flow-btn-danger-color, #FFF))]
* @cssvar {color} [--flow-btn-info-color=var(--flow-btn-info-color, #FFF))]
* @cssvar {background-color} [--flow-btn-primary-bg-color=var(--flow-primary-color, rgba(0,151,115,1))]
* @cssvar {background-color} [--flow-btn-success-bg-color=var(--flow-primary-color, rgba(0,151,115,1))]
* @cssvar {background-color|border-color} [--flow-btn-hover-success-bg-color=var(--flow-primary-color, rgba(0,151,115,1)))]
* @cssvar {background-color|border-color} [--flow-btn-warning-bg-color=var(--flow-primary-color, rgba(0,151,115,1)))]
* @cssvar {background-color|border-color} [--flow-btn-hover-warning-bg-color=var(--flow-primary-color, rgba(0,151,115,1)))]
* @cssvar {background-color|border-color} [--flow-btn-danger-bg-color=var(--flow-primary-color, rgba(0,151,115,1)))]
* @cssvar {background-color|border-color} [--flow-btn-hover-danger-bg-color=var(--flow-primary-color, rgba(0,151,115,1)))]
* @cssvar {background-color|border-color} [--flow-btn-info-bg-color=var(--flow-primary-color, rgba(0,151,115,1)))]
* @cssvar {background-color|border-color} [--flow-btn-hover-info-bg-color=var(--flow-primary-color, rgba(0,151,115,1)))]
* @cssvar {border-color} [--flow-btn-hover-success-bg-color=var(--flow-primary-color, rgba(0,151,115,1)))]
* @cssvar {border-color} [--flow-btn-success-bg-color=var(--flow-primary-color, rgba(0,151,115,1)))]
* @cssvar {border-radius} [--flow-btn-radius=8px]
* @cssvar {margin} [--flow-btn-margin=""]
* @cssvar {margin} [--flow-btn-wrapper-margin=5px];
* @cssvar {min-width} [--flow-btn-wrapper-min-width=50px];
* @cssvar {padding} [--flow-btn-padding=5px]
*/
export class FlowBtn extends BaseElement {
	static get properties() {
		return {
			disabled:{type:Boolean, reflect: true},
			'on-click':{type:Function},
			icon:{type:String, reflect:true},
			i18n:{type:Boolean, reflect: true}
		}
	}

	static get styles(){
		return css`
			:host{
				display:var(--flow-btn-display, inline-flex);
				margin: var(--flow-btn-margin);
				padding:var(--flow-btn-padding, 5px);
				border: var(--flow-btn-border, 2px solid var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1))));
				border-radius:var(--flow-btn-radius, 8px);
				border-width:var(--flow-btn-border-width, 2px);
				font-family:var(--flow-btn-font-family, var(--flow-font-family, initial));
				font-weight:var(--flow-btn-font-weight, var(--flow-font-weight, bold));
				font-size:var(--flow-btn-font-size, initial);
				line-height:var(--flow-btn-line-height, inherit);
				text-transform:var(--flow-btn-text-transform, inherit);
				user-select: none;
				--fa-icon-size-temp:var(--fa-icon-size);
			}
			:host([icon]){
				padding:var(--flow-iconbtn-padding, var(--flow-btn-padding, 5px));
			}
			fa-icon{

				--fa-icon-size:var(--flow-btn-icon-size, var(--fa-icon-size-temp, 20px))
			}
			:host([disabled]){
				opacity:0.5;
				cursor:default;
				pointer-events:none;
			}
			:host(.start-justifed){
				justify-self:flex-start;
			}
			:host(.end-justifed){
				justify-self:flex-end;
			}
			:host(:not([disabled])){
				cursor:pointer;
				background-color:var(--flow-btn-bg-color, inherit);
				border-color:var(--flow-btn-border-color, inherit);
				color:var(--flow-btn-color, inherit);
			}
			:host(:not([disabled]):hover){
				background-color:var(--flow-btn-hover-bg-color, inherit);
				border-color:var(--flow-btn-hover-border-color, inherit);
				color:var(--flow-btn-hover-color, inherit);
				--fa-icon-color:var(--flow-btn-hover-color, inherit);
			}
			:host([primary]),
			:host(.primary){
				background-color:var(--flow-btn-primary-bg-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-color:var(--flow-btn-primary-border-color, var(--flow-primary-color, rgba(0,151,115,1)));
				color:var(--flow-btn-primary-invert-color, var(--flow-primary-invert-color, #FFF));
				--fa-icon-color:var(--flow-btn-primary-invert-color, var(--flow-primary-invert-color, #FFF));
			}
			:host([primary]:not([disabled]):hover),
			:host(.primary:not([disabled]):hover){
				background-color:var(--flow-btn-hover-primary-bg-color, var(--flow-btn-hover-border-color, var(--flow-primary-color, rgba(0,151,115,1))));
				color: var(--flow-btn-hover-primary-color);
			}

			:host(.secondary){
				background-color:var(--flow-btn-secondary-bg-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-color:var(--flow-btn-secondary-bg-color, var(--flow-primary-color, rgba(0,151,115,1)));
				color:var(--flow-btn-secondary-color, #FFF);
			}

			:host(.secondary:not([disabled]):hover){
				background-color:var(--flow-btn-hover-secondary-bg-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-color:var(--flow-btn-hover-secondary-bg-color, var(--flow-primary-color, rgba(0,151,115,1)));
				color:var(--flow-btn-hover-secondary-color, var(--flow-btn-secondary-color, #FFF));
			}

			:host(.success){
				background-color:var(--flow-btn-success-bg-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-color:var(--flow-btn-success-bg-color, var(--flow-primary-color, rgba(0,151,115,1)));
				color:var(--flow-btn-success-color, #FFF);
			}

			:host(.success:not([disabled]):hover){
				background-color:var(--flow-btn-hover-success-bg-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-color:var(--flow-btn-hover-success-bg-color, var(--flow-primary-color, rgba(0,151,115,1)));
				color:var(--flow-btn-hover-success-color, var(--flow-btn-success-color, #FFF));
			}

			:host(.warning){
				background-color:var(--flow-btn-warning-bg-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-color:var(--flow-btn-warning-bg-color, var(--flow-primary-color, rgba(0,151,115,1)));
				color:var(--flow-btn-warning-color, #FFF);
			}

			:host(.warning:not([disabled]):hover){
				background-color:var(--flow-btn-hover-warning-bg-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-color:var(--flow-btn-hover-warning-bg-color, var(--flow-primary-color, rgba(0,151,115,1)));
				color:var(--flow-btn-hover-warning-color, var(--flow-btn-warning-color, #FFF));
			}

			:host(.danger){
				background-color:var(--flow-btn-danger-bg-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-color:var(--flow-btn-danger-bg-color, var(--flow-primary-color, rgba(0,151,115,1)));
				color:var(--flow-btn-danger-color, #FFF);
			}

			:host(.danger:not([disabled]):hover){
				background-color:var(--flow-btn-hover-danger-bg-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-color:var(--flow-btn-hover-danger-bg-color, var(--flow-primary-color, rgba(0,151,115,1)));
				color:var(--flow-btn-hover-danger-color, var(--flow-btn-danger-color, #FFF));
			}

			:host(.info){
				background-color:var(--flow-btn-info-bg-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-color:var(--flow-btn-info-bg-color, var(--flow-primary-color, rgba(0,151,115,1)));
				color:var(--flow-btn-info-color, #FFF);
			}

			:host(.info:not([disabled]):hover){
				background-color:var(--flow-btn-hover-info-bg-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-color:var(--flow-btn-hover-info-bg-color, var(--flow-primary-color, rgba(0,151,115,1)));
				color:var(--flow-btn-hover-info-color, var(--flow-btn-info-color, #FFF));
			}

			.wrapper{
				display:flex;
				align-items:center;
				margin:var(--flow-btn-wrapper-margin,5px);
				min-width:var(--flow-btn-wrapper-min-width,50px);
				text-align:center;
				justify-content:center;
				box-sizing:border-box;
			}
			:host([full-height-wrapper]) .wrapper,
			:host([icon]) .wrapper{
				height:100%;
				margin:0px;
			}
			:host([i18n]) slot,
			:host(:not([i18n])) #text-element{
				display:none
			}
		`;
	}
	constructor(){
		super()
		this.addEventListener("click", ()=>{
			this.click();
		})
		this.i18nSupport = (this.getAttribute("i18n") != null);
		if(this.i18nSupport){
			i18nElementsMap.set(this, {});
		}
	}
	render() {
		let {icon=''} = this;
		return html`<div 
			class="wrapper">${icon?html`<fa-icon icon=${icon}></fa-icon>`:''} <slot id="text-slot"></slot><span id="text-element"></span></div>`;
	}

	firstUpdated(...args){
		super.firstUpdated(...args);
		this.setAttribute('role', 'button');
		if(this.i18nSupport){
			this.slotElement = this.renderRoot.querySelector("#text-slot")
			this.textElement = this.renderRoot.querySelector("#text-element")
			this.slotElement.addEventListener('slotchange', (e)=>{
				this.slotElementChidren = this.slotElement.assignedNodes();
				let text = [];
				this.slotElementChidren.forEach(el=>{
					text.push(el.textContent)
				})
				//console.log("this.slotElementChidren", this.slotElementChidren, text)
				this.__i18nText = i18n.cleanText(text.join(""));
				this.setI18nValue(this.__i18nText?i18n.t(this.__i18nText):'')
			})
			
		}
	}

	setI18nValue(text){
		if(this.textElement){
			this.textElement.innerHTML = text;
		}
	}

	click() {
		if(this.disabled)
			return
		this.fire("flow-btn-click", {el:this})
		let clickFn = this['on-click'];
		if(clickFn){
			if(typeof clickFn == "string"){
				try{
					eval(clickFn);
				}catch(e){

				}
			}else if(typeof clickFn == "function"){
				clickFn();
			}
		}

		/*let form = this.form || this.getAttribute("form");

		if(typeof(form) == 'string'){
			form = document.querySelector(form);
		}

		if(form && form.submit())*/
	}

	connectedCallback(){
		super.connectedCallback();
		if(this.i18nSupport){
			i18nElementsMap.set(this, {});
		}
	}

	disconnectedCallback(){
		super.disconnectedCallback();
		if(this.i18nSupport){
			i18nElementsMap.delete(this, {});
		}
	}
}

FlowBtn.define('flow-btn');

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_GROUP_BTNS_403 : &'static str = r###"



/**
* @class FlowGroupBtns
* @extends BaseElement
* @property {Boolean} [disabled]
* @property {Boolean} [toggleable] 
* @property {String} [selected=""] 
* @property {String} [valueAttr="data-value"] 
* @cssvar {font-family} [--flow-font-family="Julius Sans One"]
* @cssvar {font-weight} [--flow-font-weight=bold]
* @cssvar {border-radius} [--flow-btn-radius=8px]
* @cssvar {margin} [--flow-group-btns-margin]
* @cssvar {border-color} [--flow-group-btns-border-color=var(--flow-primary-color, rgba(0,151,115,1))]
* @cssvar {border-width} [--flow-group-btns-border-width=1px]
* @cssvar {font-size} [--flow-group-btns-font-size=initial]
* @cssvar {line-height} [--flow-group-btns-line-height=inherit]
* @cssvar {align-items} [--flow-group-btns-align-items=stretch]
* @cssvar {min-width} [--flow-group-btns-wrapper-min-width=50px]
* @cssvar {justify-content} [--flow-group-btns-wrapper-justify=initial]
* @example
*   <flow-group-btns selected="1">
*		<flow-btn data-value="1">Button 1</flow-btn>
*		<flow-btn data-value="2">Button 2</flow-btn>
*	</flow-group-btns>
*/

/*
... @ cssvar {--flow-btn-bg-color} [--flow-group-btns-active-bg-color=var(--flow-primary-color)]
... @ cssvar {--flow-btn-color} [--flow-group-btns-active-color=var(--flow-primary-invert-color)]
... @ cssvar {--flow-btn-hover-color} [--flow-group-btns-active-color=var(--flow-primary-invert-color)]
*/

export class FlowGroupBtns extends BaseElement {
	static get properties() {
		return {
			disabled:{type:Boolean, reflect: true},
			selected:{type:String},
			valueAttr:{type:String},
			toggleable:{type:Boolean}
		}
	}

	static get styles(){
		return css`
			:host{
				display:inline-block;
				margin: var(--flow-group-btns-margin);
				padding:0px;
				border:1px solid var(--flow-group-btns-border-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-radius:var(--flow-group-btns-radius, 8px);
				border-width:var(--flow-group-btns-border-width, 1px);
				font-family:var(--flow-group-btns-font-family, var(--flow-font-family, initial));
				font-weight:var(--flow-group-btns-font-weight, var(--flow-font-weight, bold));
				font-size:var(--flow-group-btns-font-size, initial);
				line-height:var(--flow-group-btns-line-height, inherit);
				user-select: none;
				overflow:hidden;
				background-color: var(--flow-group-btns-bg, var(--flow-input-bg, inherit));
			}
			:host([disabled]){
				opacity:0.5;
				cursor:default;
				pointer-events:none;
			}

			:host ::slotted(flow-btn){
				--flow-btn-margin:0px;
				--flow-btn-border-width:0px;
				--flow-btn-radius:0px;
				--flow-btn-wrapper-margin:5px;
				--flow-btn-hover-color:var(--flow-group-btns-active-color-hover, var(--flow-primary-invert-color));
				border-right:var(--flow-group-btns-border-width, 1px) solid var(--flow-group-btns-border-color, var(--flow-primary-color, rgba(0,151,115,1)))
			}

			:host ::slotted(flow-btn.active){
				--flow-btn-bg-color:var(--flow-group-btns-active-bg-color, var(--flow-primary-color));
				--flow-btn-color:var(--flow-group-btns-active-color, var(--flow-primary-invert-color));
				--flow-btn-hover-color:var(--flow-group-btns-active-color-hover, var(--flow-primary-invert-color));
			}

			:host ::slotted(flow-btn:last-child){
				border-right:0px;
			}

			.wrapper{
				display:flex;flex-direction:row;
				align-items:var(--flow-group-btns-align-items, stretch);
				min-width: var(--flow-group-btns-wrapper-min-width, 50px);
				text-align:center;
				justify-content:var(--flow-group-btns-wrapper-justify, initial);
			}
		`;
	}
	constructor(){
		super()
		this.valueAttr = "data-value";
		this.setAttribute('role', 'buttons');
	}
	render() {
		return html`
		<div class="wrapper" @click=${this.click}>
			<slot></slot>
		</div>
		`;
	}
	firstUpdated(){
		this.listSlot = this.renderRoot.querySelector('slot');
		this.updateList();
	}
	updated(...args){
		super.updated(...args);
		this.updateList();
	}

	click(e) {
		if(this.disabled)
			return
		let target = e.target;
		let selected = target.getAttribute(this.valueAttr);
		if(this.toggleable && this.selected == selected){
			selected = "";
		}

		this.log("selected", selected)

		this.selected = selected;
		this.fire("group-btn-select", {el:this, selected})
		this.updateList();
	}
	updateList(){
		if(!this.listSlot)
			return
		this.listSlot.assignedElements()
			.map(btn=>{
				let selected = this.selected == btn.getAttribute(this.valueAttr);
				btn.classList.toggle("active", selected);
			})
	}
}

FlowGroupBtns.define('flow-group-btns');

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_SHELL_LINK_405 : &'static str = r###"





/**
* @class FlowShellLink
* @extends BaseElement
*
* @property {Boolean} [disabled] 
* @property {Boolean} [icon]
* @property {String} [href]
*
* @cssvar {font-family} [--flow-font-family="Julius Sans One"]
* @cssvar {font-weight} [--flow-font-weight=bold]
* @cssvar {color} [--flow-link-color=#017b68]
* @cssvar {color} [--flow-link-hover-color=#017b68]
* @cssvar {fill} [--flow-primary-color=017b68]
*
* @example
*   <flow-shell-link url="url">text</flow-shell-link>
*
*/
export class FlowShellLink extends BaseElement {
	static get properties() {
		return {
			disabled:{type:Boolean, reflect: true},
			icon:{type:Boolean, reflect: true},
			href : { type : String }
		}
	}

	static get styles(){
		return css`
			:host{
				display:inline-block;
				font-family:var(--flow-font-family, "Julius Sans One");
				font-weight:var(--flow-font-weight, bold);
			}
			:host([disabled]){
				opacity:0.5;
				cursor:default;
				pointer-events:none;
			}
			:host(:not([disabled])){
				cursor:pointer;
			}

			.link-wrapper {
				color: var(--flow-link-color, #017b68);
				display: flex;
			}

			.link-wrapper:hover {
				color: var(--flow-link-hover-color, #017b68);
			}

			.icon-box {
				display: block;
				width: 16px;
				height: 16px;
				margin-bottom: -4px;
				opacity: 0.65;
			}

			.icon-box svg {
				fill: var(--flow-primary-color, #017b68);
				width: 100%;
				height: 100%;
			}

			.content {
				display: block;
			}
		`;
	}

	constructor(){
		super()
	}

	render() {
		let iconSrc = this.iconPath(`external-link-square-alt`);
		return html`
		<div class="link-wrapper" @click=${this.click}>
			<div class="content"><slot></slot></div>
			${ this.icon ? html`<div class="icon-box"><svg><use href="${iconSrc}"></use></svg></div>` : '' }
		</div>
		`;
	}

	click() {
		this.fire("flow-shell-link-click", {el:this})
		console.log("opening href:",this.href);
		require('nw.gui').Shell.openExternal( this.href );

	}
}

FlowShellLink.define('flow-shell-link');
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_WINDOW_LINK_349 : &'static str = r###"



//if(typeof nw != 'undefined'){
	/*chrome.webRequest.onBeforeRequest.addListener(function(details) {
    	console.log("onBeforeRequest", details)
		return {cancel: false};
    },
    {urls: ["<all_urls>"]},
    ["blocking"]);*/
	//onHeadersReceived
	/*
	chrome.webRequest.onResponseStarted.addListener(function(details){
		let {responseHeaders, url, type} = details;
		
		if(type == "script"){
			console.log("onHeadersReceived", responseHeaders, details)
			let found = responseHeaders.find(a=>{
				return a.name && a.name.toLowerCase() == 'content-type';
			})
			if(!found)
				responseHeaders.push({name: "Content-Type", value: "text/javascript"})
		}
		return responseHeaders;
	},
	{urls: ["<all_urls>"]},
	['responseHeaders', 'extraHeaders']);
	*/
//}

/**
* @class FlowWindowLink
* @extends BaseElement
*
* @property {String} [url]
* @property {String} [id]
* @property {String} [title]
* @property {Boolean} [disabled]
* @property {String} [icon] window icon
* @property {Number} [width]
* @property {Number} [height]
* @property {Boolean} [resizable]
* @property {Boolean} [frame]
* @property {Boolean} [transparent]
* @property {Number} [min_width]
* @property {Number} [min_height]
* @property {Number} [max_width]
* @property {Number} [max_height]
* @property {Boolean} [as_desktop]
* @property {Boolean} [always_on_top] 
* @property {Boolean} [visible_on_all_workspaces] (OS X Only)
* @property {Boolean} [new_instance] Open the window in a separate process
*
*
* @cssvar {font-family} [--font-family=var(--flow-font-family, "Open Sans")]
* @cssvar {font-weight} [--font-weight=var(--flow-font-weight, normal)]
* @cssvar {color} [--flow-link-color=var(--flow-link-color, #017b68)]
* @cssvar {color} [--flow-link-hover-color=var(--flow-link-hover-color, #017b68)]
* @cssvar {fill} [--flow-primary-color=rgba(0,151,115,1)]
*
* @example
*   <flow-window-link href="url">text</flow-window-link>
*
* http://docs.nwjs.io/en/latest/References/Manifest%20Format/#window-subfields
*
*/
export class FlowWindowLink extends BaseElement {
	static get properties() {
		return {
			disabled:{type:Boolean, reflect: true},
			icon:{type:String},
			url : { type : String },
			id : { type : String },
			title : { type : String },
			width : { type : Number },
			height : { type : Number },
			resizable : { type : Boolean },
			frame : { type : Boolean },
			transparent : { type : Boolean },
			fullscreen : { type : Boolean },
			min_width : { type : Number },
			min_height : { type : Number },
			max_width : { type : Number },
			max_height : { type : Number },
			as_desktop : { type : Boolean },
			always_on_top : { type : Boolean },
			visible_on_all_workspaces : { type : Boolean },
			new_instance : { type : Boolean },
		}
	}

	static get styles(){
		return css`
			:host{
				display:inline-block;
				font-family:var(--flow-font-family, "Open Sans");
				font-weight:var(--flow-font-weight, normal);
			}
			:host([disabled]){
				opacity:0.5;
				cursor:default;
				pointer-events:none;
			}
			:host(:not([disabled])){
				cursor:pointer;
			}

			.link-wrapper {
				color: var(--flow-link-color, #017b68);
				display: flex;
			}

			.link-wrapper:hover {
				color: var(--flow-link-hover-color, #017b68);
			}
/*
			.icon-box {
				display: block;
				width: 16px;
				height: 16px;
				margin-bottom: -4px;
				opacity: 0.65;
			}

			.icon-box svg {
				fill: var(--flow-primary-color, #017b68);
				width: 100%;
				height: 100%;
			}
*/
			.content {
				display: block;
			}
		`;
	}

	constructor(){
		super();

		this.width = 1024;
		this.height = 768;
		this.resizable = true;
		this.frame = true;
		this.transparent = false;
		this.fullscreen = false;
		this.icon = undefined;
		this.min_width = undefined;
		this.min_height = undefined;
		this.max_width = undefined;
		this.max_height = undefined;
		this.as_desktop = false;
		this.always_on_top = false;
		this.visible_on_all_workspaces = false;
		this.new_instance = undefined;
		this.show = true;
		this.url = '';

		if(!window.flow)
			window.flow = { };
		if(!window.flow['flow-window-link'])
			window.flow['flow-window-link'] = { windows : [ ] };
	}

	render() {
		//let iconSrc = this.iconPath(this.linkicon || `external-link-square-alt`);
		return html`
		<div class="link-wrapper" @click=${this.click}>
			<div class="content"><slot></slot></div>
		</div>
			`;
			//${ this.icon ? html`<div class="icon-box"><svg><use href="${iconSrc}"></use></svg></div>` : '' }
	}

	click() {
		this.fire("flow-window-link-click", {el:this})
		console.log("opening url:",this.url);
		//require('nw.gui').Shell.openExternal( this.href );

		const { 
			id, 
			title, 
			width, 
			height, 
			resizable, 
			frame, 
			transparent, 
			show, 
			fullscreen, 
			icon, 
			min_width, 
			min_height, 
			max_width, 
			max_height, 
			as_desktop, 
			always_on_top, 
			visible_on_all_workspaces, 
			new_instance 
		} = this;

		let args = {
			id, 
			title, 
			width, 
			height, 
			resizable, 
			frame, 
			transparent, 
			show, 
			fullscreen, 
			icon, 
			min_width, 
			min_height, 
			max_width, 
			max_height, 
//			as_desktop, 
			always_on_top, 
//			visible_on_all_workspaces, 
			new_instance,
			//new_instance: true,
			// id: this.id,
			// title: this.title,
			// width: 1027,
			// height: 768,
			// resizable: true,
			// frame: true,
			// transparent: false,
			// show: true,
			// http://docs.nwjs.io/en/latest/References/Manifest%20Format/#window-subfields
		};



        if(this.url && typeof nw != 'undefined') {
            nw.Window.open(this.url, args, (win, b) => {
            	win.window.appOrigin = location.origin;
				window.flow['flow-window-link'].windows.push(win);

				//this.window = win;
				

                // console.log("win", win)
                // win.app = this;
                // global.abcapp = "123";
                // resolve();
            });
        }

	}
}

FlowWindowLink.define('flow-window-link');
"###;

const ASPECTRON_FLOW_UX_SRC_FA_ICON_418 : &'static str = r###"


/**
 *
 * @export
 * @class FaIcon
 * @extends {BaseElement}
 * @property {String} [style] inner svg tag style text
 * @example
 * <fa-icon icon="fal:chart-network"></fa-icon>
 * <fa-icon icon="icons:chart-network"></fa-icon>
 */
export class FaIcon extends BaseElement {
	static get properties() {
		return {
			color:String,
			src: String,
			style: String,
			css: String,
			size:Number,
			w:Number,
			h:Number,
			icon:String
		};
	}
	static get styles() {
		return css`
		:host {
			display: inline-block;
			padding: var(--fa-icon-padding, 0px);
			margin: var(--fa-icon-margin, 0px);
			width: var(--fa-icon-size, 19px);
			height: var(--fa-icon-size, 19px);
		}
		svg,img{
			width: var(--fa-icon-size, 19px);
			height: var(--fa-icon-size, 19px);
			fill: var(--fa-icon-color);
		}
		img{object-fit:contain;}
		`;
	}
	constructor() {
		super();
		this.src = '';
		this.style = '';
		this.css = '';
		//this.size = 19;
		this.color = '';
	}
	firstUpdated() {
		//this.src = this.getSources(this.class_);
	}
	render() {
		this.src = this.iconPath(this.icon);
		//let size1 = this.style.getPropertyValue("--fa-icon-size");
		//console.log("size1size1", size1)
		let {size, color, w, h} = this;
		w = (w||size)?`width:${w||size}px;`:'';
		h = (h||size)?`height:${h||size}px;`:'';
		color = color?`fill:${color};`:'';
		let ext = this.src.split(/\?#/)[0].split(".").pop().toLowerCase();
		if(['png', 'jpeg', 'jpg'].includes(ext)){
			return html`<img style="${w}${h}${color}${this.css||''}" src="${this.src}" />`;
		}
		return html`
		<svg style="${w}${h}${color}${this.style}"><use href="${this.src}"></use></svg>
		`;
	}
}
FaIcon.define('fa-icon');
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_DATA_BADGE_386 : &'static str = r###"




/**
* @class FlowDataBadge
* @extends BaseElement
* @example
*   <flow-data-badge title="text">value</flow-data-badge>
* @property {Boolean} [disabled] 
* @property {String} [title] 
* @property {String} [prefix] 
* @property {String} [suffix] 
* @property {String} [align] 
* @cssvar {font-family} [--flow-data-badge-font-family="Julius Sans One"]
* @cssvar {font-weight} [--flow-data-badge-font-weight=bold]
* @cssvar {border} [--flow-primary-color=#333]
* @cssvar {width} [--flow-data-badge-width]
* @cssvar {min-width} [--flow-data-badge-min-width]
* @cssvar {max-width} [--flow-data-badge-max-width]
* @cssvar {margin} [--flow-data-badge-margin]
* @cssvar {border} [--flow-data-badge-container-border=2px solid var(--flow-primary-color,#333)]
* @cssvar {background-color} [--flow-data-badge-bg=inherit]
* @cssvar {padding} [--flow-data-badge-container-padding=2px 6px]
* @cssvar {margin} [--flow-data-badge-container-margin=6px]
* @cssvar {box-shadow} [--flow-data-badge-container-box-shadow=2px 2px 1px rgba(1, 123, 104, 0.1)]
* @cssvar {border-radius} [--flow-data-badge-container-border-radius=10px]
* @cssvar {opacity} [--flow-data-badge-title-opacity=1]
* @cssvar {font-size} [--flow-data-badge-title-font-size=10px] 
* @cssvar {color} [--flow-data-badge-caption]
* @cssvar {font-size} [--flow-data-badge-value-font-size=14px]
* @cssvar {font-family} [--flow-data-badge-value-font-family="Exo 2"]
* @cssvar {font-weight} [--flow-data-badge-value-font-weight=normal]
* @cssvar {color} [--flow-data-field-value=#333]
*/
export class FlowDataBadge extends BaseElement {
	static get properties() {
		return {
			disabled:{type:Boolean, reflect:true},
			title:{type:String},
			prefix : { type : String },
			suffix:{type:String},
			align:{type:String}
		}
	}

	static get styles(){
		return css`
			:host{
				display:inline-block;
				font-weight:bold;
				font-size:13px;
				text-transform:uppercase;
				cursor:pointer;
				font-family:var(--flow-data-badge-font-family, "Julius Sans One");
				font-weight:var(--flow-data-badge-font-weight, bold);
				width:var(--flow-data-badge-width);
				min-width:var(--flow-data-badge-min-width);
				max-width:var(--flow-data-badge-max-width);
				margin:var(--flow-data-badge-margin);
			}
			:host([disabled]){opacity:0.5;cursor:default;pointer-events:none;}
			.colon{display:none}
			:host(.has-colon) .colon{display:inline;}
			.container{
				white-space: nowrap;
				border: var(--flow-data-badge-container-border, 2px) solid var(--flow-primary-color,#333);
				background-color:var(--flow-data-badge-bg, inherit);
				xdisplay:flex;xflex-firection:column;xalign-items:center;
				padding:var(--flow-data-badge-container-padding,2px 6px);
				margin: var(--flow-data-badge-container-margin, 6px);
				box-shadow:var(--flow-data-badge-container-box-shadow, 2px 2px 1px rgba(1, 123, 104, 0.1));
				border-radius:var(--flow-data-badge-container-border-radius, 10px);

			}
			.container>div{padding:2px;}
			.title{
				text-align:left; 
				opacity:var(--flow-data-badge-title-opacity,1);
				xmargin-top:7px; 
				font-size: var(--flow-data-badge-title-font-size, 10px); 
				color:var(--flow-data-badge-caption);
				xtext-shadow: 0px 0px 0px var(--flow-data-badge-caption-shadow, #fff); }
			.value{
				text-align:right;opacity:1;
				font-size:var(--flow-data-badge-value-font-size,14px);
				font-family:var(--flow-data-badge-value-font-family,"Exo 2");
				font-weight:var(--flow-data-badge-value-font-weight,normal);
				
			}
			.prefix{opacity:0.9;margin-right:3px;margin-top:3px; font-size: 10px; }
			.suffix{opacity:0.9;margin-left:3px;margin-top:3px; font-size: 10px; }
			.col { display: flex; flex-direction: column; align-items: left; }
			.row { display: flex; flex-direction: row; color: var(--flow-data-field-value,#333); }
		`;
	}

	render() {
		return html
		`<div class="container col">
			<div class="title">${this.title}<span class="colon">:</span></div>
			<div class="row">
				${ (!this.align || this.align == 'right') ? html`<div style="flex:1;"></div>` : '' }
				<div class="prefix">${this.prefix}</div>
				<div class="value"><slot></slot></div>
				<div class="suffix">${this.suffix}</div>
				${ (this.align == 'left') ? html`<div style="flex:1;"></div>` : '' }
			</div>
		</div>`;	
	}
}

//FlowDataBadge.define('flow-data-badge');
FlowDataBadge.define('flow-data-badge');


"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_DATA_BADGE_GRAPH_366 : &'static str = r###"




/**
* @class FlowDataBadgeGraph
* @extends Flowd3Element
* @prop {Boolean} disabled
* @prop {String} title
* @prop {String} prefix
* @prop {String} suffix
* @prop {String} align
* @prop {Number} value
* @prop {String} sampler
* @prop {String} type
* @prop {Number} range
* @cssvar {font-family} [--flow-data-badge-font-family="Julius Sans One"]
* @cssvar {font-weight} [--flow-data-badge-font-weight=bold]
* @cssvar {color} [--flow-data-badge-caption]
* @cssvar {color} [--flow-data-field-value=#333]
* @example
*   <flow-data-badge-graph title="text">value</flow-data-badge-graph>
*
*/
export class FlowDataBadgeGraph extends Flowd3Element {
	static get properties() {
		return {
			disabled:{type:Boolean, reflect:true},
			title:{type:String},
			prefix : { type : String },
			suffix:{type:String},
			align:{type:String},
			value:{type:Number},
			sampler:{type:String},  // sampler: 'kaspad.kd0.info.
			type:{type:String},
			range:{type:Number},
		}
	}

	static get styles(){
		return [Flowd3Element.styles, css`
			:host{
				display:inline-flex;
				font-weight:bold;
				font-size:13px;
				text-transform:uppercase;
				cursor:pointer;
				font-family:var(--flow-data-badge-font-family, "Julius Sans One");
				font-weight:var(--flow-data-badge-font-weight, bold);
				border-radius: 10px;
				overflow: hidden;

				/*width:300px;height:300px;*/
			}
			:host([disabled]){opacity:0.5;cursor:default;pointer-events:none;}
			.colon{display:none}
			:host(.has-colon) .colon{display:inline;}

			:host([.large]) { 
			}

			.container{
				white-space: nowrap;
				xdisplay:flex;xflex-firection:column;xalign-items:center;
				padding:2px 6px;
				/*min-height: inherit;*/
			}
			.container>div{padding:2px;}
			.title{flex:1; text-align:left; opacity:1;xmargin-top:7px; font-size: 10px; color: var(--flow-data-badge-caption); xtext-shadow: 0px 0px 0px var(--flow-data-badge-caption-shadow, #fff); }
			.value{text-align:right; opacity:1;font-size:14px;font-family:"Exo 2";font-weight:normal;}
			.prefix{opacity:0.9;margin-right:3px;margin-top:3px; font-size: 10px; }
			.suffix{opacity:0.9;margin-left:3px;margin-top:3px; font-size: 10px; }
			.col { display: flex; flex-direction: column; align-items: left;  }
			.row { display: flex; flex-direction: row; flex:0; color: var(--flow-data-field-value,#333); }


			.wrapper {
				/*width:100%;height:100%;*/
				position:relative;
				flex:1;
				margin:6px;overflow:hidden;
				border: 2px solid var(--flow-primary-color,#333);
				box-shadow: 2px 2px 1px rgba(1, 123, 104, 0.1);
				border-radius: 10px;
				/*
				min-width: var(--flow-data-badge-graph-width,240px);
				min-height: var(--flow-data-badge-graph-height,80px);				
				*/				
			}
			.wrapper > div {
				width:100%;height:100%;
				position:relative;left:0px;top:0px;bottom:0px;right:0px;
				/*display: flex;
				flex-direction: column;*/
				/*min-height: inherit;*/
			}

			.d3-holder{
				min-height:10px;
				min-width:10px;
				opacity:1;
				border-radius:10px;
				/*border: 1px solid red;*/
				/*margin: 0px -5px 0px -1px;
				z-index: 100;*/
			}

			.wrapper>div.d3-holder{position:absolute;}

		`];
	}

	constructor() {
		super();
		this.range = 60 * 5;
		this.refresh = 1e3;
		//this.svgViewBox = [-1, 0, 100, 50]
		//this.svgPreserveAspectRatio = 'xMidYMid meet';
		this.svgPreserveAspectRatio = 'xMaxYMax meet';
	}

	connectedCallback() {
		super.connectedCallback();
		if(this.sampler)
			this.interval = setInterval(this.draw.bind(this), this.refresh);
	}

	disconnectedCallback() {
		super.disconnectedCallback();

		if(this.interval)
			clearInterval(this.interval);
		if(this._draw){
			let sampler = FlowSampler.get(this.sampler || 'test-sampler');
			sampler.off('data', this._draw);
			this._draw = null;
		}
	}

	onElementResize(){
		super.onElementResize();
		dpc(()=>{
			this.draw();
		})
	}

	render() {

		dpc(()=>{
			this.draw();
		})

		return html
		`
		<div class='wrapper'>
			<div class="d3-holder">${super.render()}</div>
			<div>
				<div class="container col">
					<div class="title">${this.title}<span class="colon">:</span></div>
					<div class="row">
						${ (!this.align || this.align == 'right') ? html`<div style="flex:1;"></div>` : '' }
						<div class="prefix">${this.prefix}</div>
						<div class="value"><slot></slot></div>
						<div class="suffix">${this.suffix}</div>
						${ (this.align == 'left') ? html`<div style="flex:1;"></div>` : '' }
					</div>
				</div>
			</div>
		</div>
		`;	
	}

	getMargin(){
		return {
			bottom:0,
			top:0,
			left:0,
			right:0
		}
	}


	draw(){
        if(!this.sampler)
			return;

        let sampler = FlowSampler.get(this.sampler || 'test-sampler');
		if(!this._draw){
			this._draw = this.draw.bind(this);
			sampler.on('data', this._draw);
		}

        const { data } = sampler;
		this.redraw(data);
	}

	redraw(data){

		let margin = this.getMargin();
		let {height, width} = this.el_d3.getBoundingClientRect();

		//console.log("data", data)

		let [min,max] = d3.extent(data, d => d.date);
		//console.log("processing min-max[1]",min,max);
		min = max - 1000*this.range;//@anton why we are extending this min?


		const x = d3.scaleUtc()
		.domain([min,max])
		//.domain(d3.extent(data, d => d.date))//.nice()
		.range([margin.left, width - margin.right])

		const y = d3.scaleLinear()
		//.domain([min,max])//.nice()
		.domain(d3.extent(data, d => d.value))//.nice()
		//.domain([0, d3.max(data, d => d.value)]).nice()
		.range([height - margin.bottom, margin.top]);
		/*
		const xAxis = g => g
		.attr("transform", `translate(0,${height - margin.bottom})`)
		//.call(d3.axisBottom(x).ticks(width / 80).tickSizeOuter(0));

		const yAxis = g => g
		.attr("transform", `translate(${margin.left},0)`)
		.call(d3.axisLeft(y))
		.call(g => g.select(".domain").remove())
		// .call(g => g.select(".tick:last-of-type text").clone()
		// 	.attr("x", 3)
		// 	.attr("text-anchor", "start")
		// 	.attr("font-weight", "bold")
		// 	.text(this.title));
		*/

		const area = d3.area()
			.curve(d3.curveLinear)
			.x(d => x(d.date))
			.y0(y(0))
			.y1(d => y(d.value));

		const { el } = this;

		if(!this.path)
			this.path = el.append('path')
				.attr("transform", `translate(${margin.left},0)`)
				.attr('stroke-opacity', 'var(--flow-data-badge-graph-stroke-opacity, 1.0)')
				.attr("stroke-linejoin", "round")
				.attr("stroke-linecap", "round")
				.attr("stroke-width", 'var(--flow-data-badge-graph-stroke-width, 0)')
				.attr('fill','var(--flow-data-badge-graph-fill, steelblue)')
                .attr('stroke','var(--flow-data-badge-graph-stroke, #000)')
                
				
		this.path.datum(data)
			.attr('d',area);
	}
}

FlowDataBadgeGraph.define('flow-data-badge-graph');

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_DIALOG_360 : &'static str = r###"
//import '../resources/extern/dialog/dialog-polyfill.js';


/**
* @class FlowDialog
* @extends BaseElement
* @example
*   <flow-dialog heading="Title">value</flow-dialog>
* @property {Boolean} [disabled] 
* @property {String} [heading] 
* @property {Array} [btns] 
* @property {Object} [body] 
*/
export class FlowDialog extends BaseElement {
	static get properties() {
		return {
			disabled:{type:Boolean, reflect:true},
			heading:{type:String},
			btns:{type:Array},
			body:{type:Object},
			hideCloseBtn:{type:Boolean},
			compact:{type:Boolean}
		}
	}

	static buildArgs(...arg){
		let args = arg.shift();
		let handler = arg[arg.length-1];
		if(typeof(args) == 'string'){
			args = {
				title:args,
				body:arg.shift(),
				cls:arg.shift(),
				btns:arg.shift(),
				modal:arg.shift(),
			};
		}
		args.handler = args.handler || args.callback;
		args.modal = args.modal !== false;

		if(!args.handler && typeof(handler) == 'function')
			args.handler = handler

		//console.log("args", args)

		return args;
	}

	static getHandler(btnName,  args){
		let {btns, handler} = args;
		if(!btnName || !btns || !btns.length)
			return handler;

		let btn = btns.find(btn=>((btn.value||btn.text)+"").toLowerCase() == btnName);
		return btn? (btn.handler || btn.callback || handler):handler;
	}

	static _show(args){

		let {btns, body, title, modal, cls, hideCloseBtn, compact, alignTo} = args;
		let {autoClose} = args;
		let dg = document.createElement("flow-dialog");
		let promise = new Promise((resolve, reject)=>{
			let resolved = false;
			let onWindoClick = (e)=>{
				if(resolved){
					removeWinEventListener();
					return
				}
				let t = e.target;
				let menu = t && t.closest && t.closest("flow-dialog");
				if(menu != dg){
					removeWinEventListener();
					dg.destroy();
				}
			}
			let onWindoResize = e=>{
				//console.log("onWindoResize22: dg.dialog", dg._isPolyfill)
				if(dg._isPolyfill){
					//console.log("onWindoResize: dg.dialog", dg.dialog)
					dialogPolyfill.reposition(dg.dialog)
				}
			}
			let removeWinEventListener = ()=>{
				window.removeEventListener("click", onWindoClick)
				window.removeEventListener("resize", onWindoResize)
			}
			let _resolve = (result)=>{
				if(resolved)
					return
				resolved = true;
				dg.remove();
				dg.removeEventListener("btn-click", onBtnClicked);
				removeWinEventListener();
				resolve(result);
			}
			let onBtnClicked = e=>{
				let result = e.detail;
				let {btn} = result;
				let handler = this.getHandler(btn, args);
				if(handler)
					return handler(_resolve, result, dg, btn, e);

				dg.resolve(result);
			}
			dg.resolve = _resolve;

			dg.addEventListener("btn-click", onBtnClicked)
			if(cls)
				dg.classList.add(...cls.split(" "));
			if(btns)
				dg.btns = btns;
			if(body)
				dg.body = body;
			if(title)
				dg.heading = title;
			if(hideCloseBtn)
				dg.hideCloseBtn = true;
			if(compact)
				dg.compact = true;
			if(alignTo)
				this.alignTo(alignTo, dg, args);


			document.body.append(dg)
			setTimeout(()=>{
				modal?dg.showModal():dg.show();
				if(autoClose){
					window.addEventListener("click", onWindoClick)
				}
				//console.log("onWindoResize:1")
				window.addEventListener("resize", onWindoResize)

			}, 100)
			
		})

		promise.dialog = dg;
		return promise
	}

	static alignTo(alignTarget, dialog, args){
		let {vOffset=0, hOffset=0, targetPos='left-bottom', dialogPos='left-top'} = args;
		let box = alignTarget.getBoundingClientRect();
		let dialogBox = dialog.getBoundingClientRect();
		let style = dialog.style;
		let [H,V] = targetPos.split("-");
		let [dH, dV] = dialogPos.split("-");
		let dVOpposite = dV=='top'?'bottom':'top';
		let dHOpposite = dH=='left'?'right':'left';
		style[dVOpposite] = 'unset';
		style[dHOpposite] = 'unset';
		let setPos = ()=>{
			style[dH] = (box[H]+hOffset)+"px";
			style[dV] = (box[V]+vOffset)+"px";
			
			/*
			if(targetPos == 'right-top'){
				
			}else{
				style.top = (box.bottom+vOffset)+"px";
				style.left = (box.right-dialogBox.width+hOffset)+"px";
			}
			*/
		}

		setPos();
		dialog.addEventListener("updated", e=>{
			let {dialog} = e.detail
			dialogBox = dialog.getBoundingClientRect();
			style = dialog.style;
			setPos();
		})
	}

	static alert(...args){
		args = this.buildArgs(...args)
		if(!args.btns){
			args.btns = ['Ok:primary']
		}
		return this._show(args)
	}
	static show(...args){
		return this._show(this.buildArgs(...args))
	}

	static confirm(...args){
		args = this.buildArgs(...args);
		if(!args.btns){
			args.btns = ['Cancel', 'Yes:danger']
		}

		return this._show(args);
	}

	createRenderRoot(){
		return this;
	}

	render() {
		return html
		`<dialog @close=${this.onDialogClose} ?compact=${this.compact}>
			<div class="heading" ?hide=${!this.heading}>${this.heading}</div>
			<span class="close-btn" title="Close" ?hide=${this.hideCloseBtn}
				@click="${this.onCloseClick}">&times;</span>
			<div class="body">
				${this.renderBody()}
			</div>
			<div class="buttons" @click=${this.onBtnClick} ?hide=${!this.btns||!this.btns.length}>
				${this.renderBtns()}
			</div>
		</dialog>`;	
	}

	renderBody(){
		return this.body||"";
	}

	renderBtns(){
		let value, text, cls;
		return (this.btns || ['Ok'])
		.map(b=>{
			if(typeof(b)=='string'){
				let [t, c, v] = b.split(":");
				text = t;
				value = v || text;
				cls = c||'';
			}else{
				text = b.text;
				value = b.value || text;
				cls = b.cls||"";
			}
			return html
			`<flow-btn 
				class="${cls}" 
				value="${(value+"").toLowerCase()}">${text}</flow-btn>`
		})
	}

	firstUpdated(){
		this.dialog = this.renderRoot.querySelector('dialog');
		this._isPolyfill = !this.dialog.showModal
		dialogPolyfill.registerDialog(this.dialog)
		if(this._show)
			this[this._show]();
	}

	updated(){
		super.updated();
		this.dispatchEvent(new CustomEvent('updated', {detail:{dialog:this.dialog}, bubbles:true}))
	}

	show(){
		if(this.dialog)
			return this.dialog.show()

		this._show = 'show';
	}
	showModal(){
		if(this.dialog)
			return this.dialog.showModal();
		this._show = 'showModal';
	}

	close(){
		this._show = false;
		if(this.dialog)
			this.dialog.close();
	}

	destroy(){
		this.close();
		this.remove();
	}

	onCloseClick(){
		if(this.resolve)
			return this.resolve({btn:"close"})
		this.destroy();
	}

	onDialogClose(e){
		if(!this.autoClose && this._show){
			this[this._show]();
			return
		}
		let detail = {e};
		this.dispatchEvent(new CustomEvent('closed', {detail}))
	}
	onBtnClick(e){
		let btnEl = e.target.closest("flow-btn");
		let btn = btnEl?.getAttribute("value");
		if(!btn)
			return
		let inputs = [...this.renderRoot.querySelectorAll(".input, flow-input, flow-checkbox, input, textarea, select,flow-menu")];
		let values = {}, name;
		inputs.forEach(input=>{
			name = input.name||input.getAttribute("name")||input.getAttribute("data-name");
			values[name] = input.value;
		})
		let detail = {
			btn,
			values
		}

		//console.log("onBtnClick", detail)
		this.dispatchEvent(new CustomEvent('btn-click', {detail}))
	}
}

window.FlowDialog = FlowDialog;

FlowDialog.define('flow-dialog', [
	()=>window.dialogPolyfill?null:baseUrl+'resources/extern/dialog/dialog-polyfill.css',
	()=>window.dialogPolyfill?null:baseUrl+'/resources/extern/dialog/dialog-polyfill.js'
]);

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_PAGES_416 : &'static str = r###"


let flowPagesStyle = css`
	flow-pages>h1,
	flow-pages>.title{
	    padding:10px;
	    font-size:2rem;
	}
	flow-pages .buttons{margin:10px;display:flex;justify-content:flex-end;z-index:10}
	flow-pages .buttons .flex{flex:1;}
	flow-pages .buttons flow-btn{margin:0px 5px;padding:5px 5px;user-select:none;}
	flow-pages .buttons flow-btn svg{
	    width:20px;
	    height:20px;
	    margin-right:10px;
	    fill:var(--flow-primary-color, rgba(0,151,115,1.0));
	    pointer-events:none;
	}
	flow-pages .buttons flow-btn span+svg{
	    margin-left:10px;
	    margin-right:0px;
	}
`

let style = document.head.querySelector('style.flow-pages-style') || document.createElement("style");
style.innerHTML = flowPagesStyle.toString();
style.classList.add("flow-pages-style");
if(!style.parentNode)
	document.head.insertBefore(style, document.head.querySelector('link[href*="flow-ux.css"], :last-child').nextSibling);
export {flowPagesStyle};

/**
 * @export
 * @class FlowPages
 * @extends {BaseElement}
 * 
 * @property {Array} pages
 * @property {Number} index 
 * 
 * @cssvar {fill|background-color} [--flow-primary-color=rgba(0,151,115,1)]
 * @cssvar {background-color} [--flow-background-color=#FFF]
 * @cssvar {box-shadow} [--flow-pages-dots-box-shadow=var(--flow-box-shadow)]
 * @cssvar {border} [--flow-border-color=var(--flow-primary-color, rgba(0,151,115,1))]
 * @cssvar {border-color} [--flow-active-border-color=var(--flow-primary-color, rgba(0,151,115,1))]
 * 
 * @example
 * <flow-pages>
 * 	<flow-page>Page 1</flow-page>
 *  <flow-page>Page 2</flow-page>
 * </flow-pages>
 *
 */

export class FlowPages extends BaseElement {
	static get properties() {
		return {
			pages:{type:Array},
			index:{type:Number},
			dotoffset:{type:Number}
		}
	}
	static get styles() {
		return css`
			:host{
				display:flex;
				flex-direction:column;
			}

			.wrapper{
				flex:1;
				position:relative;
			}

			.wrapper ::slotted(flow-page){
				background-color:var(--flow-background-color, #FFF);
				position:absolute;
				left:0px;
				top:0px;
				width:100%;
				height:100%;
				z-index:1;
				opacity:0;
				transition:opacity 1s ease;
			}
			.wrapper ::slotted(flow-page.back),
			.wrapper ::slotted(flow-page.active){
				z-index:3;
				opacity:1;
			}

			.dots{
				pointer-events: none;
				z-index:5;
				position:absolute;bottom:10px;
				display:none;
				justify-content:center;
				width:100%;
			}
			:host(.has-dots) .dots{
				display:flex;
			}
			.dots i{
				display:block;width:10px;height:10px;background-color:#FFF;
				box-shadow:var(--flow-pages-dots-box-shadow, var(--flow-box-shadow));
				margin:4px;
				border:2px solid var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-radius:50%;
			}

			.dots i.active{
				background-color:var(--flow-primary-color, rgba(0,151,115,1));
				border-color:var(--flow-active-border-color, var(--flow-primary-color, rgba(0,151,115,1)));
			}

			.dots i:not(.active){cursor:pointer;}


			.buttons flow-btn {
				align-items:center;
				display:flex;
			}


		`;
	}
	render(){
		let dots = new Array((this.pages || []).length).fill(0);
		if(dots.length)
			dots[this.index||0] = 1;

		let css = '';
		if(this.dotoffset)
			css = `bottom: -${this.dotoffset}px`;

		return html`
		<slot name="title"></slot>
		<div class="wrapper">
			<slot></slot>
			<div class="dots" style="${css}" @click="${this.onDotsClick}">${dots.map((active,i)=>{
				return html`<i data-index="${i}" class="${active?'active':''}"></i>`
			})}</div>
		</div>
		
		<div @click="${this.onButtonClick}">
			<slot name="buttons"></slot>
		</div>
		`;
	}
	firstUpdated(){
		this.wrapper = this.renderRoot.querySelector(".wrapper");
		this.buttons = this.renderRoot.querySelector(".buttons");
		let slot = this.shadowRoot.querySelector('slot[name="buttons"]');
		this.btns = {};
		slot.addEventListener('slotchange', e=>{
			this.updateBtns(slot.assignedNodes());
			
		});
		this.updateBtns(slot.assignedNodes());

		let pages = this.querySelectorAll("flow-page");
		this.initiPages(pages);
	}
	updateBtns(nodes){
		[...nodes].forEach(p=>{
			p.querySelectorAll("[data-btn]").forEach(btn=>{
				let name = btn.getAttribute("data-btn");
				if(name)
					this.btns[name] = btn;
			})
		})
	}
	get nextBtn(){
		return this.btns.next;
	}
	get prevBtn(){
		return this.btns.prev;
	}
	get skipBtn(){
		return this.btns.skip;
	}
	initiPages(pages){
		this.pages = [...pages];
		this.maxIndex = this.pages.length-1;
		let index = this.pages.findIndex(p=>p.classList.contains("active"))
		this.setActive(index)
	}
	onDotsClick(e){
		let target = e.target;
		let index = parseInt(target.getAttribute("data-index"));
		if(isNaN(index))
			return

		this.setActive(index);
	}
	onButtonClick(e){
		let btnTypeToAction ={
			'next': 'showNext',
			'prev': 'showPrevious'
		}
		let target = e.target.closest('flow-btn');
		if(!target)
			return
		let action = target.getAttribute("data-action");
		if(!action){
			let btnType = target.getAttribute("data-btn");
			action = btnType && btnTypeToAction[btnType];
		}
		if(!action || !this[action])
			return

		this[action]();
	}
	showPrevious(){
		this.setActive(this.index-1)
	}
	showNext(){
		this.setActive(this.index+1)
	}
	closePages(){
		this.fire("close-pages");
	}
	setActive(index){
		if(index<0)
			index = 0
		else if(index > this.maxIndex){
			this.closePages();
			return
		}
		
		let newPage = this.getPage(index)
		if(!newPage)
			return
		this.lastIndex = this.index;
		this.index = index;
		if(this.index === this.lastIndex)
			return
		let lastPage = this.getPage(this.lastIndex)
		if(lastPage){
			lastPage.classList.remove("active");
			lastPage.style.zIndex = 2
		}

		newPage.classList.add("active");
		newPage.style.zIndex = 3

		//console.log("index", index, this.maxIndex)
		let prevBtn = this.prevBtn;
		let nextBtn = this.nextBtn;
		let skipBtn = this.skipBtn;
		let nextBtnSpan = nextBtn?.querySelector("span");
		if(prevBtn){
			if(index<=0)
				prevBtn.setAttribute("disabled", true);
			else
				prevBtn.removeAttribute("disabled");

			if(nextBtnSpan)
				nextBtnSpan.innerText = 'NEXT';				
			skipBtn.style.display = "block";
		}
		if(nextBtn){
			if(index>=this.maxIndex) {
				skipBtn.style.display = "none";
				if(nextBtnSpan)
					nextBtnSpan.innerText = 'FINISH';
			}
			else
				nextBtn.removeAttribute("disabled");
		}
		this.fireChangeEvent();
	}
	getPage(index){
		return this.pages[index];
	}
	fireChangeEvent(){
		this.fire("change", {index: this.index})
	}
}

FlowPages.define('flow-pages');



"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_I18N_379 : &'static str = r###"


export const i18nDirMap = new Map();
export const i18nElementsMap = new Map();

class i18n extends BaseElement{

	static hash(text) {
        var A = 5381,
            B = 9835,
            i    = text.length;
        while(i) {
            var ch = text.charCodeAt(--i);
            A = (A * 33) ^ ch;
            B = (B * 55) ^ ch ^ A;
        }
        A = A >= 0 ? A : (A & 0x7FFFFFFF) + 0x80000000;
        B = B >= 0 ? B : (B & 0x7FFFFFFF) + 0x80000000;
        return A.toString(16)+B.toString(16);
    }

    static entriesChanged(){
    	let entriesAll = this.getAllEntries();
    	let entries = this.getEntries();
    	let detail = {entries, entriesAll}
    	let ce = new CustomEvent("flow-i18n-entries-changed", {detail})
    	window.dispatchEvent(ce);
    }

	static stripWhitespace(text) {
		return text.replace(/\s\s+/g,' ').trim();
	}

    static createEntry(text, values=false){
		let hash = this.hash(text);
		//console.log("i18n.entries", this.entries, text)
		if(!this.entries[hash]){
			this.entries[hash] = Object.assign({en:text}, values || {});
			this.entriesChanged();
		}else if(values){
			Object.assign(this.entries[hash], values);
			this.entriesChanged();
		}
		return hash;
	}

	static t(text, defaults, locale){
		text = this.stripWhitespace(text);
		let hash = this.createEntry(text);
		if(this.testing)
			return `[${locale||this.locale}:${text}]`;
		let entry = this.entries[hash];
		let value = entry[locale||this.locale];
		//console.log("value", value, this.locale, text)
		//if(value !== undefined)
		//	return value;
		return value || defaults || text;
	}

	static setConfig(_config){
		config = _config;
	}
	static getConfig(){
		return config;
	}
	static setActiveLanguages(locales){
		let newLocale = false;
		config.languages.forEach(l=>{
			l.active = locales.includes(l.locale);
			if(l.locale == this.locale && !l.active)
				newLocale = 'en';
		});

		if(newLocale)
			this.setLocale(newLocale);
	}
	static getActiveLanguages(){
		return config.languages.filter(l=>l.active)
	}
	static getActiveLocales(){
		return this.getActiveLanguages().map(l=>l.locale);
	}
	static setEntries(list){
		(list ||[]).forEach(entry=>{
			this.entries[this.hash(entry.en)] = entry;
		});

		onLocaleChange();
	}
	static setTesting(testing){
		this.testing = testing;
		onLocaleChange();
	}
	static getEntries(){
		let activeLocales = this.getActiveLocales();
    	let entry;
    	return this.getAllEntries().map(e=>{
    		entry = {en:e.en};
    		activeLocales.forEach(l=>{
    			entry[l] = e[l] || "";
    		})
    		return entry;
    	})
	}
	static getAllEntries(){
		return Object.values(this.entries);
	}
	static get properties() {
		return {
			text:{type:String}
			//xx:{type:String}
		}
	}

	static setLocale(locale='en'){
		this.locale = locale;
		this.setLocalSetting('i18n-locale', locale);
		onLocaleChange();
	}

	static cleanText(text){
		return text
		.replace(/<\!\-\-\?lit([^>]*)\$-\->/g, "")
		.replace(/<\!\-\-\-\->/g, "")
	}

	constructor(){
		super();
		this.text = "";
		//this.setAttribute("c1", `${this.text}`)
		//console.log("constructor:", this.innerHTML, this.text)
	}

	createRenderRoot() {
		//this.innerHTML_ = this.innerHTML;
		return this;
	}

	connectedCallback(){
		super.connectedCallback();
		this._cb = this._cb || this.onLocaleChange.bind(this);
		window.addEventListener("flow-i18n-locale", this._cb)
		//this.setAttribute("c2", `${this.text}`)
		this.update();
	}

	disconnectedCallback(){
		super.disconnectedCallback();
		this._cb && window.removeEventListener("flow-i18n-locale", this._cb)
		//TODO MAYBE:: _parts.delete(this.renderRoot)
	}

	onLocaleChange(){
		this.update();
	}

	/*
	updated(changed){
		super.updated();
		console.log("changed", changed)
		this.setAttribute("c3", `${this.text}:${JSON.stringify(changed)}`)
	}
	*/

	render() {
		if(this.innerHTML_ == undefined){
			this.innerHTML_ = i18n.cleanText(this.innerHTML)
			this.innerHTML = "";
		}
		//if(this.getAttribute("xx") == 1)
		//this.log("innerHTML", i18n.locale, "inner:"+this.innerHTML_, " text:"+this.text)
		let strings = [i18n.t(this.text || this.innerHTML_)];
		strings.raw = [];
		return html(strings);
	}  
}


let config = {
	// languages:[
	// 	{title:'English', locale:'en'},
	// 	{title:'Russian', locale:'ru'},
	// 	{title:'Hindi', locale:'hi'},
	// 	{title:'Punjabi', locale:'pu'}
	// ]
	languages : [
		{"title":"العربية‎", "locale": "ar", rtl: true },
		{"title":"Български", "locale": "bg" },
		{"title":"বাংলা", "locale": "bn" },
		{"title":"English", "locale": "en" },
		{"title":"Español", "locale": "es" },
		{"title":"Greek", "locale": "el" },
		{"title":"Esti", "locale": "et" },
		{"title":"Français", "locale": "fr" },
		{"title":"Deutsch", "locale": "de" },
		{"title":"Danish", "locale": "da" },
		{"title":"Czech", "locale": "cs" },
		{"title":"Farsi", "locale": "fa" },
		{"title":"Finnish", "locale": "fi" },
		{"title":"Filipino", "locale": "fil" },
		{"title":"עברית", "locale": "he", rtl: true },
		{"title":"Hindi", "locale": "hi" },
		{"title":"Croatian", "locale": "hr" },
		{"title":"Hungarian", "locale": "hu" },
		{"title":"Italiano", "locale": "it" },
		{"title":"Icelandic", "locale": "is" },
		{"title":"日本語", "locale": "ja" },
		{"title":"Korean", "locale": "ko" },
		{"title":"Korean", "locale": "kr" },
		{"title":"Lithuanian", "locale": "lt" },
		{"title":"Norwegian", "locale": "nb" },
		{"title":"Dutch", "locale": "nl" },
		{"title":"Norwegian", "locale": "no" },
		{"title":"Polski", "locale": "pl" },
		{"title":"PortuguÃªs", "locale": "pt" },
		{"title":"Romanian", "locale": "ro" },
		{"title":"Русский", "locale": "ru" },
		{"title":"Slovak", "locale": "sk" },
		{"title":"Serbian", "locale": "sr" },
		{"title":"Slovenian", "locale": "sl" },
		{"title":"Swedish", "locale": "sv" },
		{"title":"Tamil", "locale": "ta" },
		{"title":"Thai", "locale": "th" },
		{"title":"Turkish", "locale": "tr" },
		{"title":"Ukrainian", "locale": "uk" },
		{"title":"Urdu", "locale": "ur" },
		{"title":"Vietnamese", "locale": "vi" },
		{"title":"Mongolian", "locale": "mn" },
		{"title":"中文", "locale": "zh_HANS" },
		{"title":"繁體中文", "locale": "zh_HANT" }
	],
	aliases : {
		"en-GB": "en",
		"en-US": "en",
		"zh-CN": "zh_HANS",
		"zh-TW": "zh_HANT"
	}

}

i18n.entries = {};
i18n.locale = BaseElement.getLocalSetting('i18n-locale', 'en');
i18n.entries[i18n.hash('Hello')] = {
	en: "Hello",
	ru : 'Ð-Ð´Ñ€Ð°Ð²ÑÑ‚Ð²ÑƒÐ¹Ñ‚Ðµ',
	pu: 'ਸਤ ਸ੍ਰੀ ਅਕਾਲ',
	hi: 'नमस्ते!'
}

window.addEventListener("flow-i18n-entries", e=>{
	i18n.setEntries(e.detail.entries);
})

let Mixin = (Base, tag)=>{
	class Child extends Base{
		connectedCallback() {
			if(!this.innerHTML_)
				this.innerHTML_ = i18n.cleanText(this.innerHTML);
			this._cb = this._cb || this.onLocaleChange.bind(this);
			window.addEventListener("flow-i18n-locale", this._cb)
			this.onLocaleChange();
		}

		disconnectedCallback(){
			this._cb && window.removeEventListener("flow-i18n-locale", this._cb)
		}

		onLocaleChange(){
			this.innerHTML = this.htmlToElement(i18n.t(this.innerHTML_))
		}
		htmlToElement(html) {
		    let template = document.createElement('template');
		    template.innerHTML = `<span>${html.trim()}</span>`;
		    return template.content.firstChild.innerHTML;
		}
	}

	customElements.define('i18n-'+tag, Child, {extends: tag});

}

Mixin(HTMLDivElement, 'div');
Mixin(HTMLSpanElement, 'span')
Mixin(HTMLParagraphElement, 'p')
Mixin(HTMLLabelElement, 'label')
Mixin(HTMLTableCellElement, 'td')
Mixin(HTMLTableCellElement, 'th')
Mixin(HTMLAnchorElement, 'a')

export const buildLitHTML = (...strParts)=>{
	let strings = [...strParts];
	strings.raw = [];
	return html(strings);
}

export const i18nFormat = (str, ...values)=>{
	str = i18n.t(str);
	values.forEach(n=>{
		str = str.replace('[n]', n)
	})
	return str;
}

export const i18nHTMLFormat = (str, ...values)=>{
	str = i18n.t(str);
	values.forEach(n=>{
		str = str.replace('[n]', n)
	})
	return buildLitHTML(str);
}

class I18nDirective extends AsyncDirective{
	constructor(...args){
		super(...args);
		i18nDirMap.set(this, {});
	}
	render(text) {
		this.__text = text;
		return i18n.t(text);
	}

	disconnected() {
		i18nDirMap.delete(this);
	}

	reconnected() {
		i18nDirMap.set(this, {});
	}
}

const T = directive(I18nDirective);

let onLocaleChange = ()=>{
	let ce = new CustomEvent("flow-i18n-locale", {detail:{locale:i18n.locale}})
	window.dispatchEvent(ce);

	i18nDirMap.forEach((v, dir)=>{
		//console.log("dir", dir)
		dir.setValue(dir.__text?i18n.t(dir.__text):'')
	})

	i18nElementsMap.forEach((v, ele)=>{
		//console.log("dir", dir)
		ele.setI18nValue(ele.__i18nText?i18n.t(ele.__i18nText):'')
	})
}

i18n.setLocale(i18n.locale);


export class FlowI18nDialog{
	static open(alignTarget){
		if(this.dialog)
			return
		this._click = this._click || this.onClick.bind(this);
		
		let p = this._open(alignTarget);
		setTimeout(()=>{
			window.addEventListener("click", this._click)
		}, 100);
		return p;
	}
	static close(data){
		if(this.dialog){
			this.dialog.resolve(data);
			this.dialog = null;
		}
		this.removeEventListener();
	}
	static onClick(e){
		if(!this.dialog){
			this.removeEventListener();
			return
		}
		let t = e.target;
		let menu = t && t.closest && t.closest("flow-dialog.flow-menu");
		if(menu != this.dialog){
			this.removeEventListener();
			this.close();
		}
	}
	static removeEventListener(){
		window.removeEventListener("click", this._click)
	}
	static _open(alignTarget){
		let menuClick = e=>{
			let li = e.target.closest("li");
			let locale = li.getAttribute("data-locale");
			i18n.setLocale(locale);
			dialog.resolve({locale});
			this.dialog = false;
			this.removeEventListener();
		}

		let body = html
			`<ul class="menu" @click="${menuClick}">${
				config.languages.filter(l=>l.active).map(l=>html
					`<li data-locale="${l.locale}" 
						class="${l.locale==i18n.locale?'active':''}">${l.title}</li>`
				)
			}</ul>`

		let promise = FlowDialog.show({
			body,
			btns:[],
			cls:"flow-menu hide-close-btn",
			modal:false
		})

		let {dialog} = promise;
		this.dialog = dialog;
		if(alignTarget){
			let dialogBox = dialog.getBoundingClientRect();
			FlowDialog.alignTo(alignTarget, dialog, {
				targetPos:'right-bottom',
				dialogPos:'left-top',
				hOffset: -dialogBox.width,
				vOffset: 2
			})
		}

		return promise
	}
}


//window.xxxxx_setLocale = i18n.setLocale.bind(i18n)


export class I18nTest extends LitElement{
	static get styles(){
		return css`:host{display:inline-block;border:1px solid #DDD;padding:10px;}`
	}
	static get properties(){
		return {
			loop:{type:Boolean}
		}
	}
	constructor(){
		super();
		this.start();
	}
	start(){
		let i=0, ls = ['ru', 'pu', 'hi', 'en'];
		this.intervalId = setInterval(()=>{
			let l = ls[i++];
			if(!l){
				if(this.loop){
					i=0;
					l = ls[i++];
				}else{
					this.remove();
					clearInterval(this.intervalId);
				}
			}
			i18n.setLocale(l)
		}, 1000)
	}
	render(){
		return html`Hello: ${T('Hello')} <div title="${T('abc')}">How are You</div>`
	}
}

customElements.define('i18n-test', I18nTest);
customElements.define('flow-i18n', i18n);


export {i18n, T}



"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_CODE_408 : &'static str = r###"

if (!window.PR) {
	let prettify = document.createElement("script");
	prettify.src = baseUrl + 'resources/extern/google-prettify/prettify.js';
	document.head.appendChild(prettify);
}

/**
 * @export
 * @class FlowCode
 * @prop {Boolean} fixindent
 * @prop {String} lang
 * @extends {BaseElement}
 * @cssvar {white-space} [--flow-code-white-space=nowrap]
 * @cssvar {font-family} [--flow-code-font-family=monospace]
 * @cssvar {font-size} [--flow-code-font-size=1rem]	
 * @cssvar {padding} [--flow-code-padding=5px]
 * @cssvar {margin} [--flow-code-margin=1px]
 * @cssvar {border} [--flow-code-border=none]
 */
export class FlowCode extends BaseElement {
	static get properties() {
		return {
			lang : {type:String},
			fixindent:{type:Boolean},
			theme:{type:String}
		}
	}
	static get styles() {
		return css`

			.pln{
				color:var(--flow-code-pln, #000);
			}
			@media screen{
				.str{color:var(--flow-code-str, #080)}
				.kwd{color:var(--flow-code-kwd, #008)}
				.com{color:var(--flow-code-com, #800)}
				.typ{color:var(--flow-code-typ, #606)}
				.lit{color:var(--flow-code-lit, #066)}
				.opn{color:var(--flow-code-opn, #660)}
				.clo{color:var(--flow-code-clo, #660)}
				.pun{color:var(--flow-code-pun, #660)}
				.tag{color:var(--flow-code-tag, #008)}
				.atn{color:var(--flow-code-atn, #606)}
				.atv{color:var(--flow-code-atv, #080)}
				.dec{color:var(--flow-code-dec, #606)}
				.var{color:var(--flow-code-var, #606)}
				.fun{color:var(--flow-code-fun, red)}
			}
			@media print,projection{
				.kwd,.tag,
				.typ{font-weight:var(--flow-code-print-tag-font-weight, 700)}
				.str{color:var(--flow-code-print-str, #060)}
				.kwd{color:var(--flow-code-print-kwd, #006)}
				.com{
					color:var(--flow-code-print-com, #600);
					font-style:var(--flow-code-print-com-font-style, italic)
				}
				.typ{color:var(--flow-code-print-typ, #404)}
				.lit{color:var(--flow-code-print-lit, #044)}
				.opn{color:var(--flow-code-print-opn, #440)}
				.clo{color:var(--flow-code-print-clo, #440)}
				.pun{color:var(--flow-code-print-pun, #440)}
				.tag{color:var(--flow-code-print-tag, #006)}
				.atn{color:var(--flow-code-print-atn, #404)}
				.atv{color:var(--flow-code-print-atv, #060)}
			}
			pre{
				background:var(--flow-code-pre-bg);
			}
			pre.prettyprint{padding:2px;}
			ol.linenums{
				margin-top:var(--flow-code-linenums-margin-top, 0);
				margin-bottom:var(--flow-code-linenums-margin-bottom, 0);
				color:var(--flow-code-linenums-color, inherit);
			}
			li.L0,li.L1,li.L2,li.L3,li.L4,li.L5,li.L6,li.L7,li.L8,li.L9{
				list-style-type:none;
				padding-left:var(--flow-code-lines-padding-left, 0);
				background-color:var(--flow-code-lines-bg, initial);
			}
			li.L1,li.L3,li.L5,li.L7,li.L9{
				background:var(--flow-code-odd-line-bg, #eee)
			}

			pre{
				margin:0px;
				white-space:var(--flow-code-white-space, nowrap);
				font-family:var(--flow-code-font-family, monospace);
				font-size:var(--flow-code-font-size, 1rem);
				padding:var(--flow-code-pre-padding, 0px 0px 16px);
			}

			:host{
				display:inline-block;max-width:100%;box-sizing: border-box;
				overflow:auto;
				padding:var(--flow-code-padding, 5px);
				margin:var(--flow-code-margin, 1px);
				border:var(--flow-code-border, none);
				background:var(--flow-code-pre-bg);
			}

			:host(.block),
			:host([block]){display:block}
			:host(.hide){display:none}

			/*:host(:not(.no-border):not([no-border])){*/
			:host(.border, [border]){
				border:2px solid var(--flow-primary-color);
			}


			/* hemisu-light */
			/*
			pre.theme-hemisu-light{
				font-family:Menlo,Bitstream Vera Sans Mono,DejaVu Sans Mono,Monaco,Consolas,monospace;
				border:0!important
			}
			*/
			.theme-hemisu-light{
				--flow-code-pre-bg:#fff;
				--flow-code-lines-bg:#fff;
				--flow-code-pln:#111;
				--flow-code-linenums-color:#999;

				--flow-code-lines-padding-left:1em;
				--flow-code-linenums-margin-top:0;
				--flow-code-linenums-margin-bottom:0;

				--flow-code-str:#739200;
				--flow-code-kwd:#739200;
				--flow-code-com:#999;
				--flow-code-typ:#f05;
				--flow-code-lit:#538192;
				--flow-code-pun:#111;
				--flow-code-opn:#111;
				--flow-code-clo:#111;
				--flow-code-tag:#111;
				--flow-code-atn:#739200;
				--flow-code-atv:#f05;
				--flow-code-dec:#111;
				--flow-code-var:#111;
				--flow-code-fun:#538192;
			}


			.theme-hemisu-dark{
				--flow-code-pre-bg:#000000;
				--flow-code-lines-bg:#000000;
				--flow-code-pln:#EEEEEE;
				--flow-code-linenums-color:#777777;

				--flow-code-lines-padding-left:1em;
				--flow-code-linenums-margin-top:0;
				--flow-code-linenums-margin-bottom:0;

				--flow-code-str:#B1D631;
				--flow-code-kwd:#B1D631;
				--flow-code-com:#777777;
				--flow-code-typ:#BBFFAA;
				--flow-code-lit:#9FD3E6;
				--flow-code-pun:#EEEEEE;
				--flow-code-opn:#EEEEEE;
				--flow-code-clo:#EEEEEE;
				--flow-code-tag:#EEEEEE;
				--flow-code-atn:#B1D631;
				--flow-code-atv:#BBFFAA;
				--flow-code-dec:#EEEEEE;
				--flow-code-var:#EEEEEE;
				--flow-code-fun:#9FD3E6;
			}

			.theme-atelier-lakeside-dark{
				--flow-code-pre-bg:#161b1d;
				--flow-code-lines-bg:#161b1d;
				--flow-code-pln:#ebf8ff;
				--flow-code-linenums-color:#5a7b8c;

				--flow-code-lines-padding-left:1em;
				--flow-code-linenums-margin-top:0;
				--flow-code-linenums-margin-bottom:0;

				--flow-code-str:#568c3b;
				--flow-code-kwd:#6b6bb8;
				--flow-code-com:#6b6bb8;
				--flow-code-typ:#257fad;
				--flow-code-lit:#935c25;
				--flow-code-pun:#ebf8ff;
				--flow-code-opn:#ebf8ff;
				--flow-code-clo:#ebf8ff;
				--flow-code-tag:#d22d72;
				--flow-code-atn:#935c25;
				--flow-code-atv:#2d8f6f;
				--flow-code-dec:#935c25;
				--flow-code-var:#d22d72;
				--flow-code-fun:#257fad;
			}

			.theme-atelier-lakeside-light{
				--flow-code-pre-bg:#ebf8ff;
				--flow-code-lines-bg:#ebf8ff;
				--flow-code-pln:#161b1d;
				--flow-code-linenums-color:#7195a8;

				--flow-code-lines-padding-left:1em;
				--flow-code-linenums-margin-top:0;
				--flow-code-linenums-margin-bottom:0;

				--flow-code-str:#568c3b;
				--flow-code-kwd:#6b6bb8;
				--flow-code-com:#7195a8;
				--flow-code-typ:#257fad;
				--flow-code-lit:#935c25;
				--flow-code-pun:#161b1d;
				--flow-code-opn:#161b1d;
				--flow-code-clo:#161b1d;
				--flow-code-tag:#d22d72;
				--flow-code-atn:#935c25;
				--flow-code-atv:#2d8f6f;
				--flow-code-dec:#935c25;
				--flow-code-var:#d22d72;
				--flow-code-fun:#257fad;
			}

		`;
	}
	constructor() {
		super();
		this.lang = 'html';
	}
	render() {
		//let indent = this.clcIndent();
		if (!this.innerHTML_) {
			
			let ta = this.querySelector("textarea"); 
			let v = ta ? ta.value : this.innerHTML;
			if(this.fixindent){
				v = v.split("\n");
				let c = v[0]; 
				let count = 0;
				let spaces = true;
				while(spaces) {
					if(/^\t/.test(c)) {
						count++;
						c = c.substring(1); 
					} else if(/^    /.test(c)) {
						count++;
						c = c.substring(4);
					} else 
						spaces = false;
				}
				if(count>0){
					let regExp = `^(\t|    ){1,${count}}`;
					regExp = new RegExp(regExp)
					v = v.map(l => {
						l = l.replace(regExp, "");
						return l;
					}).join("\n");
				}else{
					v = v.join("\n");
				}
			}
			this.innerHTML_ = v;
		}

		let theme = this.theme?' theme-'+this.theme:'';
		return html`<pre class="lang-${this.lang}${theme}">${this.innerHTML_}</pre>`
	} 
	htmlEscape(s) {
		return s
			.replace(/&/g, '&amp;')
			.replace(/</g, '&lt;')
			.replace(/>/g, '&gt;');
	}

	updated() {
		this.updateStyle();
	}

	updateStyle() {
		if (!window.PR) {
			if (!this.count)
				this.count = 0;
			this.count++;
			if (this.count > 1000)
				return
			return setTimeout(() => this.updateStyle(), 100);
		}

		let pre = this.renderRoot.querySelector("pre");
		//console.log("window.PR ready", pre)
		//window.PR.prettyPrint(null, this.renderRoot.querySelector("pre"))
		let code = PR.prettyPrintOne(this.htmlEscape(this.innerHTML_))
		if (this.code != code) {
			this.code = code;
			//console.log("updating....")
			//this.update()
			pre.innerHTML = code;
		}

	}
}

FlowCode.define("flow-code");

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_COLOR_SELECTOR_423 : &'static str = r###"




/**
* @class FlowColorSlider
* @extends BaseElement
* @prop {Number} min
* @prop {Number} max
* @prop {Boolean} vertical
* @prop {Object} color
* @prop {String} chanel
* @example
*   <flow-color-slider></flow-color-slider>
*/

export class FlowColorSlider extends FlowCanvasElement {
	static get properties() {
		return {
			min : { type:Number },
			max : { type:Number },
			vertical:{ type:Boolean },
			color:{ type: Object, reflect: true },
			channel : { type:String }
		}
	}

	static get styles(){
		return css`
			:host{
				display:block;
			}
			:host([disabled]){
				opacity:0.5;
				cursor:default;
				pointer-events:none;
			}
			:host(:not([disabled])){
				cursor:pointer;
			}
		`;
	}

	constructor(){
		super();

		this.min = 0;
		this.max = 255;
		this.vertical = false;
		this.color = { };
		this.channel = '?';

		this.mixer = new ColorMixer();
	}

	render() {
		let box = this.getBoundingClientRect();
		return html`
		<canvas id="canvas" style="height:100%;width:100%;" width="${box.width*this.scale}" height="${box.height*this.scale}">Your browser does not support the HTML5 canvas tag</canvas>
		`;
	}

	click(e) {
		console.log('flow-color-slider click:',e);
		this.fire("flow-color-slider-click", {el:this, e})
	}

	firstUpdated(){
		if(window.ResizeObserver){
			this.resizeObserver = new ResizeObserver(e => {
				this.fire('flow-resize', {}, {bubbles:true})
			});
			this.resizeObserver.observe(this);
		}

		['mousedown','mouseup','mousemove','click', 'pointerdown', 'pointerup', 'pointermove','mouseenter','mouseleave'].forEach((event) => {
			this.addEventListener(event, (e) => { this.onMouseEvent(event,e); });
		})

		this.addEventListener('flow-resize', (e)=>{
			this.debounce("flow-resize", this._onResize.bind(this), 100);
		})

		this.canvas = this.renderRoot.getElementById('canvas');
		this.ctx = this.canvas.getContext('2d');
		this.ctx.globalAlpha = 0;
		this.updateCanvas();

		this.color.registerSink(()=>{
			this.redraw();
		})
	}

	_onResize() {
		console.log('_onResize!');
		this.updateCanvas();
		// this.verbose && console.log('resize:', this.getBoundingClientRect());
	}

	registerSink(sink) {
		this.sink = sink;
	}

	onMouseEvent(event,e) {
		//console.log('onMouseEvent',event,e,this);

		let update = false;
		if(event == 'click')
			update = true;
		else
		if(event == 'mousedown') {
			this.drag = true;
			this.setCapture();
		}
		else
		if(event == 'mouseup')
			this.drag = false;
		else
		if(event == 'mousemove' && this.drag)
			update = true;

		if(update) {
			let x = e.offsetX;
			let v = x / this.size.width * (this.max - this.min) + this.min;
			this.color.change(v, this.channel);
			this.color.notify();
		}
	}

	redraw(){
		let parentBox = this.getBoundingClientRect();
		let canvasBox = this.canvas.getBoundingClientRect();
//		this.verbose && console.log('parentBox:',parentBox);
//		this.verbose && console.log('canvasBox:',canvasBox);
		this.canvasBox = canvasBox;
		let { width, height } = canvasBox;
		this.size = { width, height };

		width *= this.scale;
		height *= this.scale;
		let absolute = this.value / this.max;

		const { ctx } = this;
		ctx.clearRect(0, 0, width, height);
		ctx.lineWidth = 1;

		for(let v = 0; v < width; v++) {
			const c = (v * this.max / width);
			this.mixer.assign(this.color);
			this.mixer.change(c, this.channel);

			ctx.strokeStyle = `rgba(${this.mixer.r},${this.mixer.g},${this.mixer.b},1.0)`;
			ctx.beginPath();
			ctx.moveTo(v, 0);
			ctx.lineTo(v, height);
			ctx.stroke();
		}

		let v = this.color[this.channel] / this.max * width;
		ctx.strokeStyle = `rgba(0,0,0,1.0)`;
		ctx.beginPath();
		ctx.moveTo(v, 0);
		ctx.lineTo(v, height);
		ctx.stroke();
	}
}

FlowColorSlider.define('flow-color-slider');


class ColorMixer {
	constructor(v = { r : 0, g : 0, b : 0, h : 0, s : 0, v : 0, a : 1, }) {
		this.sinks = [ ];
		Object.assign(this,v);
	}

	registerSink(sink) {
		this.sinks.push(sink);
	}

	notify() {
		this.sinks.forEach(fn=>fn(this));
		// this.sinks.forEach(fn=>fn(this));
	}

	assign(src) {
		const { r, g, b, h, s, v, a } = src;
		Object.assign(this, { r, g, b, h, s, v, a });
	}

	change(value, channel) {

		if(channel == 'h' || channel == 's' || channel == 'v') {
			this[channel] = value;
			Object.assign(this, HSVtoRGB(this));
		}
		else {
			this[channel] = Math.round(value);
			Object.assign(this, RGBtoHSV(this));
		}
	}
}

/**
* @class FlowColorSolid
* @extends BaseElement
* @prop {Object} color
* @example
*   <flow-color-solid></flow-color-solid>
*/
export class FlowColorSolid extends BaseElement {
	static get properties() {
		return {
			color : { type : Object, reflect : true }
		}
	}

	static get styles() {
		return css`
			:host {
				display : block;
				border: 1px solid #ccc;
			}

			.solid {
				min-width: 32px;
				min-height: 32px;
				width: 100%;
				height: 100%;
			}
		`;
	}

	constructor() {
		super();
	}


	firstUpdated() {
		this.color.registerSink(()=>{
			this.requestUpdate();
		});
	}

	render() {
		let clr = `rgba(${this.color.r},${this.color.g},${this.color.b}, 1.0)`;
		return html`
			<div class='solid' style="background-color: ${clr}">
			</div>			
		`;
	}
}

FlowColorSolid.define('flow-color-solid');

/**
* @class FlowColorSelector
* @extends BaseElement
* @prop {String} caption
* @example
*   <flow-color-selector></flow-color-selector>
*/

export class FlowColorSelector extends BaseElement {
	static get properties() {
		return {
			caption : { type : String },
		}
	}

	static get styles() {
		return css`
			:host {
				display : block;
				border: 1px solid #ccc;
				padding: 6px;
				margin: 6px;
			}
			
			#wrapper {
				display: flex;
				flex-direction: column;
			}

			#caption {
				text-align: left;
			}

			#ctl {
				display: flex;
				flex-direction: row;
			}

			.sliders {
				flex: 1;
				display: flex;
				flex-direction: column;
			}

			flow-color-slider {
				min-height: 24px;
				margin: 4px;
				border: 1px solid #000;
			}

			flow-color-solid {
				width: 96px;
				height: 96px;
			}
		`;
	}

	constructor() {
		super();

		this.color = new ColorMixer();
		this.color.registerSink(()=>{

		})
	}

	firstUpdated() {
		this.addEventListener('flow-color-slider-click', (e)=>{
			console.log("color selector receiving flow-color-slider-click!");
			//this.debounce("flow-resize", this._onResize.bind(this), 100);
		})

	}

	render() {
		return html`
			<div id="wrapper">
				<div id="caption">${this.caption}</div>
				<div id="ctl">
					<div class='sliders'>
						<flow-color-slider .color=${this.color} channel="r"></flow-color-slider>
						<flow-color-slider .color=${this.color} channel="g"></flow-color-slider>
						<flow-color-slider .color=${this.color} channel="b"></flow-color-slider>
						<flow-color-slider .color=${this.color} channel="h" max="1"></flow-color-slider>
						<flow-color-slider .color=${this.color} channel="s" max="1"></flow-color-slider>
						<flow-color-slider .color=${this.color} channel="v" max="1"></flow-color-slider>
					</div>
					<div>
						<flow-color-solid .color=${this.color}></flow-color-solid>
					</div>			
				</div>
			</div>
		`;
	}
}

FlowColorSelector.define('flow-color-selector');


/* accepts parameters
 * h  Object = {h:x, s:y, v:z}
 * OR 
 * h, s, v
*/
function HSVtoRGB(h, s, v) {
    var r, g, b, i, f, p, q, t;
    if (arguments.length === 1) {
        s = h.s, v = h.v, h = h.h;
    }
    i = Math.floor(h * 6);
    f = h * 6 - i;
    p = v * (1 - s);
    q = v * (1 - f * s);
    t = v * (1 - (1 - f) * s);
    switch (i % 6) {
        case 0: r = v, g = t, b = p; break;
        case 1: r = q, g = v, b = p; break;
        case 2: r = p, g = v, b = t; break;
        case 3: r = p, g = q, b = v; break;
        case 4: r = t, g = p, b = v; break;
        case 5: r = v, g = p, b = q; break;
    }
    return {
        r: Math.round(r * 255),
        g: Math.round(g * 255),
        b: Math.round(b * 255)
    };
}

/* accepts parameters
 * r  Object = {r:x, g:y, b:z}
 * OR 
 * r, g, b
*/
function RGBtoHSV(r, g, b) {
    if (arguments.length === 1) {
        g = r.g, b = r.b, r = r.r;
    }
    var max = Math.max(r, g, b), min = Math.min(r, g, b),
        d = max - min,
        h,
        s = (max === 0 ? 0 : d / max),
        v = max / 255;

    switch (max) {
        case min: h = 0; break;
        case r: h = (g - b) + d * (g < b ? 6: 0); h /= 6 * d; break;
        case g: h = (b - r) + d * 2; h /= 6 * d; break;
        case b: h = (r - g) + d * 4; h /= 6 * d; break;
    }

    return {
        h: h,
        s: s,
        v: v
    };
}


"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_REFERENCE_350 : &'static str = r###"



/**
* @class FlowReference
* @extends BaseElement
* @property {String} [for]
* @property {String} [type]
* @example
*   <flow-tooltip>Button 1</flow-tooltip>
*
*
*/

export class FlowReference extends BaseElement {
	static get properties() {
		return {
			for : { type : String },
            type : { type : String },
            icon : {type : String},
            visible : { type : Boolean },
            'right-align-tooltip':{type:Boolean}
		}
	}

	static get styles() {
		return css`
			:host {
		
            }
			.icon-box{
				display:inline-block;
				width:20px;
				max-width:20px;
				text-align:center;
				/*border: 1px solid red;*/
			}
	
			.icon-box svg{
				width:15px;
				height:15px;
				margin-right: 8px;
				margin-bottom: 8px;
				/*margin-left: 8px;*/
				/*fill:var(--flow-primary-color, rgba(0,151,115,1.0));*/
				fill: #666;
			}
			.tooltip-content{display:none}			
		`;
	}
	constructor() {
        super();
    }

	render() {
		let iconSrc = "";
		if(this.icon != "-")
			iconSrc = this.iconPath(this.icon || "fal:info-circle");
		// const iconSrc = this.iconPath(this.icon || "info-circle");

		return html`
			<slot></slot>
			<span class="tooltip" @mouseenter="${this.onTooltipMouseEnter}">
				<div class="icon-box"><svg><use href="${iconSrc}"></use></svg></div>
				<slot class="tooltip-content" name="tooltip"></slot>
			</span>
		`;
	}

	firstUpdated(){
		super.firstUpdated();
		this.tooltipEl = this.renderRoot.querySelector(".tooltip");
		this.tooltipTextEl = document.createElement("div");
		this.tooltipTextEl.classList.add("flow-tooltip-text")
		this.tooltipSlot = this.renderRoot.querySelector(".tooltip-content");
		this.tooltipSlot.addEventListener('slotchange', e=>{
			this.updateTooltipContent();
		});
		document.body.append(this.tooltipTextEl);
		this.updateTooltipContent();

		this.tooltipTextEl.addEventListener("mouseenter", ()=>{
			this.mouseInTooltipContent = true;
		})

		this.tooltipTextEl.addEventListener("mouseleave", ()=>{
			this.mouseInTooltipContent = false;
			this.tooltipTextEl.classList.remove("active")
			if(this.timeoutId){
				clearTimeout(this.timeoutId)
				delete this.timeoutId;
			}
		})
	}

	updateTooltipContent(){
		let nodes = this.tooltipSlot.assignedNodes();
		this.tooltipTextEl.innerHTML = "";
		nodes.forEach(n=>{
			this.tooltipTextEl.append(n.cloneNode(true));
		})
	}

	onTooltipMouseEnter(e){
		let box = this.tooltipEl.getBoundingClientRect();
		//console.log("box", box)
		let cX = box.left + box.width/2;
		let cY = box.top + box.height/2;
		let winWidth = window.innerWidth;
		let winHWidth = winWidth/2;
		let winHeight = window.innerHeight;
		let winHHeight = winHeight/2;

		let style = this.tooltipTextEl.style;
		if(this['top-align-tooltip'] || cY > winHHeight){
			style.top = 'initial';
			style.bottom = (winHeight-box.top)+"px";
		}else{
			style.bottom = 'initial';
			style.top = box.bottom+"px";
		}
		
		if(this['right-align-tooltip'] || cX > winHWidth){
			style.left = 'initial';
			style.right = (winWidth - box.right)+"px";
		}else{
			style.right = 'initial';
			style.left = box.left+"px";
		}

		this.tooltipTextEl.classList.add("active")
		this.checkAndCloseTooltipIfAway();
	}
	checkAndCloseTooltipIfAway(e){
		this.timeoutId = setTimeout(()=>{
			if(this.mouseInTooltipContent)
				return this.checkAndCloseTooltipIfAway();
			this.tooltipTextEl.classList.remove("active")
		}, 1000)
	}
}

FlowReference.define('flow-reference');

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_CANVAS_432 : &'static str = r###"


export class FlowCanvasElement extends BaseElement {
	constructor() {
		super();

		this.canvasScale = 0.75;
	}

	getPixelRatio(){
    	const ctx = this.canvas.getContext("2d"),
        dpr = window.devicePixelRatio || 1,
        bsr = ctx.webkitBackingStorePixelRatio ||
              ctx.mozBackingStorePixelRatio ||
              ctx.msBackingStorePixelRatio ||
              ctx.oBackingStorePixelRatio ||
              ctx.backingStorePixelRatio || 1;

    	return dpr / bsr * 2;
	}

	setHiDPICanvas(w, h, ratio) {
		const can = this.canvas;
		let w_ = w;
		let h_ = h;
		can.width = w_ * ratio;
		can.height = h_ * ratio;
		// can.style.width = w_ + "px";
		// can.style.height = h_ + "px";
		can.getContext("2d").setTransform(ratio, 0, 0, ratio, 0, 0);
	}

	updateCanvas() {
		if(!this.canvas)
			return;

		let parentBox = this.getBoundingClientRect();
		let canvasBox = this.canvas.getBoundingClientRect();
		let { width, height } = canvasBox;
		this.PIXEL_RATIO = this.getPixelRatio();
		this.setHiDPICanvas(width*this.canvasScale,height*this.canvasScale,this.PIXEL_RATIO);
		this.redraw(this.canvasContext2d, canvasBox);
	}

	get htmlCanvasElement() {
		let box = this.getBoundingClientRect();
		return html`<canvas id="canvas" style="height:100%;width:100%;" width="${box.width*this.canvasScale}" height="${box.height*this.canvasScale}">Your browser does not support the HTML5 canvas tag</canvas>`;
	}

	firstUpdated() {
		if(window.ResizeObserver){
			this.resizeObserver = new ResizeObserver(e => {
				this.fire('flow-canvas-resize', {}, {bubbles:true})
			});
			this.resizeObserver.observe(this);
		}

		[
			'mousedown','mouseup','mousemove','click', 'pointerdown',
			'pointerup', 'pointermove','mouseenter','mouseleave'
		].forEach((event) => {
			this.addEventListener(event, (e) => { this.onMouseEvent(event,e); });
		})

		this.addEventListener('flow-canvas-resize', (e)=>{
			this.debounce("flow-canvas-resize", this.handleResize.bind(this), 100);
		})

		this.canvas = this.renderRoot.getElementById('canvas');
		this.canvasContext2d = this.canvas.getContext('2d');
		this.ctx.globalAlpha = 0;
		this.updateCanvas();
	}

	handleResize() {
		this.updateCanvas();
	}

	redraw(ctx, size) {
		throw new Error('BaseCanvasElement::redraw() - missing implementation!');
	}
}



"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_D3_391 : &'static str = r###"


export class Flowd3Element extends BaseElement {
	static get properties() {
		return {
		};
	}

	static get styles(){
		return css `
			
			:host([hidden]){display:none;}
			.d3-holder{
				min-height:100px;
				min-width:100px;
				display:flex;
				flex-direction:column;
				box-sizing:border-box;
				position:relative;
				user-select: none;      
			}
            
            #d3 {flex:1;overflow:hidden}
		`;
	}
	constructor() {
		super();
	}

    render() {
		return html`<div id="d3"></div>`;
	}

    firstUpdated() {
		this.el_d3 = this.renderRoot.getElementById('d3');
		if(this.el_d3){
			this._firstUpdated();
		}else{
			this.debounce("_firstUpdated", ()=>{
				this._firstUpdated()
			}, 500)
		}
	}
	_firstUpdated() {
		this.el_d3 = this.renderRoot.getElementById('d3');
		if(!this.el_d3)
			return
		this.el_d3Rect = this.getBoundingClientRect.call(this.el_d3);
		this.el_d3.getBoundingClientRect = ()=>{
			if(this.el_d3Rect.width==0 && this.el_d3Rect.height==0)
				this.el_d3Rect = this.getBoundingClientRect.call(this.el_d3);
			return this.el_d3Rect;
		}
	
    
        this.init_d3();
    }
   
    
    init_d3() {
		this.svg = d3.select(this.el_d3).append("svg");
		//this.svg.attr("viewBox", this.svgViewBox || [0,0,1,1]);
		this.svg.attr("width", this.svgWidth || '100%');
		this.svg.attr("height", this.svgHeight || '100%');
		this.svg.attr('preserveAspectRatio', this.svgPreserveAspectRatio || 'xMidYMid meet');
    	this.el = this.svg.append("g")
    	this.el.transform = d3.zoomIdentity.translate(0, 0).scale(1);
		this.updateSVGSize();
		//setTimeout(()=>{
		//	this.updateSVGSize();
		//}, 100)

		this.fire("ready", {})
    }

    onElementResize(){
    	if(this.el_d3)
    		this.el_d3Rect = this.getBoundingClientRect.call(this.el_d3);
		this.updateSVGSize();
    }

    connectedCallback(){
    	super.connectedCallback();
    	//this._onWindowResize = this._onWindowResize || this.onWindowResize.bind(this);
		//window.addEventListener("resize", this._onWindowResize)
		if(!this.__resizeObserver){
    		this.__resizeObserver = new ResizeObserver(()=>{
	    		this.onElementResize();
			});
			this.__resizeObserver.observe(this);
	    }
    }

    disconnectedCallback() {
		super.disconnectedCallback();
		//if(this._onWindowResize)
		//	window.removeEventListener("resize", this._onWindowResize)
		if(this.__resizeObserver){
			this.__resizeObserver.unobserve(this);
			this.__resizeObserver.disconnect();
			delete this.__resizeObserver;
		}
	}

    updateSVGSize(){
    	if(!this.el_d3)
    		return
    	let {width, height} = this.el_d3.getBoundingClientRect();
    	this.svg.attr("viewBox", this.svgViewBox || [0,0, width, height]);
    	this.draw();
    }

    draw(){
    	//
    }
}
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_GRAPH_351 : &'static str = r###"





/**
* @class FlowGraph
* @extends Flowd3Element
* @prop {Boolean} disabled
* @prop {String} title 
* @prop {String} prefix
* @prop {String} suffix
* @prop {String} align
* @prop {Number} value
* @prop {String} data
* @prop {String} sampler
* @prop {Number} range
* @prop {Boolean} overlay
* @prop {String} format
* @prop {Number} precision 
* @prop {Boolean} axes
* @prop {Boolean} info
* @cssvar {font-family} [--flow-data-field-font-family="Julius Sans One"]
* @cssvar {font-weight} [--flow-data-field-font-weight=bold]
* @cssvar {background-color} [--flow-background-color]
* @cssvar {background} [--flow-graph-info-bg=#FFF);
* @cssvar {border} [--flow-graph-info-border=1px solid #DDD]
* @example
*   <flow-graph>overlay content</flow-graph>
*
*/
export class FlowGraph extends Flowd3Element {
	static get properties() {
		return {
			disabled:{type:Boolean, reflect:true},
			title:{type:String},
			prefix:{type:String },
			suffix:{type:String},
			align:{type:String},
			value:{type:Number},
			data:{type:String},  // sampler: 'kaspad.kd0.info.
			sampler:{type:String},
			range:{type:Number},
			overlay:{type:Boolean},
			format:{type:String},
			precision:{type:Number},
			axes:{type:Boolean},
			info:{type:Boolean},
		}
	}

	static get styles(){
		return [Flowd3Element.styles, css`
			:host{
				display:inline-flex;
				font-weight:bold;
				font-size:13px;
				text-transform:uppercase;
				cursor:pointer;
				font-family:var(--flow-data-field-font-family, "Julius Sans One");
				font-weight:var(--flow-data-field-font-weight, bold);
				border-radius: 10px;
				overflow: hidden;
				position:relative;
			}
			:host([disabled]){opacity:0.5;cursor:default;pointer-events:none;}
			.colon{display:none}
			:host(.has-colon) .colon{display:inline;}
			.container{
				white-space: nowrap;
				padding:2px 6px 6px 6px;
				height: 100%;
			}
			
			.container>div{padding:2px;}
			.title{flex:1; text-align:left; opacity:1;xmargin-top:7px; font-size: 10px; color: var(--flow-data-badge-caption); xtext-shadow: 0px 0px 0px var(--flow-data-badge-caption-shadow, #fff); }
			.value{text-align:right; opacity:1;font-size:14px;font-family:"Exo 2";font-weight:normal;background-color: var(--flow-background-color:);}
			.prefix{opacity:0.9;margin-right:3px;margin-top:3px; font-size: 10px;}
			.suffix{opacity:0.9;margin-left:3px;margin-top:3px; font-size: 10px;}
			.col{display: flex; flex-direction: column; align-items: left;}
			.row{display: flex; flex-direction: row; flex:0;}

			.wrapper {
				/*width:100%;height:100%;*/
				position:relative;
				flex:1;
				margin:6px;overflow:hidden;
				/*
				min-width: var(--flow-data-badge-graph-with,240px);
				min-height: var(--flow-data-badge-graph-height,80px);				
				*/				
			}
			
			:host([border]) .wrapper {
				border: 2px solid var(--flow-primary-color,#333);
				box-shadow: 2px 2px 1px rgba(1, 123, 104, 0.1);
				border-radius: 10px;

			}

			.wrapper > div {
				width:100%;height:100%;
				position:relative;left:0px;top:0px;bottom:0px;right:0px;
			}

			.d3-holder{
				min-height:10px;
				min-width:10px;
				opacity:1;
				border-radius:10px;
			}
			.wrapper>div.d3-holder{position:absolute;}
			.overlay{pointer-events:none}
			.info{
				position:absolute;pointer-events:none;
				background:var(--flow-graph-info-bg, #FFF);
				border:var(--flow-graph-info-border, 1px solid #DDD);
				padding:3px;font-size:0.7rem;left:10px;top:10px;
				opacity:0;max-width:48%;
			}
			.info-dot{opacity:0}
			[flex] {
				flex: 1;
			}


			.axis {
				font-size:12px;
				font-family: "Consolas", "Source Sans Pro";
				font-weight: 300;
				strokeColor: #333;
			}

			.axis text {
				fill:var(--flow-background-inverse-soft, #aaa);
			}
			.axis path {
				stroke:var(--flow-background-inverse-soft, #aaa);
			}
			.axis line {
				stroke:var(--flow-background-inverse-soft, #aaa);
			}

			.value-container {
				/*background-color: var(--flow-background-color, rgba(0,0,0,0));*/
				display:flex;
				flex-direction:row;
			}

			.title-bottom { display: none; }
			.host([bottom])	.title-bottom { display: block; }
			.host([bottom])	.title-top { display: none; }
			.host([top])	.title-bottom { display: none; }
			.host([top])	.title-top { display: block; }
		`];
	}

	constructor() {
		super();
		this.sampler = '';
		this.range = 60 * 5;
		this.refresh = 1e3;
		this.precision = 0;
		this.axes = false;
		this.info = false;

		this.svgPreserveAspectRatio = 'xMaxYMax meet';
	}

	onElementResize(){
		super.onElementResize();
		dpc(()=>{
			super.onElementResize();
			this.requestUpdate("element-resize", null);
		})
		dpc(1000, ()=>{
			super.onElementResize();
			this.requestUpdate("element-resize", null);
		})
	}

	connectedCallback() {
		super.connectedCallback();
		if(this.sampler)
			this.interval = setInterval(this.requestUpdate.bind(this), this.refresh);
	}

	disconnectedCallback() {
		super.disconnectedCallback();

		if(this.interval)
			clearInterval(this.interval);
	}

	render() {

		dpc(()=>{
			this.draw();
		})

		let value = '';
		//this.log("render flow-graph");
		if(this.sampler) {
			let idents = this.sampler.split(':');
			let ident = idents.shift(); 
			let sampler =  FlowSampler.get(ident);
			value = sampler.last() || '';
			if(value !== undefined) { 
				value = FlowFormat[this.format || 'default'](value || 0, this);
			}
		}
		else {
			console.log("no sampler", this);
		}


		if(this.overlay) {
			return html`
			<div class='wrapper'>
				<div class="d3-holder">${super.render()}</div>
				<div class="overlay">
					<div class="container col">
						<!-- div class="title title-top">${this.title}<span class="colon">:</span></div -->
						<div class="row">
							<div class="title title-top">${this.title}<span class="colon">:</span></div>
							${ (!this.align || this.align == 'right') ? html`<div style="flex:1;"></div>` : '' }
							<div class="value-container">
								<div class="prefix">${this.prefix}</div>
								<div class="value">${value}</div>
								<div class="suffix">${this.suffix}</div>
							</div>
							${ (this.align == 'left') ? html`<div style="flex:1;"></div>` : '' }
						</div>
						<div flex></div>
						<!-- div class="row">
							<div class="title title-bottom">${this.title}<span class="colon">:</span></div>
							${ (!this.align || this.align == 'right') ? html`<div style="flex:1;"></div>` : '' }
							<div class="value-container">
								<div class="prefix">${this.prefix}</div>
								<div class="value">${value}</div>
								<div class="suffix">${this.suffix}</div>
							</div>
							${ (this.align == 'left') ? html`<div style="flex:1;"></div>` : '' }
						</div -->
					</div>
				</div>
			</div>
			<div class="info"></div>
			`;	
		} else {

			return html`
			<div class='wrapper'>
				<div class="d3-holder">${super.render()}</div>
				<div>
					<div class="container col">
						<slot></slot>
					</div>
				</div>
			</div>
			<div class="info"></div>
			`;	
		}
	}

	getMargin(){
		if(this.axes){
			return {
				bottom:40,
				top:30,
				left:20,
				right:20
			}
		}
		return {
			bottom:0,
			top:10,
			left:0,
			right:0
		}
	}
	draw(){
		let margin = this.getMargin();
		let {height:fullHeight, width:fullWidth} = this.el_d3.getBoundingClientRect();
		let width = fullWidth - margin.left - margin.right;
    	let height = fullHeight - margin.top - margin.bottom;

		if(!this.sampler)
			return;
		let samplerIdents = this.sampler.split(':');
		
		let samplers = samplerIdents.map((ident) => {
			let sampler =  FlowSampler.get(ident);
			if(!this._draw){
				this._draw = this.draw.bind(this);
				sampler.on('data', this._draw);
			}
			return sampler;
		})
		
		let data = samplers[0].data;

		//console.log(JSON.stringify(data, null))
		let [min,max] = d3.extent(data, d => d.date);
		//console.log("processing min-max[1]",min,max);
		if(!this.axes)
			min = max - 1000*this.range;
		let maxTextLength = 0;
		data.forEach(d=>{
			if(d.value.toFixed(this.precision).length>maxTextLength)
				maxTextLength = d.value.toFixed(this.precision).length;
		})

		if(this.axes && margin.left < maxTextLength * 10){
			let oldLeft = margin.left
			margin.left = maxTextLength * 10;
			width += oldLeft - margin.left;
		}


		const x = d3.scaleUtc()
		.domain([min, max])
		.range([0, width])

		const y = d3.scaleLinear()
		.domain(d3.extent(data, d => d.value)).nice()
		.range([height, 0]);

		let xAxis, yAxis;
		if(this.axes){
			xAxis = g => g
			.attr("transform", `translate(0,${height})`)
			.call(d3.axisBottom(x).ticks(width / 80).tickSizeOuter(0));

			yAxis = g => g
			//.attr("transform", `translate(${margin.left},0)`)
			.call(d3.axisLeft(y).ticks(height / 20).tickSizeOuter(0))
			//.call(g => g.select(".domain").remove())
		}
		
		// .call(g => g.select(".tick:last-of-type text").clone()
		// 	.attr("x", 3)
		// 	.attr("text-anchor", "start")
		// 	.attr("font-weight", "bold")
		// 	.text(this.title));

		const area = d3.area()
			.curve(d3.curveLinear)
			.x(d => x(d.date))
			.y0(height)
			.y1(d => y(d.value));

		const { el } = this;
		// el.append('path')
		// 	.datum(data)
		// 	.attr('fill','var(--flow-graph-fill, steelblue)')
		// 	.attr('stroke','var(--flow-graph-stroke, steelblue)')
		// 	.attr('d',area);
		let t = `translate(${margin.left},${margin.top})`;
		if(el.__t != t){
			el.__t = t
			el.attr("transform", t)
		}

		if(this.svg.__w != fullWidth){
			this.svg.__w = fullWidth;
			this.svg
				.attr("width", fullWidth)
		}
		if(this.svg.__h != fullHeight){
			this.svg.__h = fullHeight;
			this.svg
				.attr("height", fullHeight)
		}
			

		if(!this.path)
			this.path = el.append('path')
				.attr('stroke-opacity', 'var(--flow-graph-stroke-opacity, 1.0)')
				.attr("stroke-linejoin", "round")
				.attr("stroke-linecap", "round")
				.attr("stroke-width", 'var(--flow-graph-stroke-width, 0)')
				.attr('fill','var(--flow-graph-fill, steelblue)')
				.attr('stroke','var(--flow-graph-stroke, "#000)')

		try {				
			this.path.datum(data)
				.attr('d', area);
		} catch(ex) {
			if(this.sampler)
				console.log('error while processing sampler:',this.sampler);
			console.log(ex);

		}

		if(this.info){
			let me = this;
			let bisect = d3.bisector(d=>d.date).left;
			//let _data = data.map(d=>d.date.getTime());
			let timeFormat = d3.timeFormat("%x %X");
			this.getDataByPoint = (p)=>{
				let x0 = x.invert(p-margin.left);
				/*let t = x0.getTime();
				let dif = -1, _dif, index=-1;
				_data.forEach((ts, i)=>{
					_dif = Math.abs(ts-t)
					if(dif<0 || dif>_dif){
						index = i;
						dif = _dif
					}
				})
				//console.log("index", index, x0, p)*/
				let index = bisect(data, x0);
				let d = data[index];
				if(!d)
					return
				let cx = x(d.date);
				let cy = y(d.value);
				let l = null;
				let r = null;
				if(cx>width*0.5){
					r = width-cx+margin.right+12;
				}else{
					l = cx+12+margin.left
				}
				return {cx, cy, d, l, r, t:cy+12+margin.top};
			}
			if(!this.infoEl){
				this._infoEl = this._infoEl||this.renderRoot.querySelector('.info')
				this.infoEl = d3.select(this._infoEl);
				this.infoDot = el.append("circle")
					.attr("class", "info-dot")
			        .attr("fill", "var(--flow-graph-info-dot-fill, red)")
			        .attr("stroke", "var(--flow-graph-info-dot-stroke, none)")
			        .attr("r", 3)
				this.svg 
					.on('mousemove', function(_d){
						let p = d3.mouse(this)[0];
					    let data = me.getDataByPoint(p);
						//console.log("ddd", d.value, x0)
						if(!data)
							return
						let {cx, cy, l, r, t, d} = data;
						let infoEl = me.infoEl;
						infoEl
							.html(FlowFormat[me.format||'default'](d.value, me)+", "+timeFormat(d.date))
		     				.style("top", t+"px")
		     			if(l){
		     				infoEl.style("right", 'initial')
		     				infoEl.style("left", l+"px")
		     			}else{
		     				infoEl.style("right", r+"px")
		     				infoEl.style("left", 'initial')
		     			}
						infoEl.transition()
							.duration(50)
							.style("opacity", 1)
						me.infoDot
							.style("opacity", 1)
							.attr("cx", cx)
			        		.attr("cy", cy)
					})
					.on('mouseout', ()=>{
						this.infoDot.style("opacity", 0);
						this.infoEl.transition()
							.duration(50)
							.style("opacity", 0);
					});
			}
		}


		if(this.axes){
			this.xAxis = this.xAxis || el.append("g")
			this.xAxis.call(xAxis);
			this.yAxis = this.yAxis || el.append("g")
			this.yAxis.call(yAxis);

			this.xAxis.classed('axis', true);
			this.yAxis.classed('axis', true);
		}

	}
}

FlowGraph.define('flow-graph');

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_SAMPLER_401 : &'static str = r###"


if(!flow.samplers) {
	flow.samplers = {
        inst : { },
        sinks : [ ],
		get : (ident, options) => {
			let sampler = flow.samplers.inst[ident];
			if(!sampler)
				sampler = flow.samplers.inst[ident] = new FlowSampler(ident, options);
			return sampler;
        },
        registerSink : (fn) => {
            flow.samplers.sinks.push(fn);
        },
        unregisterSink : (fn) => {
            let idx = flow.samplers.sinks.indexOf(fn);
            if(idx >= 0)
                flow.samplers.sinks.splice(idx, 1);
        }        
	}
}

export class FlowSampler {

	static get(...args) {
		return flow.samplers.get(...args);
	}

	constructor(ident, options = { }) {
		this.ident = ident;
		if(!this.ident)
			throw new Error('fatal: FlowSampler::constructor() missing options.ident');
		
		this.options = options;
		this.generator = this.options.generator;
        this.data = [ ];
		this.eventHandlers = new Map();
		this.eventHandlers.set("data", new Map());

		if(this.options.interval)
			this.start();
	}

	async start() {
		if(this.running)
			return Promise.reject('already running');

		const { interval } = options;
		this.interval = setInterval(this.poller.bind(this), interval);

		this.running = true;
	}

	stop() {
		if(this.interval) {
			clearInterval(this.interval);
			delete this.interval;
		}

		this.running = false;
	}

	poll(poller) {
		this.generator = poller;
	}

	poller() {
		if(!this.generator) {
			console.error('FlowSampler::poller() missing generator');
			return;
		}

		if(typeof this.generator != 'function') {
			console.error('FlowSampler::poller() generator must be a function');
			return;
		}

        
		this.generator(ts, lastTS);
		
	}

	put(value) {
        const { ident, data, options, sinks } = this;
		const date = new Date();
		data.push({date,value});
		let max = options.maxSamples || (60*5);
		while(data.length > max)
			data.shift();
        this.fire('data', {ident,data});
        flow.samplers.sinks.forEach((sink) => {
            sink(ident, value, date);
        })
    }

    lastEntry() {
        if(!this.data.length)
            return undefined;
        return this.data[this.data.length-1];
    }


    last(_default) {
        if(!this.data.length)
            return _default;
        return this.data[this.data.length-1].value;
    }

    first(_default = undefined) {
        if(!this.data.length)
            return _default;
        return this.data[0].value;
    }

	fire(name, data={}){
		let ce = new CustomEvent(`flow-sampler-${name}-${this.ident}`, {detail:data})
		document.body.dispatchEvent(ce);
		let handlers = this.eventHandlers.get(name);
		if(handlers){
			handlers.forEach((v, fn)=>{
				fn({data});
			})
		}
	}
	on(name, fn){
		let handlers = this.eventHandlers.get(name);
		if(!handlers)
			return
		handlers.set(fn, 1);
		//document.body.addEventListener(`flow-sampler-${name}-${this.ident}`, fn);
	}
	off(name, fn){
		let handlers = this.eventHandlers.get(name);
		if(!handlers)
			return
		handlers.delete(fn);
		//document.body.removeEventListener(`flow-sampler-${name}-${this.ident}`, fn);
    }
    

}
"###;

const ASPECTRON_FLOW_UX_SRC_BASE_ELEMENT_410 : &'static str = r###"
/*
* Flow-UX: src/base-element.js
* version: 1.0.0
*/















/**
* @class BaseElement
* @extends LitElement
*/
export class BaseElement extends LitElement{

	static get baseUrl(){
		return baseUrl;
	}

	static hashCode(str){//java String#hashCode
	    var hash = 0;
	    for (var i = 0; i < str.length; i++) {
	       hash = str.charCodeAt(i) + ((hash << 5) - hash);
	    }
	    return hash;
	} 

	static intToRGB(i){
	    var c = (i & 0x00FFFFFF)
	        .toString(16)
	        .toUpperCase();

	    return "00000".substring(0, 6 - c.length) + c;
	}

	/**
	* convert any string to color hex code
	* @param {String} str any string i.e 'hello'
	* @return {String} color hex code i.e `#DDFFAA`
	*/
	static strToColor(str){
		return '#'+this.intToRGB(this.hashCode(str));
	}

	/**
	* @desc define customElements. you can call on drived/child class as <code class="prettyprint js">CoolElement.define('my-cool-element')</code>
	* @param {String} name name of tag i.e. 'my-cool-element'
	* @since 0.0.1
	*/
	static define(name, deps){
		if(deps) {
			DeferComponent(this,name,deps);
		}
		else
			this.defineElement(name);
	}

	static defineElement(name){
		customElements.define(name, this);
	}

	static get svgStyle(){
		return css`
			svg.icon{
				width:28px;
				height:28px;
				margin:0px 5px;
				fill:var(--flow-primary-color);
			}
		`
	}

	static createElement(name, attr={}, props={}){
		let el = document.createElement(name);
		Object.keys(attr).forEach(k=>{
			el.setAttribute(k, attr[k])
		})
		Object.keys(props).forEach(k=>{
			el[k] = props[k];
		})

		return el;
	}

	static setLocalSetting(name, value, prefix='flow-'){
		if(!window.localStorage)
			return

		window.localStorage[prefix+name] = value;
	}

	static getLocalSetting(name, defaults, prefix='flow-'){
		if(!window.localStorage)
			return defaults;

		let value = window.localStorage[prefix+name];
		if(typeof(value) == 'undefined')
			return defaults

		return value;
	}

	/**
	* fire CustomEvent
	* @param {String} eventName name of event
	* @param {Object=} detail event's [detail]{@link https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/detail} property
	* @param {Object=} options [CustomEventInit dictionary]{@link https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/CustomEvent}
	* @param {HTMLElement=} event target (default: window)
	* @return {boolean} The return value is false if event is cancelable and at least one of the event handlers which handled this event called Event.preventDefault(). Otherwise it returns true.
	* @since 0.0.1
	*/
	static fire(eventName, detail={}, options={}, el=null, returnEvent=false){
		let ev = new CustomEvent(eventName, Object.assign({}, options, {detail}));
		let result = (el || window).dispatchEvent(ev);
		return returnEvent?ev:result
	}

	static get sizeClsMap(){
		return sizeClsMap
	}
	static setElementSizeClass(cmp, width){

		width = width || cmp.getBoundingClientRect().width;
		let found = [...this.sizeClsMap.entries()].find(([key, size])=>{
			//console.log("foundfoundfound:key,size", key, size)
			return width<=size
		}) || ["LG"];

		//console.log("foundfoundfound", width, found)
		let cls = [...this.sizeClsMap.keys(), "LG"];
		let [clsToAdd] = found;
		cls.splice(cls.indexOf(clsToAdd), 1);
		cmp.classList.remove(...cls);
		
		//temporarily remove XSS size for accounts panel
		// if(clsToAdd == "XXS")
		// 	clsToAdd = "TINY";
		//console.log("foundfoundfound:adding cls", cls, clsToAdd, cmp)
		cmp.classList.add(clsToAdd);
		cmp.sizeCls = clsToAdd;

	}

	constructor(){
		super();
		const name = this.constructor.name;
		this.__cname = name.toLowerCase().replace("flow", "");
		this._initLog();
	}

	initPropertiesDefaultValues(props=null){
		this.constructor.elementProperties.forEach((v, key)=>{
			//console.log("key, v", key, v)
			let type = typeof v.value;
			if(!['undefined', 'function'].includes(type))
				this[key] = v.value;
		})
		if(props){
			Object.keys(props).forEach(name=>{
				if(typeof props[name].value != 'undefined')
					this[name] = props[name].value
			})
		}
	}

	_initLog(forceLog = false, name){
		let {localStorage:lS} = window;
		let {debug} = lS||{};
		name = name || this.constructor.name;
		if(forceLog||debug=="all"||debug=="*"||
			(debug+"").indexOf(this.__cname)>-1 ||
			(debug+"").indexOf(name) >-1){
			this.log = Function.prototype.bind.call(
				console.log,
				console,
				`%c[${name}]`,
				`font-weight:bold;color:${this.constructor.strToColor(name)}`
			);
		}else{
			this.log = ()=>{
				//
			}
		}
	}

	cloneValue(value){
		if( value instanceof Array )
			return value.map(v=>this.cloneValue(v))
		else if( value instanceof Object ){
			let r = {}
			Object.entries(value).forEach( ([k,v])=>{
				r[k] = this.cloneValue(v);
			})
			return r;
		}

		return value;	
	}

	/**
	* update the property by its path
	* @param {String} path propery path i.e `tabs.2.disable`
	* @param {*} value new value
	*
	*/
	set(path, value){
		const parts = path.split(".");
		let v = this;
		let last = parts.length-1;
		let updated = false;
		let lastValue = this.cloneValue(v[parts[0]]);
		
		parts.find((p, i)=>{
			if( !(v instanceof Object) )
				return
			if(i==last){
				v[p] = value;
				//this.log("v, p, i, v", {v, p, i, value})
				updated = true;
				return true;
			}
			v = v[p];
		})
		if(updated){
			this.requestUpdate(parts[0], lastValue)
			//this.log("requestUpdate, prop, lastValue", parts[0], lastValue)
		}
		return updated;
	}
 
	/**
	* fire CustomEvent
	* @param {String} eventName name of event
	* @param {Object=} detail event's [detail]{@link https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/detail} property
	* @param {Object=} options [CustomEventInit dictionary]{@link https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/CustomEvent}
	* @param {HTMLElement=} event target (default: this element)
	* @return {boolean} The return value is false if event is cancelable and at least one of the event handlers which handled this event called Event.preventDefault(). Otherwise it returns true.
	* @since 0.0.1
	*/
	fire(eventName, detail={}, options={}, el=null, returnEvent=false){
		let ev = new CustomEvent(eventName, Object.assign({}, options, {detail}));
		let result = (el || this).dispatchEvent(ev);
		return returnEvent?ev:result

	}

	/**
	* debounce provided function for given time
	* @param {String} name key name to debounce function
	* @param {Function} fn a function to debounce
	* @param {Number} time time in milliseconds for delay
	* @return {Object} a reference to debounced function
	* @since 0.0.1
	*/
	debounce(name, fn, time){
		this._debounce = this._debounce || new Map();
		if(this._debounce.has(name))
			this._debounce.get(name).cancel();

		this._debounce.set(name, {
			id:setTimeout(fn, time),
			cancel(){
				if(!this.id){
					clearTimeout(this.id)
					this.id = null;
				}

			}
		})

		return this._debounce.get(name);
	}

	buildUrl(url){
		return this.constructor.baseUrl + url;
	}
	iconPath(name, i){
		return resolveIcon(this.__cname, name, i);
	}

	/**
	* Logs given values
	* @param {...*} args
	*/
	log(...args){
	}


	renderOnStateChange(state, ON=true){
		if(!this._renderOnStateChange)
			this._renderOnStateChange = {};
		this._renderOnStateChange[state] = ON;
	}

	connectedCallback() {
		super.connectedCallback();

		let stateChange = this._renderOnStateChange || {};
		let ONLINE = stateChange[FlowStates.ONLINE];
		let AUTH = stateChange[FlowStates.AUTH];
		let GET_NETWORK_AND_USER_STATE = false;
		if(this.onlineCallback || ONLINE) {
			this.onlineCallback_ = (...args)=>{
				this.__online = true;
				this.onlineCallback?.(...args);
				if(ONLINE)
					this.requestUpdate("FLOW-NETWORK-ONLINE", false)
			}
			window.addEventListener('network-iface-online', this.onlineCallback_);
			GET_NETWORK_AND_USER_STATE = true;
		}

		if(this.offlineCallback || ONLINE) {
			this.offlineCallback_ = (...args)=>{
				this.__online = false;
				this.offlineCallback?.(...args);
				if(ONLINE)
					this.requestUpdate("FLOW-NETWORK-ONLINE", true)
			}
			window.addEventListener('network-iface-offline', this.offlineCallback_);
			GET_NETWORK_AND_USER_STATE = true;
		}

		if(this.signinCallback || AUTH) {
			this.signinCallback_ = (...args)=>{
				this.__signedin = true;
				this.signinCallback?.(...args);
				if(AUTH)
					this.requestUpdate("FLOW-USER-AUTH", false)
			}
			window.addEventListener('flow-user-signin', this.signinCallback_);
			GET_NETWORK_AND_USER_STATE = true;
		}

		if(this.signoutCallback || AUTH) {
			this.signoutCallback_ = (...args)=>{
				this.__signedin = false;
				this.signoutCallback?.(...args);
				if(AUTH)
					this.requestUpdate("FLOW-USER-AUTH", true)
			}
			window.addEventListener('flow-user-signout', this.signoutCallback_);
			GET_NETWORK_AND_USER_STATE = true;
		}

		if(GET_NETWORK_AND_USER_STATE){
			let ce = this.fire("flow-network-and-user-state-get", {}, {}, window, true);
			//console.log("signedin, online", ce.detail, this)
			let {signedin, online} = ce.detail || {};

			if(typeof online != 'undefined'){
				if(online){
					this.onlineCallback_?.()
				}else{
					this.offlineCallback_?.()
				}
			}
			if(typeof signedin != 'undefined'){
				if(signedin){
					this.signinCallback_?.()
				}else{
					this.signoutCallback_?.()
				}
			}
		}

		if(this.onReCaptchaReady){
			this._onReCaptchaReady = this.onReCaptchaReady.bind(this);
			window.addEventListener("g-recaptcha-ready", this._onReCaptchaReady)
		}

		if(this.registeredListeners) {
			this.registeredListeners.forEach(({ name, handler }) => {
				window.addEventListener(name, handler);
			})
		}

		if(this.socketSubscriptions) {
			this.socketSubscriptions.forEach((subscription) => {
				subscription.resubscribe();
			})
		}

	}

	onlineCallback() {
		super.onlineCallback?.();
		if(this.pendingSocketSubscriptions) {
			this.pendingSocketSubscriptions.forEach((subscription) => {
				const { subject } = subscription;
				this.subscribe(subject, subscription);
				//subscription.resubscribe();
			})
		}

	}

	disconnectedCallback() {
		super.disconnectedCallback();
		
		if(this.onlineCallback_) {
			window.removeEventListener('network-iface-online', this.onlineCallback_);
			delete this.onlineCallback_;
		}
		if(this.offlineCallback_) {
			window.removeEventListener('network-iface-offline', this.offlineCallback_);
			delete this.offlineCallback_;
		}
		if(this.signinCallback_) {
			window.removeEventListener('flow-user-signin', this.signinCallback_);
			delete this.signinCallback_;
		}
		if(this.signoutCallback) {
			window.removeEventListener('flow-user-signout', this.signoutCallback);
			delete this.signoutCallback;
		}
		if(this._onReCaptchaReady){
			window.removeEventListener("g-recaptcha-ready", this._onReCaptchaReady)
		}

		if(this.registeredListeners) {
			this.registeredListeners.forEach(({ name, handler }) => {
				window.removeEventListener(name, handler);
			})
		}

		if(this.socketSubscriptions) {
			this.socketSubscriptions.forEach((subscription)=>{
				subscription.unsubscribe();
			})
		}
	}

	registerListener(name_, handler_) {
		const {name, handler} = this.addToListenersStack(name_, handler_);
		window.addEventListener(name, handler);
		//console.log("window.addEventListener",name,handler);
	}
	addToListenersStack(name, handler_, stack){
		if(!this.registeredListeners)
			this.registeredListeners = [];
		const handler = handler_ || (() => { this.requestUpdate(); });//.bind(this);
		(stack||this.registeredListeners).push({name,handler});
		return {name,handler};
		
	}

	removeListeners() {
		if(this.registeredListeners) {
			this.registeredListeners.forEach(({ name, handler }) => {
				window.removeEventListener(name, handler);
			})
		}
		this.registeredListeners = [];
	}

	bindDDTriggers(skipEventBind=false){
		
		let triggers = this.renderRoot.querySelectorAll("[data-trigger-for]");
		//console.log("bindDDTriggers", this, triggers)
		triggers.forEach(node=>{
			let id = node.getAttribute("data-trigger-for");
			//console.log("idididid", id)
			if(!id)
				return
			let selector = id;
			if(id[0]!="#")
				selector = "#"+selector;
			let key = node.dataset.ddKey
			if(!key){
				key = id.replace("#", "");
				if(!/DD$/.test(key))
					key = key+"DD";
			}
			let dd = this[key]||this.renderRoot.querySelector(selector);
			//console.log("idididid", {id, key, selector, dd})
			if(!dd)
				node.flowDropdown = null;
			this[key] = dd;
			node.flowDropdown = dd;
			if(skipEventBind)
				return
			if(!node["event-bind-"+id]){
				node["event-bind-"+id] = true;
				node.addEventListener("click", ()=>{
					if(!this[key])
						return
					this[key].toggle();
				})
			}
		})
	}

	isOnline(){
		return this.__online;
	}
	isSignedin(){
		return this.__signedin;
	}
	serialize(){
		return {
			nodeName: this.nodeName
		}
	}
	deserialize(){
		//
	}
	subscribe(subject, subscriber){
		if(!flow.app.defaultRPC) {
			subscriber = new AsyncQueueSubscriber(null,subject);
			if(!this.pendingSocketSubscriptions)
				this.pendingSocketSubscriptions = [];
			this.pendingSocketSubscriptions.push(subscriber);
			return subscriber;
		} else {
			subscriber = flow.app.defaultRPC.subscribe(subject, subscriber);
			if(!this.socketSubscriptions)
				this.socketSubscriptions = new Map();
			this.socketSubscriptions.set(subscriber.uid, subscriber);
			return subscriber;
		}
	}
	request(subject, data, callback){
		return flow.app.defaultRPC.request(subject, data, callback);
	}
}

export const ScrollbarStyle = css`
*::-webkit-scrollbar-track,
:host::-webkit-scrollbar-track{
    box-shadow:var(--flow-scrollbar-track-box-shadow, initial);
    background:var(--flow-scrollbar-track-bg, initial);
}

*::-webkit-scrollbar,
:host::-webkit-scrollbar{
	width:var(--flow-scrollbar-width, initial);
	height:var(--flow-scrollbar-width, initial);
	background:var(--flow-scrollbar-bg, initial);
}
*::-webkit-scrollbar-thumb,
:host::-webkit-scrollbar-thumb{
    box-shadow:var(--flow-scrollbar-thumb-box-shadow, initial);
    background:var(--flow-scrollbar-thumb-bg, initial);
}
`

ScrollbarStyle.appendTo = styleAppendTo(ScrollbarStyle);

let getLocalSetting = BaseElement.getLocalSetting;
let setLocalSetting = BaseElement.setLocalSetting;
export {getLocalSetting, setLocalSetting}

export const SpinnerStyle = css`
	@keyframes spinner-animation{0%{transform:rotate(0deg)}100%{transform:rotate(359deg)}}
	.spinner{
		webkit-animation: spinner-animation 2s linear infinite;
	    animation: spinner-animation 2s linear infinite;
	    transform-origin:center;
	}
	fa-icon.spinner:not([hidden]){display:inline-block;position:relative}
`

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_SOCKET_417 : &'static str = r###"




export class FlowSocket {
	constructor(options){
		this.online = false;
		

		this.options = Object.assign({
			path:'/rpc',
			id:UID(),
			timeout:30,
			origin:window.location.origin
		}, options || {});

		this.timeout  = this.options.timeout;
		this.id = this.options.id;
		this.transport = this.options.transport || 'ws';

		this.init();
	}

	init() {
		this.initEvent();
		this.connected = false;
		if (this.options.path)
			this.connect();
	}

	initEvent() {
		this.pending = new Map();
		this.events = new FlowEvents();
	}

	connect_impl() {

		switch(this.transport) {
			case 'ws': {
				const url = this.options.origin.replace(/^http/,'ws')+this.options.path;
				// console.log('WS connecting to',url);
				this.socket_impl = new WebSocket(this.options.origin.replace(/^http/,'ws')+this.options.path);
			} break;

			case 'sockjs': {
				this.socket_impl = SockJS(this.options.origin+this.options.path, this.options.args || {});
			} break;
		}

		this.socket_impl.onopen=(event)=>{
			this.online = true;
			console.log("RPC connected");
			this.events.emit('open');
		}
		this.socket_impl.onerror=(err)=>{
			console.log("RPC connect_error", err);
			this.events.emit('connect.error', err);
			this.socket_impl.close();
//			this.reconnect_impl();
		}
		this.socket_impl.onmessage=(msg)=>{
			let [ event, data ] = JSON.parse(msg.data);
			this.intake.emit(event, data);
		}
		this.socket_impl.onclose=(event)=>{
			this.online = false;
			console.log("RPC disconnected");
			this.events.emit('disconnect');

			this.pending.forEach((info, id)=>{
				info.callback({ error : "Connection Closed"});
			});

			this.pending.clear();

			this.reconnect_impl();
		};
	}

	reconnect_impl() {
		dpc(1000, ()=>{
			this.connect_impl();
		})
	}

	async connect(){
		if (this._connected || !this.options.path)
			return;
		this._connected = true;
		this.events.emitAsync('rpc-connecting');
		//let io = this.options.io || window.SockJS;
		this.intake = new FlowEvents();
		this.socket = {
			on : (...args) => { this.intake.on(...args); },
			emit : (subject, msg) => { this.socket_impl.send(JSON.stringify([subject,msg])); }
		}
		this.socket.on('auth.setcookie', (msg)=>{
			document.cookie = msg.cookie;
		})
		this.socket.on('auth.getcookie', ()=>{
			let cookie = (document.cookie.length === 0) ? null : document.cookie;
			let response = {
				cookie: cookie
			}
			this.socket_impl.send(JSON.stringify(['auth.cookie', response]));
		})
		this.socket.on('ready', (msg) => {
			if(msg.websocketMode != this.options.websocketMode)
				throw new Error(`Error - incompatible websocket mode: client ${this.options.websocketMode} server: ${msg.websocketMode}`);
			this.events.emit('connect');
		})

		this.connect_impl();

		await this.initSocketHandlers();

		let timeoutMonitor = ()=>{
			let ts = Date.now();
			let purge = [ ]
			this.pending.forEach((info, id)=>{
				if(ts - info.ts > this.timeout * 1000) {
					info.callback({ error : "Timeout "});
					purge.push(id);
				}
			});
			purge.forEach(id=>{
				this.pending.delete(id);
			});
			dpc(1000, timeoutMonitor);
		}
		dpc(1000, timeoutMonitor);

	}

	close(){
		if(this.socket)
			this.socket.close();
	}

    
	on(op, callback) {
		this.events.on(op, callback);
	}


	initSocketHandlers() { return Promise.resolve(); }
}


"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_SOCKET_RPC_400 : &'static str = r###"




export class FlowSocketRPC extends FlowSocket {
	constructor(options) {
		super(Object.assign({},options,{websocketMode:'RPC'}));
		this.asyncSubscribers = new AsyncQueueSubscriberMap();
	}

	initSocketHandlers() {

		this.socket.on('publish', msg=>{
			let {subject, data} = msg;
			if(this.trace) {
				if(this.trace === 1 || this.trace === true)
					console.log('RPC ['+this.id+']:', subject);
				else
				if(this.trace === 2)
					console.log('RPC ['+this.id+']:', subject, data);
			}
			this.asyncSubscribers.post(subject, {data});
		})

		this.socket.on('response', (msg)=>{
			let {rid, error, data} = msg;
			let info = rid && this.pending.get(rid);
			if(info) {
				try {
					info.callback.call(this, error, data);
				} catch(ex) {
					console.error('RPC handler error:', msg);
					console.error(ex);
				}
			}
			else if(rid) {
				console.log("RPC received unknown rpc callback (strange server-side retransmit?)");
			}

			rid && this.pending.delete(rid);
		})

		return Promise.resolve();
	}

	subscribe(subject) {
		return this.asyncSubscribers.subscribe(subject);
	}

	publish(subject, data) {
		return this.socket.emit('publish', {subject, data});
	}

	request(subject, data, callback) {
		return new Promise((resolve, reject) => {

			if(typeof(data)=='function') {
				callback = data;
				data = undefined;
			}

			let rid = UID();

			this.pending.set(rid, {
				ts:Date.now(),
				callback : (err, resp) => {
					if(callback)
						callback(err,resp);
					else
					if(err)
						reject(err);
					else
						resolve(resp);
				}
			});

			this.socket.emit('request', {
				rid,
				req : {subject, data}
			});
		})
	}

}


"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_SOCKET_NATS_359 : &'static str = r###"




export class FlowSocketNATS extends FlowSocket {
	constructor(options) {
		super(Object.assign({},options,{websocketMode:'NATS'}));
		this.trace = false;
		this.subscribers = new Map();
		this.subscriptionTokenMap = new Map();
		this.handlers = new Map();

		this.asyncSubscribers = new AsyncQueueSubscriberMap();
	}

	initSocketHandlers() {

		this.on('connect', ()=>{
			// reconnect subscribers
			this.connectSubscribers(this.asyncSubscribers);
		});

		this.socket.on('message', msg=>{
			this.trace && console.log("sio/message",msg);
			let {subject, data} = msg;
			if(msg.op){
				subject = msg.op;
				data = msg;
			}
			if(this.trace) {
				if(this.trace === 1 || this.trace === true)
					console.log('RPC ['+this.id+']:', subject);
				else
				if(this.trace === 2)
					console.log('RPC ['+this.id+']:', subject, data);                
			}
			this.events.emit(subject, data);
		})

		this.socket.on('publish::response', (msg)=>{
			this.trace && console.log("sio/publish::response",msg);
			let {rid, error, data} = msg;
			let info = rid && this.pending.get(rid);
			if(info)
				info.callback.call(this, error, data);
			else if(rid)
				console.log("RPC received unknown rpc callback (strange server-side retransmit?)");

			rid && this.pending.delete(rid);
		})

		this.socket.on('subscribe::response', (msg)=>{
			this.trace && console.log("sio/subscribe::response",msg);
			let {rid, error, token, subject} = msg;
			let info = rid && this.pending.get(rid);
			if(info) {
				info.callback.call(this, error, token);
				let handler = this.handlers.get(rid);
				if(!error && subject?.length && handler) {
					let subscribers = this.subscribers.get(subject);
					if(!subscribers){
						subscribers = new Map();
						this.subscribers.set(subject, subscribers)
					}
					//subscribers.set(rid, {token, handler});
					subscribers.set(token, {token, handler});
					this.handlers.delete(rid);
					this.subscriptionTokenMap.set(token, { subscribers, rid });
				}
			}
			else if(rid)
				console.log("RPC received unknown subscribe::response rid");

			rid && this.pending.delete(rid);
		})

		this.socket.on('unsubscribe::response', (msg)=>{
			this.trace && console.log("sio/subscribe::response",msg);
			let {rid, error, ok} = msg;
			let info = rid && this.pending.get(rid);
			if(info)
				info.callback.call(this, error, ok);
			else if(rid)
				console.log("RPC received unknown unsubscribe::response rid");

			rid && this.pending.delete(rid);
		})


		this.socket.on('request', (msg)=>{
			this.trace && console.log("sio/request",msg);
			let { req : { subject, data }, rid } = msg;
			
			// TODO - check this.events for handlers
		})


		this.socket.on('publish', (msg)=>{
			this.trace && console.log("sio/publish",msg);
			let { subject, data,  token} = msg;

//			console.log("PUBLISH",msg);

			//this.events.emit(subject, data);
			const subscribers = this.subscribers.get(subject);
			if(subscribers){
				const target = subscribers.get(token);
				if(target){
					//console.log("PUBLISHING     SUBJECT::::", subject, "  TOKEN:", token, "    DATA:", data);
					target.handler(data);
				}
				//subscribers.forEach(subscriber=>subscriber.handler(data));
			}
			// TODO - check this.events for handlers

			this.asyncSubscribers.post(subject, {data});
		})

		this.socket.on('response', (msg)=>{
			this.trace && console.log("sio/response",msg);
			let {rid, error, data} = msg;
			error = error || data?.error;
			if(error?.code == "TIMEOUT" && !error.error)
				error.error = "NATS TIMEOUT";
			let info = rid && this.pending.get(rid);
			if(info)
				info.callback.call(this, error, data);
			else if(rid)
				console.log("NATS RPC received unknown rpc callback (strange server-side retransmit?)");

			rid && this.pending.delete(rid);
		})

		return Promise.resolve();
	}

	on(op, callback) {
		this.events.on(op, callback);
	}

	request(subject, data, callback) {
		return new Promise((resolve, reject) => {
			if(typeof(data)=='function'){
				callback = data;
				data = undefined;
			}

			let rid = UID();
			this.pending.set(rid, {
				ts:Date.now(),
				callback : (error, data)=>{
					if(callback) {
						callback(error, data);
					} else {
						if(error) {
							console.log('NATS request error - Subject:',subject,'Error:',error)
							reject(error);
						}
						else
							resolve(data);
					}
				}
			})

			this.socket.emit('request', {
				rid,
				req : {subject, data}
			});
		});
	}

	publish(subject, data, callback) {
		return new Promise((resolve, reject) => {
			let rid = UID();

			let ack = !!callback;

			ack && this.pending.set(rid, {
				ts:Date.now(),
				callback : (err, data)=>{
					if(typeof callback == 'function')
						return callback(err, data);
					else
						return err ? reject(err) : resolve();
				}
			})

			this.socket.emit('publish', {
				req : { subject, data },
				rid,
				ack
			})
		});
	}


	subscribe(subject, opt) {

		//this.create_nats_subscription_(subject);

		let subscriber = this.asyncSubscribers.subscribe(subject);

		subscriber.on('subscribe',(subject)=>{
			this.registerSubscriptionWithNATS_(subject, subscriber, opt);	
		})

		subscriber.on('unsubscribe',(subject)=>{
			this.unsubscribe(subscriber);	
		})

//		this.registerSubscriptionWithNATS_(subject, subscriber, opt);
		// console.log("NATS SUBSCRIPTION:", subject);
		return subscriber;
	}

	connectSubscribers(subscribers) {

		// this.asyncSubscribers
		subscribers.forEach((subscriber)=>{
			const { subject } = subscriber;
			subscriber.ready = this.registerSubscriptionWithNATS_(subject, subscriber);
		});

	}


	registerSubscriptionWithNATS_(subject, subscriber, opt = { }) {
		subscriber.state = 'connecting';
		let rid = UID();
		let p = new Promise((resolve, reject)=>{
			// if(handler)
			// 	this.handlers.set(rid, handler);
			this.pending.set(rid, {
				ts:Date.now(),
				callback:(err, token)=>{
					if(err)
						console.error('subscribe failure for subject:',subject);
					subscriber.token = token;
					subscriber.state = 'connected';
					//console.log("NATS - SUCCESSFUL SUBSCRIPTION to",subject,token,subscriber.state);
					return err?reject(err):resolve(subscriber);
				}
			});

			this.socket.emit('subscribe', {
				req : { subject, opt },
				rid
			})
		});
		p.rid = rid;
		return p;
	}


	unsubscribe(subscriber) {
//		this.unsubscribe_local_refs(token);
//		if(subscriber.state == 'connecting')

		const { token } = subscriber;
		let rid = UID();
		let p = new Promise((resolve, reject)=>{
			this.pending.set(rid, {
				ts:Date.now(),
				callback:(err, ok)=>{
					return err?reject(err):resolve(ok);
				}
			});

			this.socket.emit('unsubscribe', {
				req : { token },
				rid
			})
		});
		p.rid = rid;
		return p;
	}

/*
	unsubscribe_local_refs(token) {
		// this.subscriptionTokenMap.set(token, { subscribers, rid });
		let rec = this.subscriptionTokenMap.get(token);
		if(!rec)
			return;

		const { rid } = rec;

		this.handlers.delete(rid);
		this.pending.delete(rid);
		this.subscribers.forEach(subscribers=>{
			subscribers.delete(rid);
		})
	}
*/
}


"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_APP_381 : &'static str = r###"






/**
* @class FlowApp
* @extends BaseElement
* @example <flow-app></flow-app>
* 
* @property {String} ["menu-icon"] 
* @property {Boolean} [floating-drawer]
* @property {Boolean} [open-drawer]
* @cssvar {font-family} [--flow-btn-font-family=var(--flow-font-family, initial)]
* @cssvar {width} [--flow-app-width=100vw]
* @cssvar {height} [--flow-app-height=100vh]
* @cssvar {align-items} [--flow-app-header-align-items=center]
* @cssvar {height} [--flow-app-header-height=60px]
* @cssvar {background-color} [--flow-app-header-bg=#161926]
* @cssvar {color} {--flow-app-header-color=#91aec1]
* @cssvar {padding} [--flow-app-header-padding=0px 100px]
* @cssvar {padding} [--flow-app-header-link-padding=0px 0px 0px 16px]
* @cssvar {color} [--flow-app-header-color=#91aec1]
* @cssvar {background-color} [--flow-app-drawer-bg=var(--flow-background-color, inherit)]
* @cssvar {color} [--flow-app-drawer-color=var(--flow-color, inherit)]
* @cssvar {width} [--flow-app-drawer-width=300px]
* @cssvar {overflow} [--flow-app-drawer-overflow=initial]
* @cssvar {transition} [--flow-app-drawer-transition=left 0.5s ease]
* @cssvar {z-index} [--flow-app-drawer-z-index=10001]
* @cssvar {left} [--flow-app-drawer-hidden-left=-500px]
* @cssvar {transition} [--flow-app-drawer-transition=right 0.5s ease]
* @cssvar {right} [--flow-app-drawer-hidden-right=-500px]
* @cssvar {overflow} [--flow-app-main-overflow=auto]
* @cssvar {position} [--flow-app-main-position=initial]
* @cssvar {display} [--flow-app-main-display=initial]
* @cssvar {align-items} (--flow-app-main-align-items=stretch]
* @cssvar {justify-content} [--flow-app-main-align-justify-content=space-between);
* @cssvar {height} [--flow-app-header-height=60px]
* @cssvar {padding} --flow-app-header-padding=0px 100px]
* @cssvar {z-index} --flow-app-body-mask-z-index=10000]
* @cssvar {background-color} [--flow-app-body-mask-bg=initial]
* @cssvar {padding} [--flow-app-header-sm-padding=0px 15px]
* #cssvar {overflow} [--flow-app-body-overflow=hidden]
*/
	// {--fa-icon-color} [--flow-app-drawer-close-icon-color=var(--flow-color)]
	// {--fa-icon-color} [--flow-app-menu-icon-color=var(--flow-color)]		
	// {--flow-dropdown-trigger-bg} [--flow-app-header-bg=#161926]
	// {--flow-dropdown-trigger-color} [--flow-app-header-color=#91aec1]
	// {--flow-dropdown-trigger-hover-color} [--flow-app-header-color=#91aec1]
export const FlowAppMixin = (baseClass)=>{
	class base extends baseClass{
		constructor(...args) {
			super(...args)
			this.uid = UID();
			this.setTheme(this.getTheme("light"));
			flow.app = this;
		}

		getDefaultRPC() { return this.defaultRPC; }

		getTheme(defaultTheme){
			return getTheme(defaultTheme);
		}

		setTheme(theme){
			setTheme(theme);
		}

		initSocketRPC(options={}){
			return new Promise((resolve, reject) => {
				this.rpc = new FlowSocketRPC(Object.assign({
					path:"/rpc"
				}, options||{}));

				this.rpc.events.on("connect", ()=>{
					this.log("RPC:init");
					console.log("READY!!!!!")
					this.onNetworkIfaceOnline();
					resolve();
				})
				this.rpc.events.on('disconnect', () => {
					console.log("disconnected...");
					this.onNetworkIfaceOffline();
				});

				this.defaultRPC = this.rpc;
			})
		}

		// taken from BaseElement...
		fireEvent(eventName, detail={}, options={}){
			let ev = new CustomEvent(eventName, Object.assign({}, options, {detail}));
			return window.dispatchEvent(ev);
		}
	
		initSocketNATS(options={}){
			return new Promise((resolve, reject) => {
				this.nats = new FlowSocketNATS(Object.assign({
					path:"/nats"
				}, options));

				this.nats.events.on("connect", ()=>{
					this.log("NATS:init");
					this.onNetworkIfaceOnline();
					resolve();
				})

				this.nats.events.on('disconnect', () => {
					console.log("disconnected...");
					this.onNetworkIfaceOffline();
				})

				this.defaultRPC = this.nats;
			})
		}

		setLoading(isLoading=true, el=null){
			(el || document.body).classList.toggle("loading", isLoading)
		}

		initLog(...args){
			if(super._initLog)
				return super._initLog(...args)
			const name = this.constructor.name;
			this.log = Function.prototype.bind.call(
				console.log,
				console,
				`%c[${name}]`,
				`font-weight:bold;color:#41c7ef`
			);
		}

		onNetworkIfaceOnline(){
			this._online = true;
			this.fireEvent('network-iface-online');
		}
		onNetworkIfaceOffline(){
			if(this._online) {
				this._online = false;
				this.fireEvent('network-iface-offline');
			}
		}

		isOnline(){
			return this._online;
		}
	}
	return base;
}

export class FlowAppLayout extends BaseElement{
	static get properties(){
		return {
			"menu-icon":{type:String},
			"floating-drawer":{type:Boolean, reflect:true},
			"open-drawer":{type:Boolean, reflect:true},
		}
	}
	static get styles(){
		return [ScrollbarStyle, css`
			:host{
				display:flex;
				flex-direction:column;
				width:var(--flow-app-width, 100vw);
				height:var(--flow-app-height, 100vh);
			}
			.header{
				display:flex;flex-direction:row;
				align-items:var(--flow-app-header-align-items, center);
				height:var(--flow-app-header-height, 60px);
				background-color:var(--flow-app-header-bg, #161926);
				color:var(--flow-app-header-color, #91aec1);
				padding:var(--flow-app-header-padding, 0px 100px);
				--flow-dropdown-trigger-bg:var(--flow-app-header-bg, #161926);
				--flow-dropdown-trigger-color:var(--flow-app-header-color, #91aec1);
				--flow-dropdown-trigger-hover-bg:transparent;
				--flow-dropdown-trigger-hover-color:var(--flow-app-header-color, #91aec1);
				--flow-dropdown-trigger-width:20px;
				--flow-dropdown-trigger-padding:0px;
			}
			:host([no-header]) .header{display:none}
			.header-sm{display:none}
			.header ::slotted(.logo){
				height:100%;
				max-height:80%;
			}
			.logo ::slotted(.logo){
				max-height:80%;
			}
			.header ::slotted(a.link){
				padding:var(--flow-app-header-link-padding, 0px 0px 0px 16px);
				color:var(--flow-app-header-color, #91aec1);
				text-decoration:none;
			}
			.footer {
				height: var(--flow-app-footer-height, initial);
				color:var(--flow-app-footer-color, #000);
				background:var(--flow-app-footer-bg, #91aec1);				
			}
			
			.body{
				flex:1;display:flex;flex-direction:row;
				overflow:var(--flow-app-body-overflow, hidden);
			}
			.drawer{
				background-color:var(--flow-app-drawer-bg, var(--flow-background-color, inherit));
				color:var(--flow-app-drawer-color, var(--flow-color, inherit));
				width:var(--flow-app-drawer-width, 300px);
				overflow:var(--flow-app-drawer-overflow, initial);
				position:relative;
			}
			:host([no-drawer]) .drawer{display:none}
			:host([floating-drawer]) .drawer{
				position:absolute;
				left:0px;top:0px;bottom:0px;
				transition:var(--flow-app-drawer-transition, left 0.5s ease);
				z-index:var(--flow-app-drawer-z-index, 10001);
			}
			:host([floating-drawer]:not([open-drawer])) .drawer{
				left:var(--flow-app-drawer-hidden-left, -500px);
			}
			:host([floating-drawer][right-drawer]) .drawer{
				left:initial;right:0px;
				transition:var(--flow-app-drawer-transition, right 0.5s ease);
			}
			:host([floating-drawer][right-drawer]:not([open-drawer])) .drawer{
				right:var(--flow-app-drawer-hidden-right, -500px);
			}
			.main{
				flex:1;
				overflow:var(--flow-app-main-overflow, hidden);
				position:var(--flow-app-main-position, initial);
				display:var(--flow-app-main-display, flex);
				flex-direction:var(--flow-app-main-flex-direction, column);
			}

			.wrapper {
				/*
				min-height: 100%;
				height:var(--flow-app-wrapper-height, 100%);
				margin-bottom: calc(-1 * var(--flow-app-footer-height,0px));
				*/
				height:var(--flow-app-wrapper-height, 100px);
				overflow:var(--flow-app-wrapper-overflow, auto);
				position:var(--flow-app-wrapper-position, initial);
				display:var(--flow-app-wrapper-display, initial);
				flex:var(--flow-app-wrapper-flex, 1);
			}
			:host([main-v-box]) .main{
				display:flex;flex-direction:column;
				align-items:var(--flow-app-main-align-items, stretch);
				justify-content:var(--flow-app-main-align-justify-content, space-between);
			}
			fa-icon{
				--fa-icon-color:var(--flow-color);
			}

			.menu-icon{
				cursor:pointer;
				--fa-icon-color:var(--flow-app-menu-icon-color, var(--flow-color));
			}
			.drawer-top{
				height:var(--flow-app-header-height, 60px);
				display:flex;align-items:center;
				padding:var(--flow-app-header-padding, 0px 100px);
			}
			.main-mask{
				position:absolute;z-index:var(--flow-app-body-mask-z-index, 10000);
				left:0px;top:0px;right:0px;bottom:0px;width:100%;height:100%;
				background-color:var(--flow-app-body-mask-bg, initial);
			}
			:host([floating-drawer][open-drawer]) .main{position:relative}
			:host(:not([floating-drawer])) .drawer-top,
			:host(:not([floating-drawer])) .main-mask,
			:host(:not([open-drawer])) .main-mask{display:none}
			.drawer-close-icon{
				cursor:pointer;
				--fa-icon-color:var(--flow-app-drawer-close-icon-color, var(--flow-color));
			}

			@media(max-width:760px){
				:host([floating-drawer]) .header-sm{display:flex}
				:host([floating-drawer]) .header:not(.header-sm){display:none}
				.drawer-top,
				.header{
					padding:var(--flow-app-header-sm-padding, 0px 15px);
				}
			}



			::slotted(.flex){flex:1}
		`]
	}

	render(){
		return html`
		<div class="header"><slot name="logo"></slot><slot name="header"></slot></div>
		<div class="header header-sm"><fa-icon class="menu-icon"
			icon="${this['menu-icon'] || 'bars'}" 
			@click="${this.toggleFloatingDrawer}"></fa-icon><slot 
			name="header-sm"></slot></div>
		<div class="body">
			<div class="drawer sbar">
			<div class="drawer-top">
				<fa-icon class="drawer-close-icon"
				icon="${this['drawer-close-icon'] || 'times'}" 
				@click="${this.toggleFloatingDrawer}"></fa-icon>
			</div>
			<slot name="drawer"></slot></div>
			<div class="main sbar">
				<div class="wrapper">
					<slot name="main"></slot><div 
					class="main-mask" @click="${this.toggleFloatingDrawer}"></div>
				</div>
				<div class="footer">
					<slot name="footer"></slot>
				</div>
			</div>
		</div>
		`
	}

	toggleFloatingDrawer(){
		if(!this['floating-drawer'])
			return
		this['open-drawer'] = !this['open-drawer'];
	}
}

FlowAppLayout.define("flow-app-layout");


export class FlowApp extends FlowAppMixin(BaseElement){
	static get properties(){
		return {
			"menu-icon":{type:String},
			"floating-drawer":{type:Boolean, reflect:true},
			"open-drawer":{type:Boolean, reflect:true},
			"external-dom":{type:Boolean}
		}
	}
	constructor(...args){
		super(...args);
		this.registerListener("flow-network-and-user-state-get", (e)=>{
			//e.detail = e.detail||{};
			e.detail.online = this.isOnline();
			e.detail.signedin = this.signedin;
		})
	}
	createRenderRoot(){
		this.usingExternalDom = this.getAttribute("external-dom")!=null 
			|| this.querySelector("flow-app-layout, .flow-app-layout")
		if(this.usingExternalDom)
			return this.attachShadow({mode:"open"});
		return this;
	}
	render(){
		if(this.usingExternalDom)
			return html`<slot></slot>`;

		return html``;
	}
	firstUpdated(){
		this.setLoading(false);
	}
	signinCallback(){
		this.signedin = true;
		document.body.classList.toggle("flow-user-signedin", true);
	}
	signoutCallback(){
		this.signedin = false;
		document.body.classList.toggle("flow-user-signedin", false);
	}
	connectedCallback() {
		super.connectedCallback();
		this.signinCallback_ = (...args)=>{
			this.signinCallback(...args);
		}
		window.addEventListener('flow-user-signin', this.signinCallback_);

		this.signoutCallback_ = (...args)=>{
			this.signoutCallback(...args);
		}
		window.addEventListener('flow-user-signout', this.signoutCallback_);
	}
	disconnectedCallback() {
		super.disconnectedCallback();
		if(this.signinCallback_){
			window.removeEventListener('flow-user-signin', this.signinCallback_);
			delete this.signinCallback_;
		}
		if(this.signoutCallback_){
			window.removeEventListener('flow-user-signout', this.signoutCallback_);
			delete this.signoutCallback_;
		}
	}
}

FlowApp.define("flow-app");


"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_APP_DRAWER_368 : &'static str = r###"



/**
* @class FlowAppDrawer
* @extends BaseElement

* @property {Boolean} [disabled]
* @property {String} [btnText]
* @property {String} [value]
* @property {Boolean} [opened]
* @property {String} [parentSelector]
*
* @cssvar {font-family} [--flow-font-family="Julius Sans One"]
* @cssvar {font-weight} [--flow-font-weight=bold]
* @cssvar {border-radius} [--flow-appdrawer-btn-border-radius=50%]
* @cssvar {box-shadow} [--flow-appdrawer-btn-box-shadow=0px 0px 3px 1px var(--flow-background-inverse-soft))]
* @cssvar {padding} [--flow-appdrawer-btn-padding=10px]
* @cssvar {right} [--flow-appdrawer-btn-right=-10px]
* @cssvar {top} [--flow-appdrawer-btn-right=10px]
* @cssvar {background-color} [--flow-appdrawer-btn-bg=var(--flow-background-color)]
* @cssvar {width} [-flow-appdrawer-btn-width=15px]
* @cssvar {height} [--flow-appdrawer-btn-height=15px]
* @cssvar {left} [--flow-appdrawer-btn-left=15px]
* @cssvar {top} [--flow-appdrawer-btn-top=5px]
* @cssvar {top} [--flow-appdrawer-wrapper-top=20px]
* @cssvar {border} [--flow-appdrawer-border=2px solid var(--flow-primary-color]

* @example
*   <flow-app-drawer></flow-app-drawer>
*
*/
export class FlowAppDrawer extends BaseElement {
	static get properties() {
		return {
			opened:{type:Boolean, reflect:true},
			parentSelector:{type:String}
		}
	}

	static get styles(){
		return css`
			:host{
				display:block;
				position:absolute;left:0px;top:0px;right:0px;bottom:0px;
			}
			.warpper{
				position:absolute;left:0px;top:0px;right:0px;bottom:0px;
				overflow:auto;
			}
			.toggle-btn{
				border-radius:var(--flow-appdrawer-btn-border-radius, 50%);
				box-shadow:var(--flow-appdrawer-btn-box-shadow, 0px 0px 3px 1px var(--flow-background-inverse-soft));
				padding:var(--flow-appdrawer-btn-padding, 10px);
				position:absolute;
				right:var(--flow-appdrawer-btn-right, -10px);
				top:var(--flow-appdrawer-btn-right, -10px);
				background-color:var(--flow-appdrawer-btn-bg, var(--flow-background-color));
				width:var(--flow-appdrawer-btn-width, 15px);
				height:var(--flow-appdrawer-btn-height, 15px);
				z-index:1000;box-sizing:border-box;cursor:pointer;
			}

			:host(.top-btn) .toggle-btn{
				right:initial;
				left:var(--flow-appdrawer-btn-left, 15px);
				top:var(--flow-appdrawer-btn-top, 5px);
			}
			:host(.top-btn) .warpper{
				top:var(--flow-appdrawer-wrapper-top, 20px);
			}
			.toggle-btn:after{
				content:"";
				border:var(--flow-appdrawer-border, 2px solid var(--flow-primary-color));
				border-left-color:transparent;
				border-bottom-color:transparent;
				box-sizing:border-box;
				width:50%;height:50%;
				position:absolute;
				left:15%;
				top:25%;
				transition:transform 0.2s ease;
				transform-origin:center;
				transform:rotate(45deg);
			}
			:host([opened]) .toggle-btn:after{
				transform:translateX(4px) rotate(225deg);
			}
			:host(.hide-btn) .toggle-btn{display:none}

		`;
    }
    constructor() {
        super();
    }
	render() {
		return html`<div class="warpper"><slot></slot></div>
		<span class="toggle-btn" @click="${this.toggle}"></span>`;
	}
	toggle() {
		this.opened = !this.opened;
	}
	updated(changes){
		super.updated(changes);
		if(changes.has("opened")){
			let el = document.querySelector(this.parentSelector || "body");
			el?.classList.toggle("drawer-opened", this.opened);
		}
	}
}

FlowAppDrawer.define('flow-app-drawer');

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_DROPDOWN_352 : &'static str = r###"


/**
 * @export
 * @class FlowDropdown
 * @extends {BaseElement}

 * @prop {Boolean} opened 
 * @prop {Boolean} disabled
 * 
 * @cssvar {color} [--flow-color, #000);
 * @cssvar {color}[--flow-dropdown-trigger-color=#FFF]
 * @cssvar {color} [--flow-dropdown-trigger-hover-color=#FFF]
 * @cssvar {color} [--flow-dropdown-color=var(--flow-color, #000)]
 * @cssvar {font-size} [--flow-dropdown-trigger-font-size=var(--flow-input-font-size, 1rem)]
 * @cssvar {font-weight} [--flow-dropdown-trigger-font-weight=var(--flow-input-font-weight, 400)]
 * @cssvar {line-height} [--flow-dropdown-trigger-line-height=var(--flow-input-line-height, 1.2)]
 * @cssvar {background-color} [--flow-dropdown-trigger-hover-bg=var(--flow-primary-color, #3498DB)]
 * @cssvar {background-color} [--flow-dropdown-trigger-bg=var(--flow-primary-color, #3498DB)]
 * @cssvar {background-color} [--flow-dropdown-bg=var(--flow-background-color, #FFF)]
 * @cssvar {box-shadow} [--flow-box-shadow]
 * @cssvar {box-shadow} [--flow-box-shadow]
 * @cssvar {min-width} [--flow-dropdown-trigger-width=80px]
 * @cssvar {min-width} [--flow-dropdown-content-min-width=160px]
 * @cssvar {min-width} [--flow-dropdown-trigger-width=80px]
 * @cssvar {top} [--flow-dropdown-top=0px]
 * @cssvar {padding} [--flow-dropdown-trigger-padding, 21px 20px 20px);
 * @cssvar {padding} [--flow-dropdown-content-padding=5px]
 * @cssvar {padding} [--flow-dropdown-trigger-padding=21px 20px 20px]
 * @cssvar {border} [--flow-dropdown-border=none]
 * 
 * @example
 * <flow-dropdown>
 *   <div slot="trigger">Menu</div>
 *   <ul>
 *		<li>Menu Item 1</li>
 *	 	<li>Menu Item 2</li>
 *   </ul>
 * </flow-dropdown>
 */
export class FlowDropdown extends BaseElement {
	static get properties() {
		return {
			opened:{type:Boolean, reflect:true},
			modal:{type:Boolean},
			backdrop:{type:Boolean},
			disabled:{type:Boolean, reflect:true},
			absolute:{type:Boolean, reflect:true},
			"right-align":{type:Boolean},
			xMargin:{type:Number, value:10},
			yMargin:{type:Number, value:10},
			yOffset:{type:Number, value:8},
			yOverflowMargin:{type:Number, value:-10}
		}
	}

	static get styles() {
		return [ScrollbarStyle, css`
		:host{
			display:var(--flow-dropdown-display, inline-block);margin:5px 0px;
			vertical-align:middle;
			color:var(--flow-color, #000);
		}
		.trigger{
			background-color:var(--flow-dropdown-trigger-bg, var(--flow-primary-color, #3498DB));
			color:var(--flow-dropdown-trigger-color, #FFF);
			border-radius:var(--flow-dropdown-trigger-border-radius, 3px);
			border:none;
			user-select:none;
			padding:var(--flow-dropdown-trigger-padding, 21px 20px 20px);
			min-width:var(--flow-dropdown-trigger-width, 80px);
			font-size:var(--flow-dropdown-trigger-font-size, var(--flow-input-font-size, 1rem));
			font-weight:var(--flow-dropdown-trigger-font-weight, var(--flow-input-font-weight, 400));
			line-height:var(--flow-dropdown-trigger-line-height, var(--flow-input-line-height, 1.2));
		}
		:host(:not([disabled])) .trigger{cursor:pointer;}

		.trigger:hover, .trigger:focus {
			background-color:var(--flow-dropdown-trigger-hover-bg, var(--flow-primary-color, #3498DB));
			color:var(--flow-dropdown-trigger-hover-color, #FFF);
		}
		.dropdown{position:relative;display:block;}
		.dropdown-content{
			display:none;
			position:absolute;
			background-color:var(--flow-dropdown-bg, var(--flow-background-color, #FFF));
			color:var(--flow-dropdown-color, var(--flow-color, #000));
			min-width:var(--flow-dropdown-content-min-width, 160px);
			overflow:auto;border-radius:3px;
			box-shadow:var(--flow-box-shadow);
			z-index:1000;
			top:var(--flow-dropdown-content-top, var(--flow-dropdown-top, 0px));
			left:var(--flow-dropdown-content-left, 0px);
			right:var(--flow-dropdown-content-right, initial);
			padding:var(--flow-dropdown-content-padding, 5px);
			border:var(--flow-dropdown-border, none);
			box-sizing:border-box;
			transform:translate(var(--flow-transform-translate-x), var(--flow-transform-translate-y));
		}
		:host([opened]) .dropdown-content,
		:host([opened]:not([absolute])) .backdrop{display:block;}
		:host(.right-align) .dropdown-content,
		:host([right-align]) .dropdown-content{
			left:var(--flow-dropdown-content-left, initial);
			right:var(--flow-dropdown-content-right, 0px);
		}
		:host([no-trigger]) .trigger{display:none}
		:host([no-trigger]){margin:0px;}
		:host([absolute]) .dropdown-content{
			position:absolute;
		}
		:host(:not([absolute])) .dropdown-content{
			position:fixed;z-index:1010;
		}
		:host(:not([absolute])) .backdrop{
			position:fixed;z-index:1009;
			top:0px;
			bottom:0px;
			left:0px;
			right:0px;
			display:none;
			background:var(--flow-dropdown-backdrop-bg, var(--flow-backdrop-bg))
		}
		`];
	}
	render() {
		return html`
			<div class="trigger"><slot name="trigger"></slot></div><div class="dropdown">
				<div class="dropdown-content">
					<slot></slot>
				</div>${this.renderBackdrop()}
			</div>
		`;
	}
	renderBackdrop(){
		if(!this.backdrop && !isSmallScreen)
			return '';
		
		return html`<a href="javascript:void(0)" class="backdrop"></a>`;
	}
	constructor(){
		super();
		this.initPropertiesDefaultValues();
	}
	firstUpdated(){
		if(this.classList.contains("right-align"))
			this["right-align"] = true;
		this.triggerEl = this.renderRoot.querySelector(".trigger");
		this.triggerEl.addEventListener("click", this._onClick.bind(this));
		this.dropdownEl = this.renderRoot.querySelector(".dropdown");
		this.dropdownContentEl = this.renderRoot.querySelector(".dropdown-content");
	}
	updated(changes){
		this.updateDropdownSize();
		if(changes.has("opened"))
			this.fire(this.opened?"opened":"closed", {opened:this.opened, dd:this})
	}

	_onClick(e){
		if(this.disabled)
			return
		this.toggle();
	}

	open(){
		this.opened = true;
	}
	close(){
		this.opened = false;
	}
	toggle(){
		this.opened = !this.opened;
	}
	onWindowClick(e){
		if(!this.opened)
			return
		let target = e.target;
		//let log = document.createElement("pre");
		//let data = [];
		//for (let k in e){
		//	data.push(k)
		//}
		//log.innerHTML = JSON.stringify(data, null, " ");
		//target.parentNode.appendChild(log)
		if(!target){
			this.opened = false;
			return
		}
		let dropdown = target.flowDropdown || target.closest?.('flow-dropdown');
		let isBackdrop = target.classList?.contains("backdrop")||false;
		if(!dropdown){
			let path = e.path || (typeof e.composedPath == "function" ? e.composedPath() : null);
			let p = path?.[0] || target;
			while(p){
				isBackdrop = isBackdrop||p.classList?.contains("backdrop")||false;
				//data.push(p.tagName)
				if(p.flowDropdown){
					dropdown = p.flowDropdown;
					break;
				}
				if(p.matches?.("flow-dropdown")){
					dropdown = p;
					break;
				}
				if(p instanceof ShadowRoot){
					p = p.host;
					continue;
				}
				p = p.parentNode;
			}
		}
		if(!dropdown || dropdown!=this){
			//log.innerHTML = JSON.stringify(data, null, " ");
			this.opened = false;
		//if clicked on this DD's backdrop and this DD is not a modal
		}else if(isBackdrop && !this.modal){
			this.opened = false;
		}
	}
	onWindowResize(){
		this.updateDropdownSize();
	}
	onParentScroll(){
		this.updateDropdownSize();
	}
	updateDropdownSize(){
		let {dropdownContentEl, dropdownEl, scrollParants, triggerEl} = this;

		if(!triggerEl||!dropdownContentEl||!dropdownEl||!scrollParants||!scrollParants.length)
			return

		let parentBox = dropdownEl.getBoundingClientRect();
		if(!this.absolute){
			let {left, bottom, right} = parentBox;
			let width = window.innerWidth;
			let height = window.innerHeight;
			let DCElStyle = dropdownContentEl.style;
			bottom -= this.yOffset;
			if(this["right-align"]){
				DCElStyle.maxWidth = `${right-this.xMargin}px`;
				DCElStyle.left = 'initial';
				DCElStyle.right = `${width-right}px`;
			}else{
				DCElStyle.maxWidth = `calc(100vw - ${left+this.xMargin}px)`;
				DCElStyle.right = 'initial';
				DCElStyle.left = `${left}px`;
			}
			DCElStyle.bottom = 'unset';
			DCElStyle.top = `${bottom}px`;
			let maxHeight = height-(bottom+this.yMargin);
			DCElStyle.maxHeight = `${maxHeight}px`;//`calc(100vh - ${bottom+this.yMargin}px)`;
			//let box = dropdownContentEl.getBoundingClientRect();
			//console.log("box.height", height, maxHeight, DCElStyle.maxHeight);
			let triggerELBox = triggerEl.getBoundingClientRect();
			let maxHeightInUpperPosition = triggerELBox.top+this.yOverflowMargin;
			if (maxHeight < maxHeightInUpperPosition){
				DCElStyle.top = 'unset';
				DCElStyle.bottom = `calc(100% - ${triggerELBox.top}px)`;
				DCElStyle.maxHeight = `${maxHeightInUpperPosition}px`;
			}
			return
		}

		//let box = dropdownContentEl.getBoundingClientRect();
		let firstScrollParent = scrollParants[0];
		//console.log("firstScrollParent", firstScrollParent)
		let firstScrollParentBox = firstScrollParent.getBoundingClientRect();
		

		let topMargin = Math.max(parentBox.top - firstScrollParentBox.top, 0);
		let top = Math.max(firstScrollParentBox.top - parentBox.top, 0);
		let height = firstScrollParentBox.height - topMargin - 20

		let leftMargin = Math.max(parentBox.left - firstScrollParentBox.left, 0);
		let left = Math.max(firstScrollParentBox.left - parentBox.left, 0);
		let width = firstScrollParentBox.width - leftMargin - 20

		/*this.log("width, height",
			top,
			topMargin,
			firstScrollParent.scrollTop,
			firstScrollParentBox,
			parentBox,
			width,
			height
		)*/
		DCElStyle.transform = `translate(${left}px, ${top}px)`;
		DCElStyle.maxWidth = width+"px";
		DCElStyle.maxHeight = height+"px";
	}
	connectedCallback(){
    	super.connectedCallback();
    	this._onWindowClick = this._onWindowClick||this.onWindowClick.bind(this);
    	this._onWindowResize = this._onWindowResize||this.onWindowResize.bind(this);
    	this._onParentScroll = this._onParentScroll||this.onParentScroll.bind(this);
    	window.addEventListener("click", this._onWindowClick, {capture:true})
    	window.addEventListener("resize", this._onWindowResize, {capture:true});
    	let buildScrollEvents = ()=>{
	    	this.scrollParants = this.findScrollParents();
	    	this.scrollParants.forEach(p=>{
	    		p.addEventListener("scroll", this._onParentScroll);
	    	});
	    }
	    buildScrollEvents();
    	if(!this.scrollParants.length){//Safari/FF issue
    		dpc(1000, ()=>{
    			buildScrollEvents();
    			//this.log("this.scrollParants", this.scrollParants)
    		});
    	}
    }
	disconnectedCallback(){
    	super.disconnectedCallback();
    	window.removeEventListener("click", this._onWindowClick);
    	window.removeEventListener("resize", this._onWindowResize);
    	this.scrollParants.forEach(p=>{
    		p.removeEventListener("scroll", this._onParentScroll);
    	});
    	this.scrollParants = [];
    }
    findScrollParents(){
    	let list = [];
    	let p = this.parentNode;
    	while(p){
    		if(p instanceof ShadowRoot){
    			p = p.host;
    			continue;
    		}
    		if(!(p instanceof HTMLElement))
    			break;
    		if(p.nodeName=="BODY"){
    			list.push(p);
    			break;
    		}
    		if(this.isScrollEl(p))
    			list.push(p);
    		p = p.parentNode;
    	}

    	return list;
    }

    isScrollEl(element){
    	const { overflow, overflowX, overflowY } = getComputedStyle(element);
    	//this.log("overflow:::", element, overflow, overflowX, overflowY)
  		return /auto|scroll|overlay|hidden/.test(overflow + overflowY + overflowX);
    }
}

FlowDropdown.define('flow-dropdown');

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_MENU_374 : &'static str = r###"


/**
 * @export
 * @class FlowMenu
 * @extends {BaseElement}


 * 
 * @example
 * <flow-menu selected="one">
 * 	<flow-menu-item value="one">One</flow-menu-item>
 * 	<flow-menu-item value="two">Two</flow-menu-item>
 * </flow-menu>
 */

 export class SelectOption{
	constructor(text, value){
		this.text = text;
		this.value = value;
	}
}

export class FlowMenu extends BaseElement {
	static get properties() {
		return {
			selected:{type:String, reflect:true},
			selector:{type:String},
			valueAttr:{type:String},
			multiple:{type:Boolean}
		}
	}

	static get styles() {
		return css`
		:host{
			display:block;padding:5px 0px;
			--flow-menu-item-margin-internal: var(--flow-menu-item-margin, 1px);
			--flow-menu-item-margin-internal2x:calc(var(--flow-menu-item-margin-internal) * 2);
		}

		::slotted(flow-menu-item){
			display:flex;align-items:center;
		}
		
		::slotted(flow-menu-item),
		::slotted(.menu-item),
		.menu-item{
			box-sizing:border-box;
			cursor:pointer;user-select:none;
			padding:var(--flow-menu-item-padding, 10px);
			margin:var(--flow-menu-item-margin-internal);
			background-color:var(--flow-menu-item-bg, var(--flow-background-color));
			color:var(--flow-menu-item-color, var(--flow-color));
			pointer-events:auto;
		}
		::slotted(flow-menu-item:hover:not(.disabled):not(.selected)),
		::slotted(.menu-item:hover:not(.disabled):not(.selected)),
		.menu-item:hover:not(.disabled):not(.selected){
			background-color:var(--flow-menu-item-hover-bg, #DDD);
			color:var(--flow-menu-item-hover-color, #000);
		}
		::slotted(flow-menu-item.selected),
		::slotted(.menu-item.selected),
		.menu-item.selected{
			background-color:var(--flow-menu-item-selected-bg, var(--flow-primary-color));
			color:var(--flow-menu-item-selected-color, var(--flow-primary-invert-color));
		}
		:host(.grid),
		:host(.grid) .menu-list-container{
			display:flex;
			flex-wrap:wrap;
			width:var(--flow-menu-grid-width, 500px);
		}
		:host(.grid.full),
		:host(.grid.full) .menu-list-container{
			width:var(--flow-menu-gridfull-width, 1000px);
		}
		:host(.grid:not(.full)) ::slotted(flow-menu-item),
		:host(.grid:not(.full)) ::slotted(.menu-item),
		:host(.grid:not(.full)) .menu-item{
			min-width:calc(20% - var(--flow-menu-item-margin-internal2x));
			max-width:calc(20% - var(--flow-menu-item-margin-internal2x));
		}
		:host(.grid.full) ::slotted(flow-menu-item),
		:host(.grid.full) ::slotted(.menu-item),
		:host(.grid.full) .menu-item{
			min-width:var(--flow-menu-gridfull-item-min-width, 100px);
			max-width:var(--flow-menu-gridfull-item-max-width, initial);
			flex:var(--flow-menu-gridfull-item-flex, 1);
		}
		::slotted(div.divider),
		::slotted(div.section){
			padding:var(--flow-menu-divider-padding, 10px);
			box-shadow:var(--flow-menu-divider-box-shadow, var(--flow-box-shadow));
			margin:var(--flow-menu-divider-margin, 0 0 5px 0);
			background-color:var(--flow-menu-divider-bg-color, var(--flow-background-inverse-soft));
			color:var(--flow-menu-divider-color, var(--flow-background-color));
		}
		::slotted(flow-menu-item.disabled),
		::slotted(.menu-item.disabled),
		.menu-item.disabled{
			cursor:var(--flow-menu-disabled-item-cursor, default);
			opacity:var(--flow-menu-disabled-item-opacity, 0.7);
		}
		`;
	}
	constructor(){
		super();
		this.selected = "";
		this.selector = "flow-menu-item, .menu-item";
		this.valueAttr = "value";
		this._selected = []
	}
	render(){
		return html`
		<slot></slot>
		`;
	}
	static SelectOption=SelectOption
	static createOption(text, value, cls=""){
		return {text, value, cls};
	}
	static createOptionItem(text, value, cls=""){
		let isDivider = cls.includes("divider")||cls.includes("section")
		let item = document.createElement(isDivider?"div":"flow-menu-item");
		if(cls){
			item.setAttribute("class", cls);
		}
		item.setAttribute("value", value);
		item.innerHTML = text;
		return item;
	}
	static createOptionItems(items=[]){
		return items.map(item=>{
			return this.createOptionItem(item.text, item.value, item.cls||"")
		})
	}
	changeOptions(items=[]){
		let children = this.querySelectorAll("*");
		children.forEach(c=>{
			c.remove();
		});

		FlowMenu.createOptionItems(items).forEach(el=>{
			this.appendChild(el)
		})
		this.onSlotChange();
	}
	firstUpdated(){
		this.renderRoot
			.addEventListener("click", this._onClick.bind(this));

		let slot = this.renderRoot.querySelector('slot');
		this.listSlot = slot;
		if(slot){
			slot.addEventListener('slotchange', (e)=>{
				//let items = slot.assignedElements();
				//this.items = items
				//TODO update selection 
				this.onSlotChange()
			});
		}
	}
	onSlotChange(){
		this.updateList();
	}
	updated(changes){
		//this.log("changes", changes)
		if(changes.has("selected")){
			this.parseSelected();
			this.requestUpdate("_selected")
		}

		this.updateList(changes)
		super.updated(changes)
	}

	parseSelected(){
		let {selected} = this;
		this.log("parseSelected:selected:"+JSON.stringify(selected), selected)
		if(this.multiple){
			if(!Array.isArray(selected)){
				try{
					selected = JSON.parse(selected);
				}catch(e){
					selected = undefined;
				}
				if(selected !== undefined)
					selected = [selected];
				else
					selected = [];
			}
		}else{
			if(Array.isArray(selected))
				selected = selected[0];
			if(selected !== undefined)
				selected = [selected];
			else
				selected = []
		}
		selected = selected.filter(s=>s!==undefined).map(s=>s+"");
		//this.log("updated:selected", selected)
		this.select(selected);
	}

	get list(){
		if(!this.listSlot)
			return [];
		return this.listSlot
			.assignedElements()
			.filter(item=>item.matches(this.selector))
	}

	updateList(){
		this.list.forEach(item=>{
			let value = item.getAttribute(this.valueAttr)
			item.onclick = ()=>{};//<--- iphone issue
			item.classList.toggle("selected", this.isSelected(value));
		});
	}

	isSelected(value){
		return this._selected.includes(value);
	}

	_onClick(e){
		let target = e.target.closest(this.selector);
		if(!target || target.classList.contains("disabled"))
			return
		let value = target.getAttribute(this.valueAttr);
		if(this.multiple)
			this.toggle(value);
		else
			this.selectOne(value)
	}
	selectFirst(){
		let item = this.list[0];
		if (item){
			let value = item.getAttribute(this.valueAttr);
			this.selectOne(value);
			return value;
		}
		return "";
	}
	selectOne(value){
		this._selected = [value];
		this.selectionChanged();
	}
	select(values){
		this._selected = values;
		this.selectionChanged();
	}
	toggle(value){
		let index = this._selected.indexOf(value);
		if(index<0)
			this._selected.push(value);
		else
			this._selected.splice(index, 1);
		this.selectionChanged();
	}
	selectionChanged(){
		this.updateList()
		let selected = this._selected.slice(0);
		let selected_str;
		if(!this.multiple){
			selected = selected[0];
			selected_str = selected;
		}else{
			selected_str = JSON.stringify(selected)
		}
		this.selected = selected_str;
		this.fire("select", {selected}, {bubbles:true})
	}
	get value(){
		if(!this.multiple)
			return this._selected[0]
		return this._selected
	}
}

FlowMenu.define('flow-menu');

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_SELECT_413 : &'static str = r###"



/**
 * @export
 * @class FlowSelect
 * @extends {FlowMenu}

* @cssvar {font-family} [--flow-font-family="Julius Sans One"]
* @cssvar {font-size} [--flow-select-font-size-label=0.7rem]
* @cssvar {font-weight} [--flow-input-font-weight=400]
* @cssvar {font-size} [--flow-input-font-size=1rem]
* @cssvar {height} [--flow-select-input-height]
* @cssvar {line-height} [--flow-input-line-heigh=1.2]
* @cssvar {min-height} [--flow-select-selected-min-height=44px]
* @cssvar {min-width} [--flow-select-selected-min-width=100px]
* @cssvar {max-width} [--flow-select-selected-max-width=500px]
* @cssvar {background-color} [--flow-input-bg=inherit]
* @cssvar {border} [--flow-select-border-label=2px solid  var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1)))]
* @cssvar {border} [--flow-select-border=2px solid var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1)))]
* @cssvar {color} [--flow-input-color=inherit]
* @cssvar {color} [--flow-input-placeholder=#888]
* @cssvar {padding} [--flow-select-padding-label=2px 5px]
* @cssvar {padding} [--flow-select-padding=5px]
* @cssvar {margin} [--flow-select-filter-input-margin=0px 0px 5px]
* @cssvar {color} [--flow-select-trigger-bg=transparent]
* @cssvar {color} [--flow-select-trigger-color= var(--flow-color, #000)]
* @cssvar {padding} [--flow-select-trigger-padding=0px]
* @cssvar {color} [--flow-select-trigger-hover-bg=transparent]
* @cssvar {color} [--flow-select-trigger-hover-color=var(--flow-dropdown-trigger-color)]
* @cssvar {height} [--flow-select-trigger-line-height=1]
* @cssvar {margin} [--flow-select-input-margin=0px]

 * @example
 * <flow-select label="menu" selected="0">
 *		<flow-menu-item value="0">Menu Item 1</flow-menu-item>
 *	 	<flow-menu-item value="1">Menu Item 2</flow-menu-item>
 * </flow-select>
 */
export class FlowSelect extends FlowMenu {
	static get properties() {
		return {
			label:{type:String},
			textAttr:{type:String},
			showfilter:{type:Boolean},
			backdrop:{type:Boolean},
			modal:{type:Boolean},
			filterText:{type:String},
			searchIcon:{type:String},
			disabled:{type:Boolean, reflect:true},
			placeholder:{type:String},
			"right-align":{type:Boolean}
		}
	}

	static get styles() {
		return [super.styles, css`
			:host{
				display:var(--flow-select-display, inline-block);vertical-align:middle;
				font-family:var(--flow-font-family, "Julius Sans One");
				padding:var(--flow-select-padding, 0px);
				margin:var(--flow-select-margin, 5px);
				width:var(--flow-select-width);
				max-width:var(--flow-select-max-width, 100%);
				height:var(--flow-select-height);
				--flow-dropdown-border:var(--flow-select-dropdown-border, 1px solid var(--flow-primary-color, #000));
				--flow-dropdown-trigger-bg:var(--flow-select-trigger-bg, transparent);
				--flow-dropdown-trigger-color:var(--flow-select-trigger-color, var(--flow-color, #000));
				--flow-dropdown-trigger-padding:var(--flow-select-trigger-padding, 0px);
				--flow-dropdown-trigger-hover-bg:var(--flow-select-trigger-hover-bg, transparent);
				--flow-dropdown-trigger-hover-color:var(--flow-select-trigger-hover-color, var(--flow-dropdown-trigger-color));
				--flow-dropdown-trigger-line-height:var(--flow-select-trigger-line-height, 1);
				--flow-dropdown-top:var(--flow-select-dropdown-border-top, -8px);
				--flow-dropdown-trigger-font-size:0px;
			}
			flow-dropdown{margin:0px;}
			.wrapper{
				display:flex;
				align-items:stretch;
				min-width:50px;
				text-align:center;
				/*justify-content:center;*/
				margin-top:var(--flow-select-wrapper-margin-top,-0.5rem);
				
				
			}
			label{
				/*font-size:0.7rem;
				padding:2px 5px;*/
				font-size:var(--flow-select-label-font-size, 0.7rem);
				padding:var(--flow-select-label-padding,2px 5px);
				/*border:2px solid var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1)));*/
				border: var(--flow-select-label-border, 2px) solid  var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-radius:8px;
				margin-left:var(--flow-select-label-margin-left,10px);
				margin-right: var(--flow-input-label-margin-right,20px);
				z-index:1;
    			position:relative;background-color:var(--flow-input-bg, inherit);
    			font-weight:var(--flow-font-weight, bold);
			}
			.input{
				
				flex:1;box-sizing:border-box;
			    /*border:2px solid var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1)));*/
				border: var(--flow-select-border, 2px) solid var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-radius:var(--flow-select-input-border-radius, var(--flow-input-border-radius, 8px));
    			margin:var(--flow-select-input-margin, 0px);
    			padding:var(--flow-select-input-padding,16px 30px 10px 10px);
				background-color:var(--flow-select-input-bg, var(--flow-input-bg, inherit));
				color:var(--flow-input-color, inherit);
				font-size:var(--flow-input-font-size, 1rem);
				font-weight:var(--flow-input-font-weight, 400);
				line-height:var(--flow-input-line-height, 1.2);
				text-align:left;
				height:var(--flow-select-input-height);
				width:var(--flow-select-input-width);
			}
			:host(.no-border) .input.selected{
				border:0px;
			}
			[no-label] .input.selected{
				padding-top:var(--flow-select-no-label-input-padding-top, 10px);
			}
			.input:focus{outline:none}
			.input::-webkit-input-placeholder { color: var(--flow-input-placeholder, #888 ); }
			.input.selected{
				min-width:var(--flow-select-selected-min-width, 100px);
				max-width:var(--flow-select-selected-max-width, 500px);
				min-height:var(--flow-select-selected-min-height, 44px);
				position:relative;
				box-shadow:var(--flow-input-box-shadow);
				height:var(--flow-select-selected-height);	
				width:var(--flow-select-selected-width);
				overflow: hidden;
    			text-overflow: ellipsis;
			}
			.placeholder{
				color: var(--flow-input-placeholder, #888 );
			}
			[multiple] .input.selected span.item{
				margin:var(--flow-select-selected-item-margin, 2px 4px 2px 0);
				padding:var(--flow-select-selected-item-padding, 1px 5px);
				border-radius:var(--flow-select-selected-item-border-radius, 5px);
				border:var(--flow-select-selected-item-border, 1px solid var(--flow-primary-color, #DDD));
				line-height:var(--flow-input-line-height, 1.3);
			}
			[multiple] .input.selected{
				display:var(--flow-select-selection-display, flex);
				flex-wrap:var(--flow-select-selection-flex-wrap, wrap);
				min-height:var(--flow-select-selected-min-height, 60px);
				align-items:var(--flow-select-selection-align-items, center);
			}

			:host(:not([disabled])) .input.selected::after{
				content:"";display:inline-block;
				position:absolute;right:10px;
				top:var(--flow-select-dropdown-arrow, calc(50% - 2px));
				width:0px;height:0px;
				border:5px solid var(--flow-primary-color, #000);
				border-left-color:transparent;
				border-bottom-color:transparent;
				border-right-color:transparent;
			}
			flow-dropdown:not([multiple]) .input.selected{white-space:nowrap}
			.filter{
				padding-top:10px;border-radius:3px;
			}
			.filter-box{
				position:relative;display:flex;align-items:center;
				margin:var(--flow-select-filter-input-margin, 0px 0px 5px);
			}
			.filter-box[hidden]{display:none}
			.filter-box .clear-btn{
				position:absolute;right:10px;cursor:pointer;display:none;
				font-size:1.5rem;color:var(--flow-input-color, inherit);
			}
			.filter-box input[has-content]+.clear-btn{display:inline-block}
			.filter-box input{
				padding-left:30px;
			}
			.filter-box .icon{
				width:15px;height:15px;fill:var(--flow-primary-color);
				position:absolute;left:10px;
			}

			.dd ::slotted([flow-select-filtred]){display:none}
		`];
	}
	constructor(){
		super();
		this.textAttr = "data-text";
	}
	//focus(){
	//	console.log("focus", this, this.dropdown?.scrollIntoView())
	//	//this.wrapperEl?.focus();
	//	//super.focus();
	//}
	render() {
		let iconSrc = this.iconPath(this.searchIcon || "search");
		let isLabel = !!this.label;
		return html
		`<flow-dropdown ?multiple="${this.multiple}" ?backdrop="${this.backdrop}"
			?modal="${this.modal}"
			?disabled=${this.disabled}
			?no-label=${!isLabel}
			?right-align=${this["right-align"]}>
			<div slot="trigger">
				<label ?hidden=${!isLabel}>${this.label||""}</label>
				<div class="wrapper" @click=${this.onWrapperClick}>
					<slot name="prefix"></slot>
					<div class="input selected">
						${this.renderSelected()}&nbsp;
					</div>
					<slot name="sufix"></slot>
				</div>
				
			</div><div class="dd">
				<div class="filter-box" ?hidden=${!this.showfilter}>
					<svg class="icon">
						<use href="${iconSrc}"></use>
					</svg>
					<input class="input filter" type="text" 
						placeholder="${this.placeholder || 'Search...'}"
						?has-content=${this.filterText}
						.value="${this.filterText||''}"
						@keyup=${this.onSearch} />
					<a class="clear-btn" @click=${this.clearFilter}>&times;</a>
				</div>
				<div class="menu-list-container">
				<slot class="menu-list"></slot>
				${this.renderItems()}
				</div>
			</div>
		</flow-dropdown>`;
	}
	onWrapperClick(){
		//
	}

	renderItems(){
		return '';
	}

	renderSelected(){
		let map = new Map();
		this.list.forEach(item=>{
			let text = item.getAttribute(this.textAttr) || item.innerText;
			map.set(item.getAttribute(this.valueAttr), text)
		});

		let items = this._selected.map(s=>{
			if(!s)
				return '';
			return html`<span class="item" value="${s}">${map.get(s) || s}</span>`
		}).filter(a=>!!a);

		if(items.length)
			return items
		return html`<span class="placeholder">${this.placeholder||""}</span>`;
	}

	selectionChanged(){
		super.selectionChanged();
		if(!this.multiple && this.dropdown)
			this.dropdown.close();
	}

	firstUpdated(){
		super.firstUpdated();
		if(this.classList.contains("right-align"))
			this["right-align"] = true;
		this.wrapperEl = this.renderRoot.querySelector("flow-dropdown .wrapper");
		this.dropdown = this.renderRoot.querySelector("flow-dropdown");
		let slot = this.renderRoot.querySelector('slot.menu-list');
		this.listSlot = slot;
		slot.addEventListener('slotchange', (e)=>{
			this.updateList();
		});
		this.parseSelected();
		this.requestUpdate("_selected", [])
	}

	updateList(changes){
		super.updateList();
		if(!changes)
			this.requestUpdate("_selected", this._selected.slice(0))
	}

	clearFilter(){
		this.filterText = "";
		this.filterList("");
	}

	onSearch(e){
		let text = e.target.value;

		this.filterText = text;
		this.debounce('onSearch', ()=>{
			this.filterList(text);
		}, 200)
	}
	filterList(text){
		const rg = new RegExp(`${text}`, 'i');
		console.log("this.list", this.list);
		this.list.forEach(item=>{
			let text = item.getAttribute(this.textAttr) || item.innerText;
			let value = item.getAttribute(this.valueAttr);
			if(!text || rg.test(value) || rg.test(text)){
				item.removeAttribute('flow-select-filtred')
			}else{
				item.setAttribute('flow-select-filtred', true)

			}
		})
	}


	/*
	onWindowResize(){
		this.updateDropdownSize();
	}
	onParentScroll(){
		this.updateDropdownSize();
	}
	
	connectedCallback(){
    	super.connectedCallback();
    	
    }
	disconnectedCallback(){
    	super.disconnectedCallback();
    	
    }
    */
}

FlowSelect.define('flow-select');

"###;

const ASPECTRON_FLOW_UX_SRC_TABLE_MIXIN_387 : &'static str = r###"

export const TableMixin = baseClass=>{
	class Table extends baseClass{

		buildFetchRequest(options, name=""){
			return {}//api.getRecords(options);
		}

		async loadRecords(options={}, name=""){
			//this.log("loadRecords", this)
			let {req, params} = this.buildFetchRequest(options, name);
			if(!req)
				return
			let {result, error} = await req;
			if(error)
				return

			let {total, items} = result;
			let {skip, limit} = params;
			let prefix = name?name+'_':'';
			this[`${prefix}pagination`] = buildPagination(total, skip, limit);
			this[`${prefix}pagination`].type = name;
			this[`${prefix}state`] = {params};
			this[`${prefix}items`] = items;
			this.setLoading(false, name);
			return result;
		}
		onPaginationClick(e){
			const target = e.target;
			if(target.matches(".disabled, .active"))
				return

			let paginationEl = target.closest("[data-pagination]");
			let pageEl = target.closest('[data-skip]');
			if(!paginationEl || !pageEl)
				return
			let name = paginationEl.getAttribute('data-pagination');
			let skip = pageEl.getAttribute('data-skip');
			let prefix = name?name+'_':'';

			let {params={}} = this[`${prefix}state`];
			params.skip = skip;
			//console.log("params", params)
			this.setLoading(true, name);
			this.loadRecords(params, name);
		}

		setLoading(isLoading, listName=""){
			let el = listName?this.querySelector(`.${listName}-holder`):this;
			(el || this).classList.toggle("loading", !!isLoading)
		}
	}

	return Table;
}
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_LINK_419 : &'static str = r###"





/**
* @class FlowLink
* Wraps content in a clickable element, simulating behavior of the `a` element.
* Intended for opening a new browser tab regardless of whether the element is
* instantiated inside of a browser environment or inside of NWJS.
* @extends BaseElement
*
* @property {Boolean} [disabled] 
* @property {Boolean} [icon]
* @property {String} [href]
* @property {String} [target]
*
* @cssvar {font-family} [--flow-font-family="Julius Sans One"]
* @cssvar {font-weight} [--flow-font-weight=bold]
* @cssvar {font-size} [--flow-link-font-size=1rem]
* @cssvar {color} [--flow-link-color=#017b68]
* @cssvar {color} [--flow-link-hover-color=#017b68]
* @cssvar {fill} [--flow-primary-color=017b68]
*
* @example
*   <flow-link href="href" target="_blank">text</flow-link>
*
*/
export class FlowLink extends BaseElement {
	static get properties() {
		return {
			disabled:{type:Boolean, reflect: true},
			icon:{type:Boolean, reflect: true},
			href : { type : String },
			target : { type : String }
		}
	}

	static get styles(){
		return css`
			:host{
				display:inline-block;
				font-family:var(--flow-link-font-family, var(--flow-font-family, "Julius Sans One"));
				font-weight:var(--flow-link-font-weight, var(--flow-font-weight, bold));
				font-size:var(--flow-link-font-size, 1rem);
			}
			:host([disabled]){
				opacity:0.5;
				cursor:default;
				pointer-events:none;
			}
			:host(:not([disabled])){
				cursor:pointer;
			}

			.link-wrapper {
				color: var(--flow-link-color, #017b68);
				display: flex;
			}

			.link-wrapper:hover {
				color: var(--flow-link-hover-color, #017b68);
			}

			.icon-box {
				display: block;
				width: 16px;
				height: 16px;
				margin-bottom: -4px;
				opacity: 0.65;
			}

			.icon-box svg {
				fill: var(--flow-primary-color, #017b68);
				width: 100%;
				height: 100%;
			}

			.content {
				display: block;
			}
		`;
	}

	constructor(){
		super();
	}

	render() {
		let iconSrc = this.iconPath(`external-link-square-alt`);
		return html`
		<div class="link-wrapper" @click=${this.click}>
			<div class="content"><slot></slot></div>
			${ this.icon ? html`<div class="icon-box"><svg><use href="${iconSrc}"></use></svg></div>` : '' }
		</div>
		`;
	}

	click() {
		// console.log("opening href:",this.href,"target:",this.target);
		this.fire("flow-link-click", {el:this})
		if(!this.href)
			return
		if(typeof nw == 'undefined') {
			let a = document.createElement('a');
			a.href = this.href;
			if(this.target)
				a.target = this.target;
			a.click();
		} else {
			require('nw.gui').Shell.openExternal(this.href);	
		}
	}

}

FlowLink.define('flow-link');
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_SELECTOR_395 : &'static str = r###"




export class FlowSelector extends FlowSelect{

    
    static get properties(){
        return {
            mergeProps:{type:String},
            mergeAttributes:{type:String},
            mergeInnerHTML:{type:Boolean}
        }
    }

    static get styles(){
        return [FlowSelect.styles, css`
            :host{
                --flow-select-input-height:var(--flow-selector-input-height, auto);
                width:var(--flow-selector-width, unset);
            }
            flow-dropdown{
                width:var(--flow-selector-dropdown-width, auto);
            }
            .input.selected{
                min-height:var(--flow-selector-selected-min-height, 50px);
                min-width:var(--flow-selector-selected-min-width, 10px);
                width:var(--flow-selector-selected-width, 100%);
                font-size:0px;display:flex;align-items:center;
                box-sizing:border-box;
                padding:var(--flow-selector-selected-padding, 16px 30px 10px 10px);
                flex-wrap:var(--flow-selector-selected-flex-wrap, wrap);
            }
            .input.selected::after{
                top:calc(50% - 2px);
            }
            .input.selected .item{
                margin:var(--flow-selector-item-margin, 0px);
                font-size:var(--flow-selector-item-line-height, 1rem);
                line-height:var(--flow-selector-item-line-height, 1.1);
            }
            :host([multiple]) .selected .item{
                margin:var(--flow-selector-multiple-item-margin, 0px 5px 5px 0px);
            }
        `]
    }

    constructor(){
        super();
        this.mergeProps = "";
    }

    renderSelected(){
        let map = new Map();
        this.list.forEach(item=>{
            map.set(item.getAttribute(this.valueAttr), item)
        })
        return this._selected.map(value=>{
            let item = map.get(value)
            if(!item)
                return
            let clone = item.cloneNode(this.mergeInnerHTML||false);
            clone.removeAttribute('flow-select-filtred')
            clone.classList.remove("menu-item", "selected");
            clone.classList.add("item")
            return this.mergeNobeProperties(clone, item);
        }).filter(item=>!!item)
    }

    get selectedNodes(){
        let map = new Map();
        this.list.forEach(item=>{
            map.set(item.getAttribute(this.valueAttr), item)
        })
        return this._selected.map(value=>{
            return map.get(value)
        }).filter(item=>!!item)
    }

    mergeNobeProperties(clone, org){
        let props = [];
        if(this.mergeProps){
            props = this.mergeProps.split(",")
        }else if(org.constructor?.properties){
            props = Object.keys(org.constructor?.properties||{})
        }
        props.forEach(p=>{
            clone[p] = org[p];
        })
        if(this.mergeAttributes){
            let attributes = this.mergeAttributes.split(",");
            attributes.forEach(name=>{
                if(!name)
                    return
                clone.setAttribute(name, org.getAttribute(name));
            })
        }
        return clone;
    }
}

FlowSelector.define('flow-selector');

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_FORMAT_378 : &'static str = r###"


export class FlowFormat {
	static 'duration'(v) {
		let hrs = Math.floor(v/1000/60/60);
		let min = Math.floor(v/1000/60%60);
		let sec = Math.floor(v/1000%60);
		if(!hrs && !min && !sec)
			return this.commas(v);
		let t = '';
		if(hrs) t += (hrs < 10 ? '0'+hrs : hrs) + ' h ';
		if(hrs || min) t += (min < 10 ? '0'+min : min) + ' m ';
		if(hrs || min || sec) t += (sec < 10 ? '0'+sec : sec) + ' s ';
		return t;
	}
	static 'commas'(v, precision = 0) {
		var parts = parseFloat(v).toFixed(parseInt(precision)).toString().split('.');
	    parts[0] = parts[0].replace(/\B(?=(\d{3})+(?!\d))/g, ",");
	    return parts.join('.');
	}
		
	static 'cs'(v, ctx) {
		const { precision } = ctx;
		return FlowFormat.commas(v, precision || 0);
	}
	static 'fiat'(v) { return this.commas(v,2); }
	static 'crypto'(v, opt) { 
		let result = this.commas(v,8); 
		if(opt?.noTrailingZeros) {
			let [int,frac] = result.split('.');
			frac = frac.replace(/0+$/,'');
			return frac ? `${int}.${frac}` : int;
		}
		return result;

	}
	static 'int'(v) { return this.commas(parseInt(v)); }
	static 'file-size-si'(v) { return parseFloat(v).toFileSize(true); }
	static 'file-size'(v) { return parseFloat(v).toFileSize(); }
	static 'hash-rate'(v, ctx) { return parseFloat(v).toHashMetric(ctx.precision, ctx.unit, ctx.commas) + "H/s"; }
	static 'default'(v, ctx) {
		return FlowFormat.cs(v, ctx);
		// const { precision } = ctx;
		// if(precision)
		// 	return parseFloat(v).toFixed(parseInt(precision));
		// return parseInt(v);
	}
}

if(!Number.prototype.toFileSize)
	Object.defineProperty(Number.prototype, 'toFileSize', {
		value: function(a, asNumber){
			var b,c,d;
			var r = (
				a=a?[1e3,'k','B']:[1024,'K','iB'],
				b=Math,
				c=b.log,
				d=c(this)/c(a[0])|0,this/b.pow(a[0],d)
			).toFixed(2)

			if(!asNumber){
				r += ' '+(d?(a[1]+'MGTPEZY')[--d]+a[2]:'Bytes');
			}
			return r;
		},
		writable:false,
		enumerable:false
	});


if(!Number.prototype.toUnitSize)
	Object.defineProperty(Number.prototype, 'toUnitSize', {
		value: function(asNumber){
			var a,b,c,d;
			var r = (
				a=1e3,
				b=Math,
				c=b.log,
				d=c(this)/c(a)|0,this/b.pow(a,d)
			).toFixed(2)

			if(!asNumber){
				r += ' '+(d?('KMGTPEZY')[--d]:' ');
			}
			return r;
		},
		writable:false,
		enumerable:false
	});


if(!Number.prototype.toHashMetric)
	Object.defineProperty(Number.prototype, 'toHashMetric', {
		value: function(precision, unit, commas) {

			var l = [
				[1e24, 'Y'],
				[1e21, 'Z'],
				[1e18, 'E'],
				[1e15, 'P'],
				[1e12, 'T'],
				[1e9, 'G'],
				[1e6, 'M'],
				[1e3, 'k']
			];

			var i = 0;
			if(unit) {
				while(i < l.length-1 && unit != l[i][1])
					i++;

			}
			else {
				while(i < l.length-1 && (this) < l[i][0])
					i++;
				unit = l[i][1];
			}

			var v = this / l[i][0];
			
			precision = _.isUndefined(precision) ? 2 : parseInt(precision);
			if(commas) {
				var parts = v.toFixed(precision).toString().split('.');
				parts[0] = parts[0].replace(/\B(?=(\d{3})+(?!\d))/g, ",");
				return parts.join('.') + ' ' + unit;
			}
			else {
				return v.toFixed(precision) + ' ' + unit;
			}
		},
		writable:false,
		enumerable:false
	});	
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_CLOCK_WIDGET_353 : &'static str = r###"

let Defaults = [
	'America/Los_Angeles:San_Francisco',
	'America/New_York',
	'Europe/London',
	'Europe/Moscow',
	'Asia/Dubai',
	'Asia/Hong_Kong',
	'Asia/Tokyo',
//	'Asia/Tel_Aviv'
]

export class FlowClockWidget extends BaseElement {

	static get properties() {
		return {
			locale: {type: String},
			tz: {type: String},
			city:{type:String}
		}
	}

	static get styles() {
		return css`
			:host {
				padding: 0px;
			}
			[col] { display: flex; flex-direction: column; }
			[row] { display: flex; flex-direction: row; }
			.caption { 
				font-size: var(--flow-clock-widget-caption-font-size, 9px);
				text-transform:var(--flow-clock-widget-caption-text-transform, uppercase);
				white-space: nowrap;
			}
			.time-wrapper {
				align-items: flex-start;
				padding: 1px;
			}
			.time { 
				font-size: var(--flow-clock-widget-time-font-size, 14px);
				font-family: var(--flow-clock-widget-time-font-family, "Consolas");	
			}
			.suffix {
				font-size: var(--flow-clock-widget-suffix-font-size, 10px);				
			}
		`;
	}

	constructor() {
		super();

		this.locale = 'en-US';
		this.tz = '';
		this.city = '';
	}

	connectedCallback() {
		super.connectedCallback();
		this.interval = setInterval(()=>{this.requestUpdate()}, 1000);
	}

	disconnectedCallback() {
		super.disconnectedCallback();
		if(this.interval) {
			clearInterval(this.interval);
			delete this.interval;
		}
	}

	render() {

		let city, timeZone;
		if(this.tz.includes(':')) {

			let parts = this.tz.split(':');
			//console.log("TIMEZONE",this.tz,parts);
			timeZone = parts.shift();
			city = parts.shift();
		} else {
			city = this.city || this.tz.split('/').pop() || 'N/A';
			timeZone = this.tz || 'UTC';
		}

		const tzTime = new Date().toLocaleString(this.locale,{timeZone});
		const s = (new Date(tzTime)).toISOString();
		const [time,ampm] = (new Date(tzTime)).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'}).split(/\s/);
		
		return html`
			<div col>
				<div class='caption'>${city.replace(/_/g,' ')}</div>
				<div row class='time-wrapper'>
					<div class='time'>${time}</div>
					${ampm?html`<div class="suffix"> ${ampm}</div>`:''}
				</div>
			</div>
		
		`;
	}  
}

FlowClockWidget.Defaults = Defaults;

/*
class FlowClockWidgetChild extends FlowClockWidget{}
class FlowClockWidgetChild2 extends FlowClockWidget{}
FlowClockWidgetChild2.Defaults = [];
console.log("FlowClockWidgetChild.Defaults", FlowClockWidgetChild.Defaults, FlowClockWidgetChild2.Defaults)
*/

FlowClockWidget.define('flow-clock-widget');

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_THEME_SELECT_404 : &'static str = r###"




/**
 * @export
 * @class FlowThemeSelect
 * @extends {FlowSelect}
 *
 *
 * 
 * @example
 * <flow-theme-select items="light,dark"></flow-dropdown>
 *
 */
export class FlowThemeSelect extends FlowSelect {
	static get properties() {
		return {
			items:{type:String}
		}
	}

	static get styles() {
		return [FlowSelect.styles, css`
			.selected{
				min-width: var(--flow-theme-select-selected-min-width, 150px);
			}
		`];
	}

	constructor(){
		super();
		this.items = "dark,light";
		this.label = "Theme"
		this.selected = this.getTheme("dark");
		/*this.hidefilter = true;*/
	}

	renderItems(){
		return this.items.split(",").map(item=>{
			let name = this.buildItemName(item);
			return html`<div class="menu-item" value="${item}">${name}</div>`
		})
	}
	get list(){
		if(!this.renderRoot){
			return [];
		}
		return [...this.renderRoot.querySelectorAll(".menu-item")]
	}
	buildItemName(name){
		name = name.toLowerCase()
					.replace(/[_-]+/g, ' ')
					.replace(/\s{2,}/g, ' ').trim();
		return name.charAt(0).toUpperCase() + name.slice(1);
	}
	selectionChanged(){
		super.selectionChanged();
		let value = this.value;
		if(value)
			this.setTheme(value);
	}

	onThemeChange(){
		//this.log("onThemeChange", getTheme())
		this.selected = this.getTheme();
		this.requestUpdate("selected")
	}

	connectedCallback(){
		super.connectedCallback();
		this._onThemeChange = this._onThemeChange || this.onThemeChange.bind(this);
		document.body.addEventListener("flow-theme-changed", this._onThemeChange)
	}
	disconnectedCallback(){
		super.disconnectedCallback();
		document.body.removeEventListener("flow-theme-changed", this._onThemeChange)
	}

	getTheme(defaultTheme){
		return getTheme(defaultTheme)
	}

	setTheme(theme){
		setTheme(theme);
	}
}

FlowThemeSelect.define('flow-theme-select');

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_DROPZONE_FIELD_369 : &'static str = r###"

import '../resources/extern/dz/min/dropzone.min.js';


/**
* @class FlowDropzoneField
* @extends BaseElement

* @property {Boolean} [disabled]
* @property {String} [btnText]
* @property {String} [value]
* @property {String} [label]
* @property {String} [type]
* @property {String} [pattern]
* @property {Function} [validator]
* @property {String} [placeholder]
* @property {Boolean} [readonly]
* @property {Object} [postData]
* @property {String} [uploadUrl]
*
*
* @cssvar {font-family} [--flow-font-family="Julius Sans One"]
* @cssvar {font-weight} [--flow-font-weight=bold]
* @cssvar {font-weight} [--flow-input-font-weight=400]
* @cssvar {font-size} [--flow-input-font-size-label=0.7rem]
* @cssvar {font-size} [--flow-input-font-size=1rem]
* @cssvar {width} [--flow-input-width=100%]
* @cssvar {min-width} [--flow-input-min-width=100px]
* @cssvar {max-width} [--flow-input-max-width=500px]
* @cssvar {height} [--flow-input-height]
* @cssvar {line-height} [--flow-input-line-height=1.2]
* @cssvar {background-color} [--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1))]
* @cssvar {background-color} [--flow-border-hover-color, var(--flow-primary-color, rgba(0,151,115,1))]
* @cssvar {background-color} [--flow-input-bg=inherit]
* @cssvar {background-color} [--flow-input-bg=inherit]
* @cssvar {border} [--flow-input-label-border=2px solid  var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1)))]
* @cssvar {border} [--flow-input-border=2px solid var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1))]
* @cssvar {border-top-left-radius} [--flow-input-btn-tlbr=0px]
* @cssvar {border-bottom-left-radius} [--flow-input-btn-blbr=0px]
* @cssvar {border-color} [--flow-border-hover-color, var(--flow-primary-color, rgba(0,151,115,1)]
* @cssvar {color} [--flow-border-invert-color, var(--flow-primary-invert-color, #FFF)]
* @cssvar {color} [--flow-input-color=inherit]
* @cssvar {color} [--flow-input-placeholder=#888]
* @cssvar {color} [--flow-input-invalid-color=red]
* @cssvar {color} [--flow-dz-field-remove-icon-color=var(--flow-color,#000)]
* @cssvar {padding} [--flow-input-label-padding=2px 5px]
* @cssvar {margin} [--flow-input-margin=5px 0px]
* @cssvar {margin-top} [--flow-input-wrapper-margin-top=-0.5rem]
* @cssvar {margin-left} [--flow-input-label-margin-left=10px]
* @cssvar {z-index} [--flow-input-label-z-index=1]
* @cssvar {position} [--flow-input-label-position=relative]
* @cssvar {background} [--flow-dz-field-remove-icon-bg=#FFF]
* @cssvar {box-shadow} [--flow-dz-field-remove-icon-box-shadow=0px 0px 4px #ccc]
* @example
*   <flow-dz-field></flow-dz-field>
*
*/
export class FlowDropzoneField extends BaseElement {
	static get properties() {
		return {
			btnText:{type: String},
            value:{type:String},
            type:{type:String},
			disabled:{type:Boolean},
			pattern:{type:String},
			validator:{type:Function},
			placeholder:{type:String},
			label:{type:String},
			readonly:{type:Boolean},
			postData:{type:Object},
			uploadUrl:{type:String}
		}
	}

	static get styles(){
		return css`
			:host{
				display:inline-block;vertical-align:middle;
				font-family:var(--flow-font-family, "Julius Sans One");
				font-weight:var(--flow-font-weight, bold);
				width:var(--flow-input-width, 100%);
				min-width:var(--flow-input-min-width, 100px);
				max-width:var(--flow-input-max-width, 500px);
				margin:var(--flow-input-margin, 5px 0px);
				font-size:0px;
			}
			:host(:not([disabled])) .btn,
			:host(:not([disabled])) .input{
				cursor:pointer;
			}
			
			:host(:not([apply-btn])) .btn{
				display: none;
			}
			
			.wrapper{
				display:flex;
				align-items:stretch;
				min-width_:50px;
				text-align:center;
				justify-content:center;
				margin-top:var(--flow-input-wrapper-margin-top,-0.5rem);
				height:var(--flow-input-wrapper-height);
			}
			label{
				font-size:var(--flow-input-label-font-size, 0.7rem);
				padding:var(--flow-input-label-padding,2px 5px);
				border: var(--flow-input-label-border, 2px) solid  var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-radius:var(--flow-input-label-border-radius, 8px);
				margin-left: var(--flow-input-label-margin-left,10px);
				z-index: var(--flow-input-label-z-index, 1);
				position: var(--flow-input-label-position, relative);
				background-color:var(--flow-input-bg, inherit);
			}
			.btn{
				position:relative;
				padding:5px;
				background-color:var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border: 2px solid var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1)));
				overflow:hidden;
				border-radius:8px;
				border-top-left-radius: var(--flow-input-btn-tlbr, 0px);
    			border-bottom-left-radius: var(--flow-input-btn-blbr, 0px);
    			color:var(--flow-border-invert-color, var(--flow-primary-invert-color, #FFF));
    			display: flex;
			    justify-content: center;
			    align-items: center;
			}
			:host(:not([disabled])) .btn:hover{
				background-color:var(--flow-border-hover-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-color:var(--flow-border-hover-color, var(--flow-primary-color, rgba(0,151,115,1)))
			}
			.input{
				width:100px;flex:1;box-sizing:border-box;
				min-height:var(--flow-input-height);
				border: var(--flow-input-border, 2px) solid var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-radius:var(--flow-input-border-radius, 8px);
    			margin:0px;
    			padding:16px 30px 10px 10px;
				background-color:var(--flow-input-bg, inherit);
				color:var(--flow-input-color, inherit);
				font-size:var(--flow-input-font-size, 1rem);
				font-weight:var(--flow-input-font-weight, 400);
				line-height:var(--flow-input-line-height, 1.2);
				box-shadow:var(--flow-input-box-shadow);
			}

			:host([apply-btn]) .input{
			    border-right-width:0px;
				border-top-right-radius: 0px;
				border-bottom-right-radius: 0px;
			}


			.input:focus{outline:none}
			.input::-webkit-input-placeholder { color: var(--flow-input-placeholder, #888 ); }
			:host([disabled]) .value{
				padding-right:10px;
			}
			.clear-btn{
				font-style: normal;
			    font-size: 25px;
			    padding: 0px 10px 0px 10px;
			    cursor: pointer;
			    display:none;
			    position: absolute;
			    right: 0px;
			    z-index: 1;
			}
			:host(:not([disabled])) [has-value] .clear-btn{display:block;}
			:host(.invalid) .input{color:var(--flow-input-invalid-color, red)}

			.dz-preview{
				position:relative;display:block;
			}
			.dz-preview .dz-progress{display:block;height:2px;}
			.dz-preview .dz-progress .dz-upload{
				display:block;height:100%;width:0;background:green
			}
			.dz-preview .dz-details{line-height:1.2;margin:2px;}
			.dz-preview .dz-error-message{color:red;display:none}
			.dz-preview.dz-error .dz-error-message,
			.dz-preview.dz-error .dz-error-mark{display:block}
			.dz-preview.dz-success .dz-success-mark{display:block}
			.dz-preview .dz-error-mark,
			.dz-preview .dz-success-mark{
				position:absolute;display:none;left:30px;top:30px;width:54px;height:58px;
				left:50%;margin-left:-27px
			}
			.dz-preview .dz-remove{
				position: absolute;
			    right:-15px;
			    top:-15px;
			    font-size:27px;
			    cursor: pointer;
			    color:var(--flow-dz-field-remove-icon-color, var(--flow-color,#000));
			    background:var(--flow-dz-field-remove-icon-bg, #FFF);
			    z-index:1;
			    box-shadow:var(--flow-dz-field-remove-icon-box-shadow, 0px 0px 4px #ccc);
			    padding: 5px;
			    border-radius:50%;
			    width: 20px;
			    height: 20px;
			    line-height:20px;
			}
			.dz-image-holder{
				position:relative;
				height:100px;
				background:var(--flow-dz-field-mage-holder-bg, rgba(0,0,0,0.1));
			}
			.dz-image-holder img{
				max-width:100%;
				max-height:100%;
			}
		`;
    }
    constructor() {
        super();
        this.type = 'text';
        this.value = '';
        this.postData = {};
    }
	render() {
		return html`<label ?hidden=${!this.label}>${this.label||""}</label>
		<div class="wrapper" @click=${this.onClick} ?has-value=${!!this.value}>
			<slot name="prefix"></slot>
			<div class="input"
				?disabled=${this.disabled} 
				@change=${this.onChange}>
			</div>
			<div class="btn">
				<div class="text"><flow-i18n text="${this.btnText || 'Select'}"></flow-i18n></div>
			</div>
			<slot name="sufix"></slot>
		</div>
		`;
	}

	firstUpdated(){
		super.firstUpdated();
		this.inputEl = this.renderRoot.querySelector(".input");
		let url = this.uploadUrl || "upload-file";
		this.dropzone = new Dropzone(this.inputEl, {
			url,
			acceptedFiles: this.acceptedFiles,
			withCredentials:true,
			paramName:this.paramName || "file",
			autoProcessQueue:this.autoProcessQueue || false,
			maxFiles:this.maxFiles || 1,
			uploadMultiple: false,
			maxFilesize : 500,
			//addRemoveLinks: true,
			previewTemplate:`<div class="dz-preview dz-file-preview">
			  <div class="dz-details">
			    <div class="dz-filename"><span data-dz-name></span></div>
			    <div class="dz-size" data-dz-size></div>
			    <div class="dz-image-holder">
			    	<img data-dz-thumbnail /> <div class="dz-remove" data-dz-remove>&times;</div>
			    </div>
			  </div>
			  <div class="dz-progress"><span class="dz-upload" data-dz-uploadprogress></span></div>
			  <!--div class="dz-success-mark"><span>✔</span></div>
			  <div class="dz-error-mark"><span>✘</span></div-->
			  <div class="dz-error-message"><span data-dz-errormessage></span></div>
			</div>`,
			maxfilesexceeded:(file)=>{
				this.dropzone.removeFile(file);
			}
		});
		this.dropzone.on("addedfile", (file)=>{
			if (this._oldFile) {
				this.dropzone.removeFile(this._oldFile);
			};
			this._oldFile = file;
			//this.set('hasOfflinePreview', true);
			this.classList.toggle('offline-preview', true);
			this.msg = 'Offline preview';
		});
		this.dropzone.on("reset", (file)=>{
			this.classList.toggle('offline-preview', false);
			//this.set('hasOfflinePreview', false);
			//this.srcChanged();
			this._oldFile = false;
		});

		this.dropzone.on("sending", (file, xhr, formData)=>{
			Object.keys(this.postData).forEach(k=>{
				formData.append(k, this.postData[k]);
			})
		});
		this.dropzone.on("success", (file, xhr)=>{
			this.cancelFiles();
			this.fire('upload-success')
			//this.msg = "Online preview";
		});
		this.dropzone.on("error", (file, xhr)=>{
			this.cancelFiles();
			this.fire('upload-error')
			//this.msg = "Online preview";
		});
		
	}

	uploadFile(){
		this.dropzone?.processQueue();
	}
	cancelFiles(){
		this.dropzone?.removeAllFiles(true);
	}

	setClear(){
		this.setValue("");
	}

	onClick() {
		this.fire("flow-dz-click", {el:this})
	}

	validate(value){
		let {pattern} = this;
		if(pattern){
			try{
				pattern = new RegExp(pattern)
			}catch(e){
				this.log("pattern error:", e)
				return false;
			}
			if(!pattern.test(value))
				return false;
		}
		if(typeof this.validator == 'function'){
			return this.validator(value, this);
		}
		return true;
	}

	onChange(e) {
		let value = this.shadowRoot.querySelector("input").value;
		if(!this.validate(value)){
			this.classList.add("invalid")
			return
		}
		this.classList.remove("invalid")
		//this.log("value", value)

		this.value = value;
		this.fire("changed", {el:this, value})
	}

	setValue(value){
		this.value = value;
		this.shadowRoot.querySelector("input").value = "";
		this.fire("changed", {el:this, value:this.value})
	}
}

FlowDropzoneField.define('flow-dz-field');
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_MARKDOWN_383 : &'static str = r###"



const escapeHtml = (unsafe) => {
    return unsafe
         .replace(/&/g, "&amp;")
         .replace(/</g, "&lt;")
         .replace(/>/g, "&gt;")
         .replace(/"/g, "&quot;")
         .replace(/'/g, "&#039;")
         .replace(/&amp;lt;/g, "&lt;")
         .replace(/&amp;gt;/g, "&gt;")
 }

export const markerdRenderer = {
    buildAnchorHref(text){
        return text.toLowerCase().replace(/[^\w]+/g, '-')
            .replace(/\-code\-/g, "").replace(/^[\-]+|[\-]+$/g, '')
    },
	heading(text, level) {
		const href = this.buildAnchorHref(text);

		return `
			<h${level} class="h-anchor">
			<a name="${href}" class="anchor" href="#${href}">
				<span class="anchor-icon" part="anchor-icon"></span>
			</a>
			${text}
			</h${level}>`;
    },
    
    code(text, info, escaped) {
        //console.log('code:',text);
        return `<flow-code lang="${info}"><textarea>${text.replace(/\t/g,'    ')}</textarea></flow-code>`;
    },

    html(text) {
        //console.log('html:',text);
        return escapeHtml(text);
    },

    codespan(text) {
        //console.log('codespan:',text);
        text = text.replace(/&amp;lt;/g, "&lt;").replace(/&amp;gt;/g, "&gt;");
        return `<code>${text}</code>`;
    },

    link(href, title, text) {
        return`<flow-link href="${href}" ${title?`title="${title}"`:''}>${text}</flow-link>`;
    }
}

/**
* @class FlowMarkdown
* @extends BaseElement
* @property {Boolean} [skipTrimming]
* @property {Boolean} [achorScroll]
* @property {Boolean} [sanitize]
* @property {Boolean} [toc]
* @property {Boolean} [full_height_toc]
* @cssvar {font-size} [--flow-markdown-anchor-icon-font-size=1rem]
* @cssvar {display} [--flow-markdown-anchor-icon-display=inline-block]
* @cssvar {width} [--flow-markdown-anchor-icon-width=15px]
* @cssvar {height} [--flow-markdown-anchor-icon-height=15px]
* @cssvar {margin} [--flow-markdown-anchor-icon-margin=0px 2px]
* @cssvar {opacity} [--flow-markdown-anchor-icon-opacity=0]
* @cssvar {opacity} [--flow-markdown-anchor-icon-opacity-hover=1]
* @cssvar {background-image} [--flow-markdown-icon]
* @cssvar {font-family} [--flow-markdown-code-font-family=monospace]
* @cssvar {font-size} [--flow-markdown-code-font-size=1rem]
* @cssvar {background-color} [--flow-markdown-code-background-color=#f3f3f3]
* @cssvar {padding} [--flow-markdown-code-padding=1px 3px]
* @cssvar {margin} [--flow-markdown-code-margin=1px 1px]
* @cssvar {border} [--flow-markdown-code-border=1px solid #ddd]
* @cssvar {min-width} [--flow-markdown-toc-width=200px]
* @cssvar {width} [--flow-markdown-toc-width=200px]
* @cssvar {top} [--flow-markdown-toc-top=0]
* @cssvar {padding} [--flow-markdown-toc-padding=10px]
* @cssvar {background-color} [--flow-markdown-toc-li-hover-bg=var(--flow-menu-item-hover-bg, #DDD)]
* @cssvar {color} [--flow-markdown-toc-li-hover-color=var(--flow-menu-item-hover-color, #000)]
* @cssvar {padding-left} [--flow-markdown-toc-level0-padding=4px]
* @cssvar {font-size} [--flow-markdown-toc-level0-font-size=0.96rem]
* @cssvar {font-weight} [--flow-markdown-toc-level0-font-weight=bold]
* @cssvar {padding-left} [--flow-markdown-toc-level1-padding=4px]
* @cssvar {font-size} [--flow-markdown-toc-level1-font-size=0.92rem]
* @cssvar {padding-left} [--flow-markdown-toc-level2-padding=18px]
* @cssvar {font-size} [--flow-markdown-toc-level2-font-size=0.86rem]
* @cssvar {padding-left} [--flow-markdown-toc-level3-padding=30px]
* @cssvar {font-size} [--flow-markdown-toc-level3-font-size=0.82rem]
* @cssvar {padding-left} [--flow-markdown-toc-level4-padding=45px]
* @cssvar {font-size} [--flow-markdown-toc-level4-font-size=0.76rem]
* @cssvar {padding-left} [--flow-markdown-toc-level5-padding=60px]
* @cssvar {font-size} [--flow-markdown-toc-level5-font-size=0.72rem]
* @cssvar {padding-left} [--flow-markdown-toc-level6-padding=75px]
* @cssvar {font-size} [--flow-markdown-toc-level6-font-size=0.66rem]
* @cssvar {padding-left} [--flow-markdown-toc-level7-padding=90px]
* @cssvar {font-size} [--flow-markdown-toc-level7-font-size=0.625rem]
* @example
*   <flow-markdown>fn()</flow-markdown>
*
*
*/

/*
... @ cssvar {--flow-code-font-family} [--flow-markdown-code-font-family]
... @ cssvar {--flow-code-font-size} [--flow-markdown-code-font-size]
... @ cssvar {--flow-code-border} [--flow-markdown-code-border=1px solid #ddd]
*/

export class FlowMarkdown extends BaseElement {
	static get properties() {
		return {
			skipTrimming:{type:Boolean},
            anchorScroll:{type:Boolean},
            sanitize : {type:Boolean},
            toc : {type:Boolean},
            'full-height-toc':{type:Boolean, reflect:true}
		}
	}

	static get styles() {
		return css`
			:host{display:block;}
			.md{display:none;}
			.anchor{font-size:0px;}
			.anchor-icon{
				font-size:var(--flow-markdown-anchor-icon-font-size, 1rem);
				display:var(--flow-markdown-anchor-icon-display, inline-block);
				width:var(--flow-markdown-anchor-icon-width, 15px);
				height:var(--flow-markdown-anchor-icon-height, 15px);
				margin:var(--flow-markdown-anchor-icon-margin, 0px 2px);
				opacity:var(--flow-markdown-anchor-icon-opacity, 0);
				border:0px solid #F00;cursor:pointer;
				background: center / contain;
				background-image:var(--flow-markdown-icon);
			}
			.h-anchor:hover>a.anchor .anchor-icon{
				opacity:var(--flow-markdown-anchor-icon-opacity-hover, 1);
            }
            
            #output > * {margin-left: 19px;}
            #output > h1, #output > h2, #output > h3, #output > h4, #output > h5 {
                margin-left: 0px;
            }

            td { vertical-align: top; }

            code, table tbody tr td code {
                display: inline-block;
                font-family: var(--flow-markdown-code-font-family, monospace);
                font-size: var(--flow-markdown-code-font-size, 1rem);
                background-color: var(--flow-markdown-code-background-color, #f3f3f3);
                padding: var(--flow-markdown-code-padding, 1px 3px);
                margin: var(--flow-markdown-code-margin, 1px 1px);
                border:var(--flow-markdown-code-border, 1px solid #ddd);
            }

            flow-code {
                --flow-code-white-space: pre;
                --flow-code-font-family: var(--flow-markdown-code-font-family);
                --flow-code-font-size: var(--flow-markdown-code-font-size);
                --flow-code-border: var(--flow-markdown-code-border,1px solid #ddd);
                /*color: var(--flow-markdown-code-color, --flow-code-color, --flow-background-inverse);*/
                /*background-color: var(--flow-markdown-code-background-color, #f3f3f3);
                color: var(--flow-markdown-code-color, #000);*/
                padding: 16px;
            }

            a, a:visited { 
                text-decoration: none; 
                color: var(--flow-link-color, #202169);
            }

            a:hover { 
                text-decoration: underline; 
                color: var(--flow-link-hover-color, #161649);
            }
            #wrapper {
                position:relative;
                display: flex;
                flex-direction:row;
            }
            :host([full-height-toc]) #wrapper{
                height:100%;
            }
            :host([full-height-toc]) .toc-outer,
            :host([full-height-toc]) #output{
                height:100%;
                overflow-y:auto;
                overflow-x:hidden;
            }
            :host([full-height-toc]) .toc-outer{
                min-width:var(--flow-markdown-toc-width, 200px);
            }
            #toc {
                border:0px solid red;
                width:var(--flow-markdown-toc-width, 200px);
                position: -webkit-sticky;
                position: sticky;
                top:var(--flow-markdown-toc-top, 0);
                list-style:none;
                padding:var(--flow-markdown-toc-padding, 10px);
                margin:0px;
            }
            :host([full-height-toc]) #toc{
                position:relative;
            }

            #toc li{
                cursor:pointer;
                padding: 1px;
            }
            #toc li:hover{
                background-color:var(--flow-markdown-toc-li-hover-bg, var(--flow-menu-item-hover-bg, #DDD));
                color:var(--flow-markdown-toc-li-hover-color, var(--flow-menu-item-hover-color, #000));
            }
            #toc [level="0"]{
                padding-left:var(--flow-markdown-toc-level0-padding, 4px);
                font-size:var(--flow-markdown-toc-level0-font-size, 0.96rem);
                font-weight:var(--flow-markdown-toc-level0-font-weight, bold);
            }
            #toc [level="1"]{
                padding-left:var(--flow-markdown-toc-level1-padding, 4px);
                font-size:var(--flow-markdown-toc-level1-font-size, 0.92rem);
            }
            #toc [level="2"]{
                padding-left:var(--flow-markdown-toc-level2-padding, 18px);
                font-size:var(--flow-markdown-toc-level2-font-size, 0.86rem);
            }
            #toc [level="3"]{
                padding-left:var(--flow-markdown-toc-level3-padding, 30px);
                font-size:var(--flow-markdown-toc-level3-font-size, 0.82rem);
            }
            #toc [level="4"]{
                padding-left:var(--flow-markdown-toc-level4-padding, 45px);
                font-size:var(--flow-markdown-toc-level4-font-size, 0.76rem);
            }
            #toc [level="5"]{
                padding-left:var(--flow-markdown-toc-level5-padding, 60px);
                font-size:var(--flow-markdown-toc-level5-font-size, 0.72rem);
            }
            #toc [level="6"]{
                padding-left:var(--flow-markdown-toc-level6-padding, 75px);
                font-size:var(--flow-markdown-toc-level6-font-size, 0.66rem);
            }
            #toc [level="7"]{
                padding-left:var(--flow-markdown-toc-level7-padding, 90px);
                font-size:var(--flow-markdown-toc-level7-font-size, 0.625rem);
            }
		`;
	}
	render() {
        let i = 1;
        let {level:firstLevel=0} = (this.toc_||[])[0]||{};
        let length = 10, num;
        (this.toc_||[]).forEach(o=>{
            if(length<o.level)
                length = o.level;
        })
        length = (length+"").length;

		return html`<div id="wrapper">
        ${
            this.toc ? 
            html`<div class="toc-outer"><ul id='toc' @click="${this.onTOCClick}">${
                this.toc_.map(t=> {
                    /*if(firstLevel == t.level){
                        num = (i++)+").";
                    }else{
                        num = "";
                    }*/
                    /*<span>${num.padStart(length, " ")}</span>*/
                    return html`<li level="${t.level}" 
                        data-scroll-to="${t.href}">${t.caption}</li>`;
                })
            }</ul></div>`:''
        }
        <div class="md"><slot></slot></div>
        <div id="output" @click="${this.onOutputClick}"></div>
        </div>`;
	}

	constructor() {
        super();
        this.sanitize = false;
        this.toc = false;
        this.toc_ = [];
    }
    
    firstUpdated() {
    	// TODO https://github.com/markedjs/marked
        const slot = this.renderRoot.querySelector('slot');
        this.outputEl = this.renderRoot.querySelector('#output');
       	this.slotEl = slot;
       	this.updateHtml();
		slot.addEventListener('slotchange', e=>{
			this.updateHtml();
		});
    }

    updateHtml(text=""){
    	if(!text.length){
	    	let nodes = this.slotEl.assignedNodes();
	    	let texts = [];
	    	nodes.forEach(el=>{
	    		if(el.nodeType==3){
	    			texts.push(el.textContent)
	    			return;
	    		}
	    		texts.push(el.innerHTML);
	    		//texts.push(el.innerText);
	    	})
            text = texts.join("\n\n");
            /*
	    	let line2 = text.trim().split("\n")[1];
	    	this.log("line2line2", line2)
	    	if(!this.skipTrimming && line2){
	    		let num = 0;
	    		let i = 0;
	    		let c = line2[i];
	    		let regExp = null;
	    		this.log("cccc:"+c, c=="\t")
	    		if(c == "\t"){
	    			while(c == "\t"){
	    				num++;
	    				c = line2[i++];
	    			}
	    			regExp = `^[\t]{1,${num}}`;
    			}else if(c == " "){
    				while(c == " "){
	    				num++;
	    				c = line2[i++];
	    			}
	    			regExp = `^[ ]{1,${num}}`;
    			}

    			if(regExp){
    				//this.log("regExp:"+regExp)
	    			regExp = new RegExp(regExp, "g");
	    			
	    			text = text.split("\n").map(line=>{
	    				//this.log("first:"+line[0]+"::::", line)
	    				return line.replace(regExp, "")
	    			}).join("\n");

	    			//this.log("tabs", regExp, tabs, text)
    			}
	    	}
	    	*/
        }

        text = text.replace(/<!---->/g, '');

        const tokens = window.marked.lexer(text);
        this.log("tokens", tokens);

        if(this.toc) {
            /*
            text.split('\n').forEach((line) => {
                if(/^#+/.test(line)) {
                    let level = -1;
                    while(line.charAt(0) == '#') {
                        level++;
                        line = line.substring(1);
                    }
                    let caption = line.trim().replace("")
                    this.toc_.push({
                        caption, level, 
                        href: markerdRenderer.buildAnchorHref(caption)
                    });
                }
            })
            */
        
            let tocList = tokens.filter(t=>t.type=="heading").map(t=>{
                let caption = t.tokens[0].text;
                return {
                    caption,
                    level:t.depth-1,
                    href: markerdRenderer.buildAnchorHref(caption)
                }
            });

            this.toc_ = tocList;

            this.log("tocList", tocList);

            this.requestUpdate();
        }

        const html = window.marked.parser(tokens);
        this.log("html", html);

    	//let html = window.marked(text);
        //this.log("this.toc_", this.toc, this.toc_, html)
    	//let html = window.marked(text);
    	this.outputEl.innerHTML =  this.sanitize ? DOMPurify.sanitize(html) : html;
    	if(this.anchorScroll){
    		dpc(100, ()=>{
    			this.scrollToLocationHash();
    		})
    	}

    }
    scrollToLocationHash(){
    	let hash = window.location.hash.replace("#", "");
    	this.scrollToElement(hash)
    }
    scrollToElement(id){
    	let ele = id.scrollIntoView?id:this.outputEl.querySelector(`a[name="${id}"]`);
    	if(ele){
            let eleP = ele.parentNode?.matches(".h-anchor")?ele.parentNode:ele;
            //this.log("eleP", eleP)
    		eleP.scrollIntoView(Object.assign(this.scrollIntoViewConfig || {}, {
    			behavior: "smooth",
    			block: "start",
    			inline: "nearest"
    		}));
        }

    }

    onTOCClick(e){
        let el = e.target.closest("li[data-scroll-to]");
        if(!el)
            return

        let id = el.getAttribute("data-scroll-to");
        this.scrollToElement(id);
    }

    onOutputClick(e){
    	let anchor = e.target.closest("a.anchor,a.scroll-to");
    	if(!anchor)
    		return
    	let href = (anchor.getAttribute("href")+"")
    	if(href.startsWith("#"))
    		anchor = href.replace("#", "")

    	this.scrollToElement(anchor);
    }
}


let defined = false;
let registerComponent =()=>{
	if(defined)
		return
	defined = true;

	marked.use({renderer: markerdRenderer})

	FlowMarkdown.define('flow-markdown');
}

let check = ()=>{
	loaded++;
	if(loaded == 2)
		registerComponent();
}

let loaded = 0;

[
	'extern/marked/marked.min.js',
	'extern/dom-purify/purify.min.js'
].forEach(file=>{
	if(file.indexOf("marked/") > -1 && window.marked)
		return check();

	let s = document.createElement("script");
	s.src = `${baseUrl}resources/${file}`;
	document.head.appendChild(s);
	s.onload = ()=>{
		check();
	}
});

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_TOOLBAR_426 : &'static str = r###"

import './fa-icon.js';


/**
* @class FlowToolbar
* @extends BaseElement
* @example
*   <flow-toolbar></flow-toolbar>
*
*/

export class FlowToolbar extends BaseElement {
	static get properties() {
		return {
			items:{type:Array, value:[]}
		}
	}

	static get styles() {
		return css`
			:host{
				padding:var(--flow-toolbar-padding, 0px 5px);
				align-items:center;
			}
			:host,
			.tools{
				display:flex;
			}
			.tools{
				padding:var(--flow-toolbar-tools-padding, 5px 0px);
				min-height:var(--flow-toolbar-tools-min-height, 76px);
				box-sizing:border-box;
			}
			/*.tool{
				position:relative;
				text-align:var(--flow-toolbar-item-text-align, center);
				padding:var(--flow-toolbar-item-padding, 5px);
			}
			.tool:before{
				position: absolute;
			    left: 0px;
			    top: -2px;
			    bottom: -2px;
			    right: 0px;
			    background:var(--flow-toolbar-item-shadow-bg, rgba(100,100,100, 0.2));
			    border-radius: 100px;
			    transform-origin: center center;
			    transform: scale(0,0);
			    transition: all 0.2s ease;
			    content:"";z-index:-1;
			}
			.tool:not(.disabled){
				cursor:pointer;
			}
			.tool:not(.disabled):hover:before{
			    border-radius: 3px;
			    transform: scale(1,1);
			}
			.icon{
				display:block;
				width:var(--flow-toolbar-item-icon-width, 28px);
			    height:var(--flow-toolbar-item-icon-height, 28px);
			    margin:var(--flow-toolbar-item-icon-margin, 0px auto);
			    --fa-icon-size:var(--flow-toolbar-item-icon-width, 28px);
			}
			.text{
				font-size:var(--flow-toolbar-item-text-font-size, 0.6rem);
			}
			.sub-text{
				font-size:var(--flow-toolbar-item-sub-text-font-size, 0.5rem);
			}
			*/
		`;
	}



	render() {
		let items = this.items
		return html`
		<div class="tools">
		<slot name="prefix"></slot>
		${
			items.map(o=>{
				return html`<flow-toolbar-item 
					data-code="${o.code||o.text}"
					class="${o.cls||''}"
					text="${o.text||''}"
					subText="${o.subText||''}"
					icon="${o.icon||''}"

					pressedText="${o.pressedText||''}"
					pressedSubText="${o.pressedSubText||''}"
					pressedIcon="${o.pressedIcon||''}"
					?togglable=${o.togglable||false}
					?pressed=${o.pressed||false}>
				</flow-toolbar-item>`
			})
		}
		<slot></slot>
		</div>`;
	}
	constructor(){
		super();
		this.initPropertiesDefaultValues();
	}

	firstUpdated(){
		this.renderRoot.addEventListener("click", this.onToolClick.bind(this));

		/////////////////////////////////////////////////////////////////
		this.renderRoot.addEventListener("flow-toolbar-item-state", e=>{
			this.fire("flow-toolbar-item-state", e.detail, {bubbles:true})
		})
		/// what the hell it is ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
		// bubbles event issue
		/////////////////////////////////////////////////////////////////

	}

	onToolClick(e){
		let tool = e.target.closest(".flow-toolbar-item, flow-toolbar-item");
		if(!tool)
			return
		this.fire("tool-click", {tool:tool.dataset.code, btn:tool});
	}
}

FlowToolbar.define('flow-toolbar');

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_GRIDSTACK_370 : &'static str = r###"


//


export class FlowGridStackTest extends BaseElement{
	render(){
		return html`
			<h1 slot="title">GridStack in SHADOW DOM</h1>
			<flow-gridstack class="gs"></flow-gridstack>`;
	}
}

FlowGridStackTest.define("flow-gridstack-test");


export const FlowGridStackMixin = (base)=>{
class FlowGridStackKlass extends base{
	static get properties() {
		return {
			gridMargin:{type:Number, value:1},
			column:{type:Number, value:30},
			disableResize:{type:Boolean},
			resizableHandles:{type:String, value:'e, s, w'},
			cellHeight:{type:Number, value:100},
			dragMode:{type:String, value:'header', reflect:true},
			items:{type:Array, value:[]},
			hidetools:{type:Boolean},
			dragInOptions:{type:Object},
			dragIn:{type:String},
			minWidth:{type:Number, value:400},
			removeTimeout:{type:Number, value:1000}
		}
	}

	static define(name, deps){
		if(deps){
			BaseElement.define.call(this, name, deps)
		}else{
			this.defineElement(name);
		}
	}

	static defineElement(name){
		this.addGridStackHelpers();
		BaseElement.defineElement.call(this, name);
	}

	static addGridStackHelpers(){

		$.ui.draggable.prototype._getHandle = function( event ) {
			let {handle, handleFn} = this.options;
			let gridEl = this.element.closest('.grid-stack')[0];
			if(gridEl && gridEl.gridstack){
				handleFn = gridEl.gridstack.opts.draggable.handleFn;
			}
			if(typeof handleFn == 'function')
				return handleFn(event, this);

			return handle?!!$(event.target)
				.closest(this.element.find(handle)).length:true;
		}
	}

	constructor() {
		var intersect = $.ui.intersect;
		/*
		let test = (droppable, draggable)=>{
			 var x1 = ( draggable.positionAbs ||
		        draggable.position.absolute ).left + draggable.margins.left,
		      y1 = ( draggable.positionAbs ||
		        draggable.position.absolute ).top + draggable.margins.top,
		      x2 = x1 + draggable.helperProportions.width,
		      y2 = y1 + draggable.helperProportions.height,
		      l = droppable.offset.left,
		      t = droppable.offset.top,
		      r = l + droppable.proportions().width,
		      b = t + droppable.proportions().height;

	       console.log("sssssss", droppable.eventNamespace, [ 
		       	l < x1 + ( draggable.helperProportions.width / 2 ) , // Right Half
		        x2 - ( draggable.helperProportions.width / 2 ) < r , // Left Half
		        t < y1 + ( draggable.helperProportions.height / 2 ) , // Bottom Half
		        y2 - ( draggable.helperProportions.height / 2 ) < b
	        ]); // Top Half
		}
		*/

		$.ui.ddmanager.dragStart = function( draggable, event ) {

		    // Listen for scrolling so that if the dragging causes scrolling the position of the
		    // droppables can be recalculated (see #5003)
		    draggable.element.parentsUntil( "body" ).on( "scroll.droppable", function() {
		      if ( !draggable.options.refreshPositions ) {
		        $.ui.ddmanager.prepareOffsets( draggable, event );
		      }
		    } );
		    const {gridstack} = draggable.element.parent()[0];
		    gridstack?._onResizeHandler();

		    
		    $.each( $.ui.ddmanager.droppables[ draggable.options.scope ] || [], function() {
		    	//if(this.eventNamespace==".droppable1"){
		    		this.isover = false;
		    		this.isout = true;
		    		this._out.call(this, event);
		    	//}
		    })
		}
		$.ui.ddmanager.drag = function( draggable, event ) {

			//console.log("CCCCCC",  $.ui.ddmanager.droppables[ draggable.options.scope ])

		    // If you have a highly dynamic page, you might try this option. It renders positions
		    // every time you move the mouse.
		    if ( draggable.options.refreshPositions ) {
		      $.ui.ddmanager.prepareOffsets( draggable, event );
		    }

		    // Run through all droppables and check their positions based on specific tolerance options
		    $.each( $.ui.ddmanager.droppables[ draggable.options.scope ] || [], function() {
	    		if ( this.options.disabled || this.greedyChild || !this.visible || !this.element.width() ) {
		        	return;
		       	}

		      var parentInstance, scope, parent;
		      var intersects = intersect( draggable, this, this.options.tolerance, event );
		      //if(!intersects && this.isover && this.options.tolerance=="intersect")
		      //	intersects = intersect( draggable, this, "fit", event );
		      //console.log("draggable.helperProportions", draggable.helperProportions)
		      //test(this, draggable);

		      var c = !intersects && this.isover ?
		          "isout" :
		          ( intersects && !this.isover ? "isover" : null );

		     //console.log("CCCCCC", this.element.width(), this.eventNamespace, this.element[0], c, intersects)
		      
		      if ( !c ) {
		        return;
		      }

		      if ( this.options.greedy ) {

		        // find droppable parents with same scope
		        scope = this.options.scope;
		        parent = this.element.parents( ":data(ui-droppable)" ).filter( function() {
		          return $( this ).droppable( "instance" ).options.scope === scope;
		        } );

		        if ( parent.length ) {
		          parentInstance = $( parent[ 0 ] ).droppable( "instance" );
		          parentInstance.greedyChild = ( c === "isover" );
		        }
		      }

		      // We just moved into a greedy child
		      if ( parentInstance && c === "isover" ) {
		        parentInstance.isover = false;
		        parentInstance.isout = true;
		        parentInstance._out.call( parentInstance, event );
		      }

		      this[ c ] = true;
		      this[ c === "isout" ? "isover" : "isout" ] = false;
		      this[ c === "isover" ? "_over" : "_out" ].call( this, event );

		      // We just moved out of a greedy child
		      if ( parentInstance && c === "isout" ) {
		        parentInstance.isout = false;
		        parentInstance.isover = true;
		        parentInstance._over.call( parentInstance, event );
		      }
		    });

		};
		super();
		this.initPropertiesDefaultValues();
		this.uid = 'flow-gs-'+(Math.random()*1000000).toFixed(0);
		this.style.display = 'block';
		//this.style.height = '1000px';
	}

	createRenderRoot(){
		return this;
	}

	render() {
		let {uid} = this;
		return html`
		<link rel="stylesheet" href="${baseUrl}resources/extern/gridstack/gridstack.min.css">
		<link rel="stylesheet" href="${baseUrl}resources/extern/gridstack/gridstack-extra.css">
		<style data-uid="${uid}"></style>
		${this.renderGSTools(uid)}
		<div class="grid-stack grid-stack-${this.column} ${uid} hide-w-opacity"
			@remove-gridstack-panel-request=${this.onRemovePanelRequest}
		></div>
		<slot></slot>`;
	}
	renderGSTools(uid){
		return html`
		<textarea class="gridstack-json" data-uid="${uid}" ?hidden=${this.hidetools}></textarea>
		<div class="buttons" ?hidden=${this.hidetools}>
			<flow-btn @click="${this.saveGrid}">Save</flow-btn>
			<flow-btn @click="${this.loadGrid}">Load</flow-btn>
			<flow-btn @click="${this.saveGridLS}">Save to LStorage</flow-btn>
			<flow-btn @click="${this.loadGridLS}">Load from LStorage</flow-btn>
			<flow-btn @click="${this.toggleDragMode}">ToggleDragMode : ${this.dragMode}</flow-btn>
		</div>`
	}
	firstUpdated(){
		let {uid} = this;
		this.gridEl = this.renderRoot.querySelector('.grid-stack');
		this.styleEl = this.renderRoot.querySelector(`style[data-uid="${uid}"]`);
		this.debugEl = this.renderRoot.querySelector(`textarea[data-uid="${uid}"]`);
		this.styleEl.textContent = `
			/*.${uid} .grid-stack-item-content{display:block}*/
			.${uid}.grid-stack .grid-stack-placeholder{
				background:var(--flow-gridstack-placeholder-bg, #1b202f);
			}
			.${uid}.grid-stack .grid-stack-placeholder .placeholder-content{
				border:var(--flow-gridstack-placeholder-content-border, 0px);
			}
			.${uid}.grid-stack.hide-w-opacity{opacity:0}
			.${uid} .grid-stack-item.ui-resizable-resizing:after{
				content:"";position:absolute;top:0px;left:0px;right:0px;bottom:0px;
				/*background:#F00;*/
				z-index:89;
			}
			${this.customCss(uid)}
		`
		//console.log("this.resizableHandles", this.resizableHandles)
		let options = {
			alwaysShowResizeHandle:false,// /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent),
			//ddPlugin:GridStackDDJQueryUI,
			resizable:{
			    handles: this.resizableHandles
			},
			minRow:1,
			margin:this.gridMargin,
			cellHeight:this.cellHeight,
			column:this.column,
			minWidth:this.minWidth,
			removeTimeout:this.removeTimeout,
			dragIn: this.dragIn ||'.sidebar .grid-stack-item',
			acceptWidgets:this.acceptWidgets||function(el) {/*console.log("acceptWidgets", this, el);*/return true; },
			dragInOptions:this.dragInOptions|| {
				revert: 'invalid',
				scroll: false,
				appendTo: this,
				helper: ()=>{
					//console.log("ssssshelper:")
					///let el = document.createElement("div");
					//el.style.backgroundColor = "#F0F";
					//return;// el;
				}
			}, // clone
			draggable:{
				handle:'.grid-stack-item-content .heading',
				//refreshPositions:true,
				helper___: ()=>{
					let el = document.createElement("div");
					el.style.backgroundColor = "#F0F";
					return el;
				},
				handleFn:(event, uiDraggable)=>{
					let {handle} = uiDraggable.options.handle;
					if(this.dragMode=="panel"){
						handle = '.grid-stack-item-content';
					}else if(this.dragMode=='header'){
						//console.log("event.target", event.originalEvent, handle, this.element.find( handle ))
						let cmp = uiDraggable.element.find('.grid-stack-item-content')[0];
						let handleEl;
						if(cmp && cmp.getGridstackDragHandle){
							handleEl = cmp.getGridstackDragHandle();
						}

						if(!handleEl)
							return false;
						return event.originalEvent.path?.includes(handleEl);
					}

					return handle?!!$(event.target)
						.closest(uiDraggable.element.find(handle)).length:true;
				}
			}
		};
		dpc(()=>{
			this.grid = GridStack.init(options, this.gridEl);
			this.gridEl.classList.remove("hide-w-opacity");
			this.grid.on('added removed change', (e, items)=>{
				let str = '';
				items.forEach(o=>{
					str += `${o.id} => x: ${o.x}, y: ${o.y}, w: ${o.width}, h: ${o.height}\n`;
				});
				this.log(`${e.type} ${items.length} items\n${str}` );
			});
			//if(this.acceptWidgets != false){
				//$(this.gridEl).droppable("option", "tolerance", "fit")
				//let dropOptions = $(this.gridEl).droppable("option")
				//dropOptions.tolerance = "fit";
				//console.log("optionsoptions", dropOptions)
			//}
			//console.log("GridStack.prototype.getElement", GridStack.prototype.getElement)
			this.initItems();
		}, 100)
	}
	customCss(uid){
		return '';
	}

	setLocalSetting(name, value){
		if(typeof value != 'string')
			value = JSON.stringify(value);
		setLocalSetting('gridstack-${this.id || this.uid)}-${name}', value);
	}

	getLocalSetting(name, defaults){
		let value = getLocalSetting('gridstack-${this.id || this.uid)}-${name}');
		if(typeof value == 'undefined')
			return defaults;

		return value;
	}

	saveGridLS(){
		this.setLocalSetting('grid', this.saveGrid())
	}
	loadGridLS(){
		let grid = this.getLocalSetting('grid', '[]');
		this.debugEl.value = grid;
		try{
			grid = JSON.parse(grid);
			if(grid)
				this.debugEl.value = JSON.stringify(grid, null, '  ');
		}catch(e){
			grid = [];
		}
		this.setGridItemsConfig(grid);
	}

	saveGrid(){
		let data = this.getGridItemsConfig();
		this.debugEl.value = JSON.stringify(data, null, '  ');
		return data;
	}
	loadGrid(){
		let data = [];
		try{
			data = JSON.parse(this.debugEl.value);
		}catch(e){
			//data;
			this.log("JSON.parse:error", e)
		}

		//console.log("loadGrid", this.debugEl.value, data)

		this.setGridItemsConfig(data);
	}

	updated(changes){
		if(changes.has('items'))
			this.setGridItemsConfig(this.items||[]);
	}
	initItems(){
		let {items} = this;
		if(items && items.length)
			this.setGridItemsConfig(items);
		this.fire("gridstack-ready", {grid:this})
	}
    getGridItemsConfig(){
    	let data = [];
		this.grid.engine.nodes.forEach(node=>{
			let serializedData = null, nodeName = 'div';
			let el = node.el.querySelector(".grid-stack-item-content");
			if(el){
				nodeName = el.nodeName;
				if(typeof el.serialize == 'function')
					serializedData = el.serialize();
			}

			data.push({
				x: node.x,
				y: node.y,
				width: node.width,
				height: node.height,
				id: node.id||el.parentNode?.dataset.gsId||'node-'+(Math.random()*10000).toFixed(),
				nodeName,
				serializedData
			});
		});

		return data;
    }
    setGridItemsConfig(config){
		this.lastConfig = config
		this.onResize();
	}
	activateGridItemsConfig(itemsConfig){
		let items = GridStack.Utils.sort(itemsConfig);
		let {grid} = this;
		if(!grid)
			return

		grid.batchUpdate();
		if (grid.engine.nodes.length === 0) {
			// load from empty
			items.forEach(item=>{
				this.addWidget(item)
			});
		} else {
			//console.log("items", items)
			// else update existing nodes (instead of calling grid.removeAll())
			let itemsIdMap = new Map();
			items.forEach(item=>{
				itemsIdMap.set(item.id, item);
				let node = grid.engine.nodes.find(n=>n.id == item.id);
				//console.log("node", node, item)
				if(node){
					//console.log("sending serializedData00", node.el, item.serializedData)
					grid.update(node.el, item.x, item.y, item.width, item.height);
					//console.log("sending serializedData11", node.el, item.serializedData)
					this.sendSerializeDataToPanel(node.el, item.serializedData);
				}else{
					this.addWidget(item)
				}
			});
			let nodes = [...grid.engine.nodes];
			nodes.forEach(node=>{
				if(!itemsIdMap.get(node.id))
					this.grid.removeWidget(node.el, true, true)
			})
		}
		grid.commit();
	}
	addWidget(item){
		let nodeName = item.nodeName || 'div';
		let el = this.grid.addWidget(`<div class="grid-stack-item" data-gs-id="${item.id}">
			<${nodeName} class="grid-stack-item-content"></${nodeName}></div>`, item);
		this.sendSerializeDataToPanel(el, item.serializedData);
	}
	sendSerializeDataToPanel(el, serializedData){
		if(!serializedData)
			return
		el = el.querySelector(".grid-stack-item-content")
		if(!el || typeof el.deserialize!='function')
			return console.log("el.deserialize is missing", el&&el.deserialize)

		//console.log("sending serializedData", el, serializedData)
		el.deserialize(serializedData);
	}
	clearGrid(){
		this.grid.removeAll();
    }
    onResize(){
    	//console.log("onResize")
    	let {grid} = this;
    	if(!grid)
    		return
    	dpc(10, e=>{
    		//console.log("onResize1")
    		//grid._updateContainerHeight();
    		//if(this.offsetHeight)
    			grid._onResizeHandler();
    			this.afterResize();
    		//grid.commit();
    		//grid.compact();
    	})
    }
	afterResize(e){
		//console.log("this.offsetWidth", this.offsetWidth, this.lastConfig)
		if(this.offsetWidth && this.lastConfig){
			let config = this.lastConfig;
			this.lastConfig = null;
			this.activateGridItemsConfig(config);
		}
	}
    removePanel(panel, removeDOM=true, fireEvent=true){
    	if(!panel.matches(".grid-stack-item"))
    		panel = panel.closest(".grid-stack-item");
    	if(!panel)
    		return
    	this.grid.removeWidget(panel, removeDOM, fireEvent)
    }
    toggleDragMode(){
    	if(this.dragMode == 'panel'){
			this.setDragMode('header');
		}else{
			this.setDragMode('panel');
		}
		return this.dragMode;
    }
    setDragMode(mode){
    	let {uid} = this;
    	//this.addCSSRule(`.${uid}.grid-stack`, 'background:#F00');
    	if(['header', 'panel'].includes(mode)){
    		this.dragMode = mode;
    		return this.dragMode;
    	}

    	return false
    }

    onRemovePanelRequest(e){
    	let {panel} = e.detail;
    	if(!panel)
    		return
    	//console.log("onRemovePanelRequest:", panel)
    	/*
    	setTimeout(()=>{
    		e.preventDefault();
    	}, 100)
    	*/

    	this.removePanel(panel);
    }

    serialize(){
    	let config = super.serialize()
		config.items = this.getGridItemsConfig();
		return config;
	}

	deserialize(config){
		super.deserialize(config)
		let {items} = config;
		this.setGridItemsConfig(items);
	}

    connectedCallback(){
		super.connectedCallback();
		if(!this.resizeObserver){
			this.resizeObserver = new ResizeObserver(()=>{
				this.onResize();
			});
		}

		this.resizeObserver.observe(this);
	}
	disconnectedCallback(){
		super.disconnectedCallback();
		this.resizeObserver.disconnect();
	}
}
return FlowGridStackKlass;
}

/**
* @class FlowGridStack
* @extends BaseElement
* @example
*   <flow-gridstack></flow-gridstack>
*
*/

export const FlowGridStackImp = FlowGridStackMixin(BaseElement);
export class FlowGridStack extends FlowGridStackImp{}

FlowGridStack.define('flow-gridstack',[baseUrl+'resources/extern/gridstack/gridstack.all.js']);

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_DOWNLOAD_BADGE_355 : &'static str = r###"


/**
* @class FlowDownloadBadge
* @extends BaseElement
* @property {String} [file]
* @property {String} [icon]
* @property {String} [title]
* @property {String} [descr]
* @property {String} [sha1]
* @example
*   <flow-download-badge icon="" title=""></flow-download-badge>
*
*
*/
export class FlowDownloadBadge extends BaseElement {
	static get properties() {
		return {
			file:{type:String},
			icon:{type:String},
			title:{type:String},
			descr:{type:String},
			sha1:{type:String}
		}
	}

	static get styles() {
		return css`
			:host{display:flex;flex-direction:row;align-items:center;}
			.title{min-width:var(--flow-download-badge-title-min-width, 230px);}
			.icon{
			    min-width:var(--flow-download-badge-icon-size, 24px);
			    min-height:var(--flow-download-badge-icon-size, 24px);
			    background-position:center;
			    background-repeat:no-repeat;background-size:contain;margin-bottom:-10px;
			    margin:var(--flow-download-badge-icon-margin, 0px 14px 0px 0px);
			}
			.file-link{
				display:flex;flex-direction:row;
				align-items:var(--flow-download-badge-file-link-align-items, center);
				padding:var(--flow-download-badge-file-link-padding, 6px);
				font-size:var(--flow-download-badge-file-link-font-size, 16px);
			}
			[disable]{pointer-event:none}
			a{color: var(--flow-link-color, #017b68);}
			a:not([disable]):hover{
				color: var(--flow-link-hover-color, #017b68);
			}
			[hide]{display:none}
			[row]{display:flex;flex-direction:row;}
			[col]{display:flex;flex-direction:column;}
		`;
	}

	render() {
		return html`
			<div class="file-link" href="${this.file}">
				<div class="icon" style="background-image:url(${this.icon})"></div>
				<div col>
					<div class="title">${this.title}</div>
					<div class="descr">${this.descr}</div>
				</div>
            	<slot></slot>
        	</div>
        	<div class="sha-link" href="${this.sha1}" ?hide="${!this.sha1}">
        		<div class="sha">SHA1</div>
        	</div>
    	</div>`;
	}
}

FlowDownloadBadge.define('flow-download-badge');

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_COLOR_PICKER_385 : &'static str = r###"


/**
* @class FlowColorPicker
* @extends BaseElement
* @property {String} [color]
* @example
*   <flow-color-picker color="#F00"></flow-color-picker>
*
*
*/
export class FlowColorPicker extends BaseElement {
	static get properties() {
		return {
			color:{type:String}
		}
	}

	static get styles() {
		return css`
			:host{
				display:inline-block;width:20px;height:20px;
				position:relative;box-sizing:border-box;
				border:var(--flow-color-picker-border, 1px solid var(--flow-border-color, var(--flow-primary-color, #FFF)));
			}
			.box{width:100%;height:100%}
			:host(:not([disabled])) input{
				cursor:pointer;
			}
			:host([disabled]) input{display:none}
			input.color{
				opacity:0;
				position:absolute;left:0px;top:0px;width:100%;height:100%;
				right:0px;buttom:0px;
			}
		`;
	}

	render() {
		return html`<div class="box" style="background-color:${this.color}"></div><input 
		class="color" type="color" .value="${this.color||""}"
		@change="${this.onInputChange}" 
		@input="${this.onInputChange}" />`;
	}

	onInputChange(e){
		this.color = e.target.value;
		this.fire("changed", {color:this.color}, {bubbles:true})
	}
}

FlowColorPicker.define('flow-color-picker');

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_RSS_415 : &'static str = r###"



/**
* @class FlowRSS
* @extends BaseElement

* @property {String} [href]
*
* @cssvar {font-family} [--flow-font-family="Julius Sans One"]

* @example
*   <flow-rss href=""></flow-app-drawer>
*
*/
export class FlowRSS extends BaseElement {
	static get properties() {
		return {
			href:{type:String},
			enablePictures:{type:Boolean}
		}
	}

	static get styles(){
		return css`
			:host{display:block;}
			img{max-width:100%;height:auto;}
			a{
				color: var(--flow-link-color, #017b68);
			}

			a:hover {
				color: var(--flow-link-hover-color, #017b68);
			}

			.article-link{
				margin: 5px 0px;
			}

			.article-content{
				font-family: var(--flow-font-family, 'Sans Serif');
				font-size: 14px;
			}
		`;
    }
	render() {
		return html`${this.href} ${this.body}`;
	}
	updated(changes){
		if((changes.has("href") || changes.has("enablePictures")) && this.href)
			this.fetch(this.href);
	}
	fetch(href){
		let opts = {method:"GET", mode:"cors", referrerPolicy: 'no-referrer'};
		return fetch(href, Object.assign(opts, this.feedOpt||{}))
	}
	
	setFeedData(data){
		//this.feedData = data;
		data = new window.DOMParser().parseFromString(data, "text/xml")
		this.buildBody(data);
		this.requestUpdate("body", null);
	}
	buildBody(xmlEl){
		let items = [...xmlEl.querySelectorAll("item")]
		this.body = html`
		${items.map(el=>{
			let link = el.querySelector("link");
			let dsc = el.querySelector("description")||"";
			if(dsc){
				if(dsc.childNodes[0]?.nodeName=="#cdata-section"){
					//console.log("dsc", dsc, dsc.childNodes[0].nodeValue)
					dsc = dsc.childNodes[0].nodeValue;
				}
				else
					dsc = dsc.innerHTML
			}
			return html`
			<article>
	          <img src="${link.innerHTML}/image/large.png" alt="">
	          <div class="article-link">
	            <flow-link href="${link.innerHTML}" target="_blank" rel="noopener">
	              ${el.querySelector("title").innerHTML}
	            </flow-link>
	          </div>
	          ${this.buildNode(dsc)}
	        </article>`
        })}
		`
	}
	buildNode(htmlContent){
		this._tpl = this._tpl || document.createElement("template");
		this._tpl.innerHTML = `<div class="article-content">${htmlContent}</div>`;
		let node = this._tpl.content.firstChild;
		//console.log("htmlContent", htmlContent, this._tpl.innerHTML, node)
		node.querySelectorAll("script"+(this.enablePictures?"":",img")).forEach(el=>{
			//console.log("fetchFeed:script,img:el", el)
			el.remove();
		})
		return node;
	}
}

FlowRSS.define('flow-rss');

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_SUNBURST_GRAPH_430 : &'static str = r###"





if(window.d3?.selection){
	d3.selection.prototype.selectAppend = function(name) {
	    let t = this.select(name);
	    return t.size()?t:this.append(name);
	}
}

let data = 
{"name":"flare","children":[
	{"name":"analytics", "children":[
		{"name":"cluster","children":[
			{"name":"AgglomerativeCluster","value":3938},
			{"name":"CommunityStructure","value":3812},
			{"name":"HierarchicalCluster","value":6714},
			{"name":"MergeEdge","value":743}
		]},
		{"name":"graph","children":[
			{"name":"BetweennessCentrality","value":3534},
			{"name":"LinkDistance","value":5731},
			{"name":"MaxFlowMinCut","value":7840}
		]},
		{"name":"optimization","children":[
			{"name":"AspectRatioBanker","value":7074}
		]}
	]},
	{"name":"animate","children":[
		{"name":"Easing","value":17010},
		{"name":"FunctionSequence","value":5842},
		{"name":"interpolate","children":[
			{"name":"ArrayInterpolator","value":1983},
			{"name":"ColorInterpolator","value":2047},
			{"name":"DateInterpolator","value":1375},
			{"name":"Interpolator","value":8746},
		]},
		{"name":"ISchedulable","value":1041},
		{"name":"Parallel","value":5176},
		{"name":"Pause","value":449},
	]},
	{"name":"data","children":[
		{"name":"converters","children":[
			{"name":"Converters","value":721},
			{"name":"DelimitedTextConverter","value":4294},
			{"name":"GraphMLConverter","value":9800}
		]},
		{"name":"DataField","value":1759},
		{"name":"DataSchema","value":2165},
		{"name":"DataSet","value":586}
	]},
	{"name":"display","children":[
		{"name":"DirtySprite","value":8833},
		{"name":"LineSprite","value":1732}
	]},
	{"name":"flex","children":[
		{"name":"FlareVis","value":4116}
	]},
	{"name":"physics","children":[
		{"name":"DragForce","value":1082},
		{"name":"GravityForce","value":1336},
		{"name":"IForce","value":1319}
	]},
	{"name":"query","children":[
		{"name":"AggregateExpression","value":1616},
		{"name":"And","value":3027}
	]}
]}

/**
* @class FlowSunburstGraph
* @extends Flowd3Element
* @prop {Boolean} noZoom
* @cssvar {color} [--flow-sunburst-graph-text-color="var(--flow-color, #000)"]
* @example
*   <flow-sunburst-graph></flow-sunburst-graph>
*
*/
export class FlowSunburstGraph extends Flowd3Element {
	static get properties() {
		return {
			noZoom:{type:Boolean},
			data:{type:Object},
			updatenum:{type:Number},
			d3margin:{type:Number}
		}
	}

	static get sampleData(){
		return data;
	}

	static get styles(){
		return [Flowd3Element.styles, ScrollbarStyle, css`
			:host{
				display:inline-flex;
				font-weight:bold;
				font-size:10px;
				text-transform:uppercase;
				font-family:var(--flow-data-field-font-family, "Julius Sans One");
				font-weight:var(--flow-data-field-font-weight, bold);
				border-radius: 10px;
				overflow: hidden;
				position:relative;
				width:100%;
				height:100%;
			}
			:host([disabled]){opacity:0.5;cursor:default;pointer-events:none;}
			.container{white-space:nowrap;padding:2px 6px 6px 6px;height:100%;}
			.container>div{padding:2px;}
			.suffix{opacity:0.9;margin-left:3px;margin-top:3px; font-size: 10px;}
			.col{display: flex; flex-direction: column; align-items: left;}
			.row{display: flex; flex-direction: row; flex:0;}

			.wrapper {
				position:relative;flex:1;
				margin:6px;overflow:hidden;
				display: flex;
			    align-items: stretch;
			    justify-content: center;
			}
			
			:host([border]) .wrapper {
				border: 2px solid var(--flow-primary-color,#333);
				box-shadow: 2px 2px 1px rgba(1, 123, 104, 0.1);
				border-radius: 10px;

			}

			.wrapper > div:not(.tip,.legends) {
				width:100%;height:100%;
				position:relative;left:0px;top:0px;bottom:0px;right:0px;
			}

			.d3-holder{
				min-height:10px;
				min-width:10px;
				opacity:1;
				display:flex;
				/*background-color:#F00;*/
				align-items:center;
			}
			[flex] {
				flex: 1;
			}
			/*#d3{background-color:#f0f}*/
			text{fill:var(--flow-sunburst-graph-text-color, var(--flow-color, #000))}
			path{cursor:default}
			.tip{
				position:absolute;border:1px solid var(--flow-primary-color,#333);
				box-sizing:border-box;display:none;
				width:var(--flow-sunburst-graph-tip-width, unset);
				max-width:var(--flow-sunburst-graph-tip-width, 95%);
				padding:var(--flow-sunburst-graph-tip-padding, 10px);
				min-width:var(--flow-sunburst-graph-tip-min-width, 100px);
				min-height:var(--flow-sunburst-graph-tip-min-height, unset);
				border-radius:var(--flow-sunburst-graph-tip-border-radius, 4px);
				background-color:var(--flow-sunburst-graph-tip-bg, var(--flow-background-color));
				color:var(--flow-sunburst-graph-tip-color, var(--flow-color));
			}
			.legends{
				margin:var(--flow-sunburst-graph-legends-margin, 0px 0px 0px 5px);
				width:var(--flow-sunburst-graph-legends-width, 30%);
				height:var(--flow-sunburst-graph-legends-height, initial);
				background-color:var(--flow-sunburst-graph-legends-bg, initial);
				max-width:var(--flow-sunburst-graph-legends-max-width, 300px);
				max-height:var(--flow-sunburst-graph-legends-max-height, 100%);
				overflow:var(--flow-sunburst-graph-legends-overflow, auto);
				display:flex;align-items:center;
				flex-direction:column;
			}
			.legends .items>div{
				display:flex;align-items:center;
			}
			.color-box{
				display:inline-block;
				margin:var(--flow-sunburst-graph-color-box-margin, 2px 10px 2px 0px);
				width:var(--flow-sunburst-graph-color-box-width, 20px);
				min-width:var(--flow-sunburst-graph-color-box-width, 20px);
				height:var(--flow-sunburst-graph-color-box-height, 20px);
				opacity:var(--flow-sunburst-graph-color-box-opacity, 1);
			}
			.tip{opacity:0;zIndex:-1;transition:opacity 0.5s ease;display:inline-block}
		`];
	}

	render() {

		dpc(()=>{
			this.draw();
		})

		return html`
			<div class='wrapper'>
				<div class="d3-holder">${super.render()}</div>
				<div class="legends"></div>
				<div class="tip"></div>
			</div>
			`;
	}

	constructor() {
		super();
		this.sampler = '';
		this.svgPreserveAspectRatio = 'xMaxYMax meet';
		this.d3margin = 10;
	}

	firstUpdated(){
		super.firstUpdated();
		this.el_wrapper = this.renderRoot.querySelector(".wrapper");
		//this.el_d3Holder = this.renderRoot.querySelector(".d3-holder");
		this.el_legends = this.el_legends||this.renderRoot.querySelector(".legends");
		this.outerBox = this.el_wrapper.getBoundingClientRect();
		this.updateD3Holder()
	}

	updateD3Holder(){
		let {width, height} = this.el_wrapper.getBoundingClientRect();
		let {width:lWidth} = this.el_legends.getBoundingClientRect();
		width -= lWidth;
		let size = width>height? height:width;
		this.el_d3.style.width = (size-this.d3margin)+"px";
		this.el_d3.style.height = (size-this.d3margin)+"px";
		this.el_d3Rect = this.getBoundingClientRect.call(this.el_d3);
	}

	onElementResize(){
		super.onElementResize();
		this.outerBox = this.el_wrapper.getBoundingClientRect();
		dpc(()=>{
			this.outerBox = this.el_wrapper.getBoundingClientRect();
			this.updateD3Holder();
			this.requestUpdate("outerBox", null);
		})
	}

	connectedCallback() {
		super.connectedCallback();
		if(this.sampler)
			this.interval = setInterval(this.requestUpdate.bind(this), this.refresh);
	}

	disconnectedCallback() {
		super.disconnectedCallback();

		if(this.interval)
			clearInterval(this.interval);
	}

	getMargin(){
		if(this.axes){
			return {
				bottom:40,
				top:30,
				left:20,
				right:20
			}
		}
		return {
			bottom:0,
			top:0,
			left:0,
			right:0
		}
	}
	draw(){
		let margin = this.getMargin();
		let data = this.data;
		if(!data || !data.children)
			return

		const self = this;
		const box = this.el_d3.getBoundingClientRect();

		let {height:fullHeight, width:fullWidth} = box;
		let width = fullWidth - margin.left - margin.right;
    	let height = fullHeight - margin.top - margin.bottom;

		const root = this.partition(data);
  		root.each(d => d.current = d);


		const { el } = this;
		let t = `translate(${(width/2)+margin.left},${(height/2)+margin.top})`;
		if(el.__t != t){
			el.__t = t
			el.attr("transform", t)
		}

		if(this.svg.__w != fullWidth){
			this.svg.__w = fullWidth;
			this.svg
				.attr("width", fullWidth)
		}
		if(this.svg.__h != fullHeight){
			this.svg.__h = fullHeight;
			this.svg
				.attr("height", fullHeight)
		}

		let length = this.getDataItemsCount(data);


		let color = d3.scaleOrdinal(d3.quantize(d3.interpolateRainbow, length + 1));
		let format = d3.format(",d");
		let radiusRef = width<height?width:height;
		let radius = radiusRef / 10;
		let offsetR = radius*2;
		let arc = d3.arc()
			.startAngle(d => d.x0)
			.endAngle(d => d.x1)
			.padAngle(d => Math.min((d.x1 - d.x0) / 2, 0.005))
			.padRadius(radius * 5)
			.innerRadius(d => d.y0 * radius + offsetR)
			.outerRadius(d => Math.max(d.y0 * radius +offsetR, d.y1 * radius - 1 + offsetR))

		
		if(!this.rootPaths){
			this.rootPaths = el.append("g")
				.attr("class", "paths")

			this.labels = el.append("g")
				.attr("pointer-events", "none")
				.attr("text-anchor", "middle")
				.style("user-select", "none")

			this.centerLabelHolder = el.append("g")
				.attr("class", "center-label")
			this.centerLabel1 = this.centerLabelHolder
				.append("text")
				.attr("dy", -10)
				.attr("class", "center-label-top")
				.attr("text-anchor", "middle")
			this.centerLabel2 = this.centerLabelHolder
				.append("text")
				.attr("dy", 10)
				.attr("class", "center-label-bottom")
				.attr("text-anchor", "middle")
			this.el_tip = this.renderRoot.querySelector("div.tip");
			this.el_legends = this.el_legends||this.renderRoot.querySelector(".legends");
		}
		const {el_tip} = this;
		const path = this.rootPaths
		    .selectAll("path")
		    .data(root.descendants().slice(1))
		    .join("path")
				.attr("fill", d => {
					//c = d.data.color;
					//while (d.depth > 1)
					//	d = d.parent;
					//console.log("d.datad.data", d.data)
					return d.data.color || color(d.data.name);
				})
				//.attr("fill-opacity", d => self.arcVisible(d.current) ? (d.children ? 1 : 0.4) : 0)
				.attr("d", d => arc(d.current));
		
		path.filter(d => d.children)
			.style("cursor", "pointer")
			.on("click", clicked)
		path
			.on("mouseenter", mouseenter)
			.on("mousemove", mousemove)
			.on("mouseleave", mouseleave);


		//const title = path.selectAppend("title")
      	//	.text(d => `${d.ancestors().map(d => d.data.name).reverse().join("/")}\n${this.format(d.value, d)}`);


		let label;
		if(this.useLabels)
			label = this.labels
			.selectAll("text")
			.data(root.descendants().slice(1))
			.join("text")
				.attr("dy", "0.35em")
				.attr("fill-opacity", d => +labelVisible(d.current))
				.attr("transform", d => labelTransform(d.current))
				.text(d => d.data.name);

		this.centerLabel1.text(root.data.title||root.data.name)
		if(root.data.subtitle)
			this.centerLabel2.text(root.data.subtitle)

		//console.log("root.descendants().slice(1)", root.descendants().slice(0))
		this.buildLegends(root, color);

		if(!this.circleEl)
			this.circleEl = el.append("circle")

	    const parent = this.circleEl
			.datum(root)
			.attr("r", radius+offsetR)
			.attr("fill", "none")
			.attr("pointer-events", "all")
			.on("click", clicked);

	    
		const noZoom = this.noZoom;
		function clicked(p, ...args) {
			if(noZoom)
				return
			parent.datum(p.parent || root);
			//console.log("p.depth", args, p, p.x0, p.x1, p.depth);
			//return
			el_tip.style.top = p.y0+"px";
			el_tip.style.left = p.x0+"px";


			root.each(d => d.target = {
				x0: Math.max(0, Math.min(1, (d.x0 - p.x0) / (p.x1 - p.x0))) * 2 * Math.PI,
				x1: Math.max(0, Math.min(1, (d.x1 - p.x0) / (p.x1 - p.x0))) * 2 * Math.PI,
				y0: Math.max(0, d.y0 - p.depth),
				y1: Math.max(0, d.y1 - p.depth)
			});

			const t = el.transition().duration(750);

			// Transition the data on all arcs, even the ones that aren’t visible,
			// so that if this transition is interrupted, entering arcs will start
			// the next transition from the desired position.
			let transition =  path.transition(t)
				.tween("data", d => {
					const i = d3.interpolate(d.current, d.target);
					return t => d.current = i(t);
				})
				.filter(function(d) {
					return +this.getAttribute("fill-opacity") || self.arcVisible(d.target);
				})
				.attr("fill-opacity", d => self.arcVisible(d.target) ? (d.children ? 0.6 : 0.4) : 0)
				.attrTween("d", d => () => arc(d.current))
			let size = transition.size();
				transition
				.on("end", (d)=>{
					//console.log("endendendendend", d, --size)
					if(!size)
						self.buildLegends(p.parent || root, color)
				})
			

			if(!label)
				return
			label.filter(function(d) {
				return +this.getAttribute("fill-opacity") || labelVisible(d.target);
			})
			.transition(t)
			.attr("fill-opacity", d => +labelVisible(d.target))
			.attrTween("transform", d => () => labelTransform(d.current));
		}

		function hideTip(){
			el_tip.style.opacity = "0";
			el_tip.style.zIndex = "-1";
		}
		function mouseenter(...args) {
			self.buildTip(...args);
			self.showTip(...args);
		}
		function mouseleave(...args) {
			hideTip(...args)
		}
		function mousemove(...args){
			self.showTip(...args);
		}

		function labelVisible(d) {
			return d.y1 <= 3 && d.y0 >= 1 && (d.y1 - d.y0) * (d.x1 - d.x0) > 0.03;
		}

		function labelTransform(d) {
			//console.log("d.x0 + d.x1", d.x0 , d.x1)
			const x = (d.x0 + d.x1) / 2 * 180 / Math.PI;
			const y = ((d.y0 + d.y1) / 2 * radius)+offsetR;
			return `rotate(${x - 90}) translate(${y},0) rotate(${x < 180 ? 0 : 180})`;
		}
	}

	arcVisible(d) {
		return d.y1 <= 3 && d.y0 >= 1 && d.x1 > d.x0;
	}

	buildLegends(root, color){
		let shown = new Map();
		let legendHtml = html`<div class="items">${
			root.descendants().slice(1).map(d=>{
				if(!this.arcVisible(d.current))
					return false;
				if(d.data.legendOnce){
					if(shown.has(d.data.legendOnce))
						return false;
					shown.set(d.data.legendOnce, 1);
				}
				return html`
				<div class="item">
					<div class="color-box" style="background-color:${d.data.color||color(d.data.name)}"></div>
					<div class="name">${d.data.name}</div>
				</div>`
			}).filter(a=>a)
		}</div>`;
		render(legendHtml, this.el_legends);
	}

	showTip(box, ...args){
		let {pageX, pageY} = d3.event;
		//if(!this.outerBox)
			this.outerBox = this.el_wrapper.getBoundingClientRect();
		let {left, top, right, width, height} = this.outerBox;
		let x = pageX-left+15, y = pageY-top+15;
		const {el_tip} = this;
		
		el_tip.style.opacity = "0";
		el_tip.style.zIndex = "-1";
		let r = x+el_tip.offsetWidth;
		let b = y+el_tip.offsetHeight;
		let tipLeft, tipTop;
		//console.log("showTip",  {x, y}, el_tip.offsetWidth, right, width, r)
		if(r>width){
			tipLeft = width-el_tip.offsetWidth;
		}else{
			tipLeft = x;
		}

		if(b>height){
			tipTop = height-el_tip.offsetHeight;
		}else{
			tipTop = y;
		}

		if(tipLeft<x && tipTop<y){
			if(y > height/2){
				tipTop = y - el_tip.offsetHeight - 20;
			}
		}

		//console.log("this.outerBox", this.outerBox, tipLeft, tipTop)
		el_tip.style.left = `${tipLeft}px`;
		el_tip.style.top = `${tipTop}px`;
		el_tip.style.opacity = "1";
		el_tip.style.zIndex = "1";
	}
	buildTip(d, ...args){
		//console.log("buildTip",  d, ...args)
		let tpl = html`
			<div class="name">${d.ancestors().slice(0, -1).map(d => d.data.name).reverse().join(" / ")}</div>
			<div class="value">${this.format(d.value, d)}</div>`;
		render(tpl, this.el_tip);
	}
	format(value, d){
		if(!this.formatFn)
			this.formatFn = d3.format(",d");
		return this.formatFn(value);
	}

	getDataItemsCount(data){
		let count = data.children?.length||0;
		data.children?.forEach(child=>{
			count+=this.getDataItemsCount(child)
		});
		return count;
	}

	partition(data){
		const root = d3.hierarchy(data)
			.sum(d => d.value)
			.sort((a, b) => b.value - a.value);
		return d3.partition()
			.size([2 * Math.PI, root.height + 1])(root);
	}
}

FlowSunburstGraph.define('flow-sunburst-graph');

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_SWIPEABLE_382 : &'static str = r###"


export const swipeableStyle = css`
.flow-swipeable-container { overflow: hidden }
.flow-swipeable-row{
	--swipeable-n: 1;
	--swipeable-f: 1;
	display: flex;
	align-items: stretch;
	overflow:hidden;
	overflow-y: hidden;
	width: 100%; /* fallback */
	width: calc(var(--swipeable-n) * 100%);
	/*max-height: 100vh;*/
	--swipeable-transform-x: calc(var(--swipeable-tx, 0px) + var(--swipeable-i, 0) / var(--swipeable-n) * -100%);
	transform: translateX(var(--swipeable-transform-x));
}
.flow-swipeable-row .flow-swipeable{
	width: 100%; /* fallback */
	height: 100%;
	_width: calc(100% / var(--swipeable-n));
	/*user-select: none;
	pointer-events: none;
	background: no-repeat;
	background-size: cover;*/
	
}

.flow-swipeable-smooth{ transition: transform  calc(var(--swipeable-f, 1) * .5s) ease }
`

export class FlowSwipeable{

	constructor(container, options={}){
		this.container = container;
		let element = container.querySelector('.flow-swipeable-row');
		this.element = element;
		let defaultOptions = {
			drag:true,
			validateEvent(e){
				return !e.target.closest('flow-dropdown,flow-select,flow-selector,flow-input,flow-checkbox,select,textarea, input,.not-swipeable')
			}
		};
		this.options = {...defaultOptions, ...options}
		this.count = element.children.length;
		this.x = null;
		this.locked = false;
		this.i = 0;
		this.onResize();
		this.updateCount();
		this.init();
	}
	updateCount(){
		let el = this.element;
		this.count = el.children.length;
		el.style.setProperty("--swipeable-n", this.count);
		this.updateFixedPositionsOffset();
	}

	init(){
		let el = this.element;
		//let onResize = this.onResize.bind(this);
		let onTouchStart = this.onTouchStart.bind(this);
		let onDrag = this.onDrag.bind(this);
		let onTouchEnd = this.onTouchEnd.bind(this);

		//el.addEventListener("resize", onResize, false);

		el.addEventListener("mousedown", onTouchStart, false);
		el.addEventListener("touchstart", onTouchStart, false);

		el.addEventListener("mousemove", onDrag, false);
		el.addEventListener("touchmove", onDrag, false);

		el.addEventListener("mouseup", onTouchEnd, false);
		el.addEventListener("touchend", onTouchEnd, false);

		if (typeof MutationObserver != 'undefined'){
			const observer = new MutationObserver(()=>{
				this.updateCount();
			});
			observer.observe(el, {childList:true});
		}

		

		this.startResizeListener();
	}
	updateFixedPositionsOffset(){
		let {width, top} = this.container.getBoundingClientRect();
		[...this.element.children].map((c, index)=>{
			c.style.setProperty('--flow-transform-translate-x', `${index * width}px`);
			c.style.setProperty('--flow-transform-translate-y', `${-top}px`)
		})
	}
	setActive(index){
		this.element.style.setProperty("--swipeable-i", index);
		this.i = index;
	}
	startResizeListener(){
		if(!this.resizeObserver){
    		this.resizeObserver = new ResizeObserver(()=>{
	    		this.onResize();
			});
			this.resizeObserver.observe(this.container);
	    }
	}
	stopResizeListener(){
		if(this.resizeObserver){
			this.resizeObserver.unobserve(this.container);
			this.resizeObserver.disconnect();
			delete this.resizeObserver;
		}
	}

	unifyEvent(e) {
		return e.changedTouches ? e.changedTouches[0] : e;
	}

	isValidEvent(e){
		return this.options.validateEvent(e);
	}

	onResize() {
		this.width = this.container.getBoundingClientRect().width;
		this.updateFixedPositionsOffset();
	}

	onTouchStart(e) {
		if(!this.isValidEvent(e))
			return
		this.x = this.unifyEvent(e).clientX;
		this.element.classList.toggle("flow-swipeable-smooth", !(this.locked = true));
	}

	onDrag(e) {
		if (!this.locked)
			return
		if(this.options.drag){
			e.preventDefault();
			this.element.style.setProperty("--swipeable-tx", 
				`${Math.round(this.unifyEvent(e).clientX - this.x)}px`);
		}
	}

	onTouchEnd(e) {
		//console.log("locked:"+this.locked)
		if (!this.locked)
			return

		let el = this.element;
		let {i, count, width, x} = this;
		let lastIndex = i;
		//console.log("i, count, width, x", {i, count, width, x})

		let dx = this.unifyEvent(e).clientX - x,
			s = Math.sign(dx),
			f = +((s * dx) / width).toFixed(2);


		//console.log("i, count, width, x", {dx, i, f, s})
		if ((i > 0 || s < 0) && (i < count - 1 || s > 0) && f > 0.1) {
			el.style.setProperty("--swipeable-i", (i -= s));
			f = 1 - f;
		}
		this.i = i;
		el.style.setProperty("--swipeable-tx", "0px");
		el.style.setProperty("--swipeable-f", f);
		el.classList.toggle("flow-swipeable-smooth", !(this.locked = false));
		this.x = null;
		if(lastIndex != i){
			this.options.onSwipe?.({index:i, element:this.element.children[i]})
		}
	}
}
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_T9_427 : &'static str = r###"



/**
* @class FlowT9
* @extends BaseElement

* @property {Boolean} [disabled]
* @property {String} [value]
*
* @cssvar {font-family} [--flow-font-family="Julius Sans One"]
* @cssvar {font-weight} [--flow-font-weight=bold]
* @example
*   <flow-t9 value="123.4"></flow-t9>
*
*/
export class FlowT9 extends BaseElement {
	static get properties() {
		return {
			value:{type:String},
			disabled:{type:Boolean}
		}
	}

	static get styles(){
		return css`
			:host{
				display:inline-block;
				font-family:var(--flow-font-family, "Julius Sans One");
				font-weight:var(--flow-font-weight, bold);
				width:var(--flow-t9-width, 100%);
			}
			.row{
				display:flex;
				align-items:stretch;
				min-width:60px;
				text-align:center;
				justify-content:space-evenly;
				margin-bottom:5px;
			}
			flow-btn{
				margin:var(--flow-t9-btn-margin, 5px);
				padding:var(--flow-t9-btn-padding, 0px);
				box-size:border-box;
				border-radius:var(--flow-t9-btn-border-radius, 50%);
				--flow-btn-wrapper-min-width:10px;
				width:var(--flow-t9-btn-width, 50px);
				height:var(--flow-t9-btn-height, 50px);
    			font-size:var(--flow-t9-btn-font-size, 1.5rem);
			}
		`;
	}
	render() {
		let {value=""} = this;
		return html`
		<div class="wrapper" @click=${this.onClick}>
			<div class="row">
				<flow-btn full-height-wrapper data-v="1">1</flow-btn>
				<flow-btn full-height-wrapper data-v="2">2</flow-btn>
				<flow-btn full-height-wrapper data-v="3">3</flow-btn>
			</div>
			<div class="row">
				<flow-btn full-height-wrapper data-v="4">4</flow-btn>
				<flow-btn full-height-wrapper data-v="5">5</flow-btn>
				<flow-btn full-height-wrapper data-v="6">6</flow-btn>
			</div>
			<div class="row">
				<flow-btn full-height-wrapper data-v="7">7</flow-btn>
				<flow-btn full-height-wrapper data-v="8">8</flow-btn>
				<flow-btn full-height-wrapper data-v="9">9</flow-btn>
			</div>
			<div class="row">
				<flow-btn full-height-wrapper data-v="." 
					?disabled="${value.includes('.')}">.</flow-btn>
				<flow-btn full-height-wrapper data-v="0">0</flow-btn>
				<flow-btn full-height-wrapper data-v="backspace"
					?disabled="${!value}">&lt;</flow-btn>
			</div>
		</div>
		`;
	}

	setClear(){
		this.setValue("");
	}

	onClick(e) {
		let btnEl = e.target.closest("flow-btn")
		if(!btnEl)
			return
		let btn = btnEl.dataset.v;
		let {value=""} = this;
		if(btn=="." && value.includes("."))
			return

		let result = this.fire("btn-click", {el:this, btn, btnEl}, {cancelable:true}, null, true)
		if(result.defaultPrevented)
			return
		
		if(btn=="backspace"){
			value = value.substring(0, value.length-1);
		}else{
			if(btn=="." && value===""){
				value = "0"
			}
			value += btn;
		}

		this.setValue(value);
	}

	setValue(value){
		this.value = value;
		this.fire("changed", {el:this, value:this.value})
	}
}

FlowT9.define('flow-t9');
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_PROGRESSBAR_388 : &'static str = r###"


//
/*
window.colorMixer = (q=0.5)=>{
	let color = colorMixer("#E0101B", "#28F003", q)
	console.log("color####", color)
	document.body.style.backgroundColor = color
}
*/



/**
* @class FlowProgressbar
* @extends BaseElement
* @property {String} [color]
* @property {String} [easing]
* @example
*   <flow-progressbar color="#red" value="0.3"></flow-progressbar>
*
*
*/
export class FlowProgressbar extends BaseElement {
	static get properties() {
		return {
			value:{type: Number},
			strokeWidth:{type: Number},
			trailWidth:{type: Number},
			trailColor:{type: String},
			color:{type: String},
			easing:{type: String},
			svgStyle:{type: String},
			shape:{type:String},
			opt:{type: Object},
			text:{type:String}
		}
	}

	static get styles() {
		return css`
			:host {
				display:inline-block;
				width:var(--flow-progressbar-width, 30px);
				height:var(--flow-progressbar-height, 30px);
				/*
				--flow-progressbar-color:red;
				--flow-progressbar-trail-color:green;
				*/
			}
			.container{width:100%;height:100%;}
			.progressbar-text{
				font-size:var(--flow-progressbar-font-size, 0.9rem);
				font-weight:var(--flow-progressbar-font-weight, normal);
				font-family:var(--flow-progressbar-font-family, inhert);
			}
		`;
	}

	render() {
		return html`<div class="container"></div>`;
	}
	updated(){
		super.updated();
		let {
			value=0, opt={},
			text='',
			strokeWidth, color, easing,
			duration, trailColor, trailWidth, svgStyle,
			shape="Circle"
		} = this;

		let defaultOpt = {
			strokeWidth: 6,
			easing: 'easeInOut',
			duration: 1400,
			color: 'var(--flow-progressbar-color, #FF0000)',
			trailColor: 'var(--flow-progressbar-trail-color, #efefef)',
			trailWidth: 6,
			svgStyle: null
		}
		let definedOpts = {color, easing, duration}
		let entries = Object.entries(definedOpts).filter(([k, v])=>v!==undefined)
		definedOpts = Object.fromEntries(entries)
		//console.log("definedOpts", definedOpts)

		let options = {...defaultOpt, ...opt, definedOpts};
		this.el = this.el || this.renderRoot.querySelector(".container")
		this.progress = this.progress || new ProgressBar[shape](this.el, options);
		this.progress.stop();
		this.progress.animate(value, {/*
			from: { color: options.color },
    		to: { color: options.color }
		*/}, ()=>{
			//this.progress.path.setAttribute('stroke', options.color);
		});

		this.progress.setText(text);
	}
}

FlowProgressbar.define('flow-progressbar');

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_ADD_TO_HOME_371 : &'static str = r###"



/**
* @class FlowAddToHome
* @extends BaseElement
* @example
*   <flow-add-to-home icon="logo.png"
		message="Add App to Home screen"></flow-add-to-home>
* @property {Boolean} [disabled] 
* @cssvar {font-family} [--flow-add2home-font-family=var(--flow-font-family, initial)]
*/
export class FlowAddToHome extends BaseElement {
	static get properties() {
		return {
			disabled:{type:Boolean, reflect: true},
			icon:{type:String},
			closeIcon:{type:String},
			message:{type:String},
			once:{type:Boolean}
		}
	}

	static get styles(){
		return css`
			:host{
				display:var(--flow-add2home-display, block);
				margin: var(--flow-add2home-margin);
				padding:var(--flow-add2home-padding, 10px);
				border: var(--flow-add2home-border, 2px solid var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1))));
				border-radius:var(--flow-add2home-radius, 2px);
				border-width:var(--flow-add2home-border-width, 2px);
				font-family:var(--flow-add2home-font-family, var(--flow-font-family, initial));
				font-weight:var(--flow-add2home-font-weight, var(--flow-font-weight, bold));
				font-size:var(--flow-add2home-font-size, initial);
				line-height:var(--flow-add2home-line-height, inherit);
				text-transform:var(--flow-add2home-text-transform, inherit);
				user-select: none;
				background-color:var(--flow-add2home-bg-color, var(--flow-primary-color, rgba(0,151,115,1)));
				border-color:var(--flow-add2home-border-color, var(--flow-primary-color, rgba(0,151,115,1)));
				color:var(--flow-add2home-invert-color, var(--flow-primary-invert-color, #FFF));
				--fa-icon-color:var(--flow-add2home-invert-color, var(--flow-primary-invert-color, #FFF));
				--fa-icon-size-temp:var(--fa-icon-size);
				cursor:pointer;
			}
			.icon{
				--fa-icon-size:var(--flow-add2home-icon-size, var(--fa-icon-size-temp, 20px));
				--fa-icon-padding:var(--flow-add2home-icon-padding, var(--fa-icon-padding));
				background-color:var(--flow-add2home-icon-bg);
				margin:0px 10px;
			}
			:host(:hover){
				background-color:var(--flow-add2home-hover-bg-color, var(--flow-primary-color, rgba(0,151,115,1)));
				color: var(--flow-add2home-hover-color);
			}
			.message{flex:1;word-wrap:break-word;}
			.close-icon{background-color:none}
			.wrapper{
				display:flex;
				align-items:center;
				margin:var(--flow-add2home-wrapper-margin, 0px);
				min-width:var(--flow-add2home-wrapper-min-width, 50px);
				text-align:center;
				justify-content:center;
				height:100%;
				box-sizing:border-box;
			}
			:host(:not(.active)){
				display:none;
			}
		`;
	}
	constructor(){
		super()
		
		if(getLocalSetting('add-to-home-disabled')==1){
			this.disabled = true;
		}else{
			//this.classList.add("active")
		}
		this.setAttribute('role', 'button');
	}
	render() {
		let {icon='', closeIcon='times'} = this;
		return html`
		<div class="wrapper">
			${icon?html`<fa-icon class="icon"
				icon=${icon} @click="${this.onAddClick}"></fa-icon>`:''}
			<div class="message" @click="${this.onAddClick}">
				${this.message||''}
				<slot></slot>
			</div>
			<fa-icon class="close-icon" icon=${closeIcon}
				@click="${this.onCloseClick}"></fa-icon>
		</div>`;
	}
	connectedCallback(){
		super.connectedCallback();
		if(!this.disabled)
			this.init();
	}
	init(){
		window.addEventListener('beforeinstallprompt', (e) => {
			//alert("beforeinstallprompt:"+e)
			// Prevent Chrome 67 and earlier from automatically showing the prompt
			e.preventDefault();
			// Stash the event so it can be triggered later.
			this.deferredPrompt = e;
			this.classList.add("active");
		})
	}

	onAddClick() {
		if(this.disabled || !this.deferredPrompt)
			return
		this.fire("add", {el:this})
		this.deferredPrompt.prompt();
		this.deferredPrompt.userChoice.then((choiceResult) => {
			if (choiceResult.outcome === 'accepted') {
				console.log('User accepted the A2HS prompt');
			} else {
				console.log('User dismissed the A2HS prompt');
			}
			this.deferredPrompt = null;
			this.close(choiceResult.outcome);
		});
	}
	onCloseClick(){
		this.close('closed');
	}
	close(reason='closed'){
		if(this.once){
			setLocalSetting('add-to-home-disabled', 1)
		}
		this.classList.remove("active");
		this.fire("closed", {el:this, reason})
	}
}

FlowAddToHome.define('flow-add-to-home');

"###;

const ASPECTRON_FLOW_UX_SRC_COLORS_402 : &'static str = r###"


export const buildColors = ({h, s, l}, opt={})=>{
	let {
		bg='hsl(var(--h), calc(var(--s) * 1%), calc(var(--l) * 1%))',
		color='hsl(var(--h), calc(var(--s) * 1%), calc(calc((-2500 * (20 / 100 + 1)) / (var(--l) - 49.999) + var(--l)) * 1%))',
	} = opt;

	const n = `color${(Math.random()*100000).toFixed(0)}`;

	bg = bg
		.replace(/var\(--h\)/g, `var(--${n}-h)`)
		.replace(/var\(--s\)/g, `var(--${n}-s)`)
		.replace(/var\(--l\)/g, `var(--${n}-l)`);
	color = color
		.replace(/var\(--h\)/g, `var(--${n}-h)`)
		.replace(/var\(--s\)/g, `var(--${n}-s)`)
		.replace(/var\(--l\)/g, `var(--${n}-l)`);

	let el = document.createElement("div");
	el.style.setProperty(`--${n}-h`, h);
	el.style.setProperty(`--${n}-s`, s)
	el.style.setProperty(`--${n}-l`, l)
	el.style.backgroundColor = bg
	el.style.color = color;
	el.style.display = 'none';
	el.style.position = 'fixed';
	document.body.appendChild(el);
	let p = new Promise((resolve)=>{
		dpc(200, e=>{
			let style = getComputedStyle(el);
			let backgroundColor = style.backgroundColor;
			let color = style.color;
			resolve({backgroundColor, color});
			dpc(e=>{
				el.remove();
			})
		})
	});
	p.element = el;
	return p;
}

export const hexToRgb = (hex)=>{
	if(hex[0]!="#")
		hex = '#'+hex
	let a;
	if(hex.length < 7)
		a = hex
			.split('')
			.reduce((rgb, val, idx) => `${rgb}${val}0,`, '')
	else
		a = hex
			.split('')
			.reduce((rgb, val, idx) => rgb + (idx % 2 ? val : val + ','), '')

	let [r,g,b] = a.split(',').splice(1, 3).map(val => parseInt(val, 16))
	return {r,g,b};
}

export const rgbToHsl = ({r, g, b})=>{
	r /= 255, g /= 255, b /= 255
	const max = Math.max(r, g, b), min = Math.min(r, g, b)
	let h, s, l = (max + min) / 2

	if(max == min){
		h = s = 0;
	} else {
		let d = max - min
		s = l > 0.5 ? d / (2 - max - min) : d / (max + min)
		h = ({
			[r]: (g - b) / d,
			[g]: 2 + ( (b - r) / d),
			[b]: 4 + ( (r - g) / d),
		})[max] * 60
		if (h < 0) h +=360
	}
	s *= 100
	l *= 100
	return { h, s, l }
}

//colorChannelA and colorChannelB are ints ranging from 0 to 255
export function colorChannelMixer(colorChannelA, colorChannelB, amountToMix){
    var channelA = colorChannelA*amountToMix;
    var channelB = colorChannelB*(1-amountToMix);
    return parseInt(channelA+channelB);
}
//rgbA and rgbB are arrays, amountToMix ranges from 0.0 to 1.0
//example (red): rgbA = [255,0,0]
export function colorMixer(rgbA, rgbB, amountToMix=1){
	rgbA = hexToRgb(rgbA)
	rgbB = hexToRgb(rgbB)
    var r = colorChannelMixer(rgbA.r,rgbB.r, amountToMix);
    var g = colorChannelMixer(rgbA.g,rgbB.g, amountToMix);
    var b = colorChannelMixer(rgbA.b,rgbB.b, amountToMix);
    return "rgb("+r+","+g+","+b+")";
}

function mixCMYKS(...cmyks) {
	let c = cmyks.map(cmyk => cmyk[0]).reduce((a, b) => a + b, 0) / cmyks.length;
	let m = cmyks.map(cmyk => cmyk[1]).reduce((a, b) => a + b, 0) / cmyks.length;
	let y = cmyks.map(cmyk => cmyk[2]).reduce((a, b) => a + b, 0) / cmyks.length;
	let k = cmyks.map(cmyk => cmyk[3]).reduce((a, b) => a + b, 0) / cmyks.length;
	return [c, m, y, k];
}

function mixHexs(...hexes) {
	let rgbs = hexes.map(hex => hex2dec(hex)); 
	let cmyks = rgbs.map(rgb => rgb2cmyk(...rgb));
	let mixture_cmyk = mix_cmyks(...cmyks);
	let mixture_rgb = cmyk2rgb(...mixture_cmyk);
	let mixture_hex = rgb2hex(...mixture_rgb);
	return mixture_hex;
}

export const rgb2cmyk = (r, g, b)=>{
	let c = 1 - (r / 255);
	let m = 1 - (g / 255);
	let y = 1 - (b / 255);
	let k = Math.min(c, m, y);
	c = (c - k) / (1 - k);
	m = (m - k) / (1 - k);
	y = (y - k) / (1 - k);
	return [c, m, y, k];
}

export const cmyk2rgb = (c, m, y, k)=>{
	let r = c * (1 - k) + k;
	let g = m * (1 - k) + k;
	let b = y * (1 - k) + k;
	r = (1 - r) * 255 + .5;
	g = (1 - g) * 255 + .5;
	b = (1 - b) * 255 + .5;
	return [r, g, b];
}

export const parseRGBA = color=>{
	let [r,g,b,a=100] = color.split("(")[1].split(")")[0].split(",");
	return {r,g,b,a};
}

export const color2hsl = (color)=>{
	if(color.h)
		return color;
	if(color.r)
		return rgbToHsl(color);
	if(color[0]=="#")
		return rgbToHsl(hexToRgb(color));
	color = color.toLowerCase();
	if(color[0]=="r")
		return rgbToHsl(parseRGBA(color));
}

export const hex2Colors = (color="#FF0000", opt={})=>{
	const hsl = color2hsl(rgb);
	return buildColors(hsl, opt);
}
export const rbg2Colors = (rgb={r:255, g:0, b:0}, opt={})=>{
	const hsl = color2hsl(rgb);
	return buildColors(hsl, opt);
}
export const hsl2Colors = (hsl={h:0, s:100, l:50}, opt={})=>{
	return buildColors(hsl, opt);
}

export const testColor = async(color="#FF0000")=>{
	const rgb = hexToRgb(color);
	const hsl = rgbToHsl(rgb);
	console.log("testColor::hexToRgb", rgb)
	console.log("testColor::rgbToHsl", hsl)
	let {backgroundColor, color:c} = await buildColors(hsl);
	console.log("testColor::backgroundColor, color", backgroundColor, c)
}

//testColor();

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_STATSD_407 : &'static str = r###"




// /**
// * @class FlowQRCode
// * @extends BaseElement
// * @property {String} [for]
// * @property {String} [type]
// * @example
// *   <flow-qrcode></flow-qrcode>
// *
// */

export class FlowStatsD extends BaseElement {
	static get properties(){
        return {
        
            url:{type:String},
            target:{type:String},
            from:{type:String},
            prefix:{type:String}
        }
            
    }
	static get styles() {
		return css`
			:host {
				display : block;
                width:100%;
                height:100%;

			}
			
			:host(.left-img) img{
				object-position:left;
			}
            img {
                width:100%;
                height: 100%
            }
		`;
	}

	constructor() {
		super();

		this.url = "https://metrics.aspectron.com";
	}

    onElementResize(){
    	this.rect = this.getBoundingClientRect();
        //console.log("RECT:", this.rect);
        this.requestUpdate();
    }

    connectedCallback(){
    	super.connectedCallback();
    	//this._onWindowResize = this._onWindowResize || this.onWindowResize.bind(this);
		//window.addEventListener("resize", this._onWindowResize)
		if(!this.__resizeObserver){
    		this.__resizeObserver = new ResizeObserver(()=>{
	    		this.onElementResize();
			});
			this.__resizeObserver.observe(this);
	    }
        this.interval = setInterval(this.requestUpdate.bind(this),1000);
    }

    disconnectedCallback(){
        super.disconnectedCallback();
        clearInterval(this.interval);
    }

	render() {
        const time = Date.now();
        this.prefix = "stats.gauges."
        const rect = this.rect || {width:64,height:64};
        const {width, height} = rect;

        const searchParams = new URLSearchParams();
        searchParams.set("width", width || 64);
        searchParams.set("height", height || 64);
        searchParams.set("areaMode", "all");
        searchParams.set("from", "-30minutes");
        searchParams.set("areaAlpha", "0.5");
        let targets = this.target.split("|");
        targets.forEach(target=>{
            target = this.prefix+target;
            searchParams.append("target",target);

        })
        //searchParams.set("target", this.target);
        searchParams.set("hideLegend", false);
        searchParams.set("salt", time);
        const url = `${this.url}/render/?`+searchParams.toString();



        // const queryString = "http://192.168.1.168/render/?
        // showTarget=stats.gauges.market.ETHBTC.orders_per_sec&
        // showTarget=stats.gauges.a.b.c&
        // showTarget=stats.gauges.statsd.timestamp_lag
        // &width=586&
        // height=308&
        // from=-1minutes&
        // areaMode=all&
        // target=stats.gauges.generic.market.sys_mem_used&
        // _salt=1620078647.885";
		return html`<img style="width:${width}px;height:${height}px" src="${url}">`
	}

	

}

FlowStatsD.define('flow-statsd');

//http://192.168.1.168/render/?showTarget=stats.gauges.market.ETHBTC.orders_per_sec&showTarget=stats.gauges.a.b.c&showTarget=stats.gauges.statsd.timestamp_lag&width=586&height=308&from=-1minutes&areaMode=all&target=stats.gauges.generic.market.sys_mem_used&_salt=1620078647.885
"###;

const ASPECTRON_FLOW_UX_SRC_HELPERS_409 : &'static str = r###"
/*
* Flow-UX: src/helper.js
* version: 1.0.0
*/
const toString = Object.prototype.toString;
const is = (obj, type) =>toString.call(obj)=='[object '+type+']'

export const utils = {};
utils.toString = toString;
utils.is = is;
utils.isArray = obj=>Array.isArray(obj);
utils.isObject = obj=>is(obj, 'Object');
utils.isString = obj=>is(obj, 'String');
utils.isNumber = obj=>is(obj, 'Number');
utils.isBoolean = obj=>is(obj, 'Boolean');
utils.isFunction = obj=>is(obj, 'Function');
utils.isUndefined = obj=>is(obj, 'Undefined');
utils.valueToDataType = (value)=>{
	return window[toString.call(value).split("object")[1]?.replace("]", "").trim()||""]
}

const storage = () => {
	if(typeof global != 'undefined')
		return global
	if(typeof globalThis != 'undefined')
		return globalThis
	return  window;
}

let uid_vec = new Uint32Array(6);
const UID = (len = 26)=>{
	window.crypto.getRandomValues(uid_vec);
	return [...uid_vec].map(v=>bs58e(v)).join('').substring(0,len);
}

window.UID = UID;

if(!window.OnReCaptchaLoad){
	window.OnReCaptchaLoad = ()=>{
		//console.log("OnReCaptchaLoad OnReCaptchaLoad OnReCaptchaLoad")
		trigger("g-recaptcha-ready")
		buildReCaptcha();
	}
}

const universe = storage();
const default_flow_global = { }
const flow = universe.flow = (universe.flow || default_flow_global);

let {debug, baseUrl, theme} = window.flowConfig || flow;
let {iconPath, icons, resolveIcon, iconMap, iconFile} = theme || {};

if(!baseUrl){
	baseUrl = (new URL("../", import.meta.url)).href;
	debug && console.log("FlowUX: baseUrl", baseUrl)
}
const isSmallScreen = navigator.userAgent.toLowerCase().includes("mobi");

const IconMap = Object.assign({
	fal:'light',
	far:'regular',
	fab:'brands',
	fa: 'solid',
	fas:'solid'
}, iconMap || {});

iconFile = iconFile||'icons';
const NativeIcons = baseUrl+'resources/icons/sprites/';
const FlowIconPath = iconPath || '/resources/fonts/sprites/';//NativeIcons;
const FlowIcons = icons || {};
const FlowStates = Object.freeze({ONLINE:'online', AUTH:'auth'});

if(!resolveIcon){
	resolveIcon = (cname, name, i)=>{
		if(!name)
			return `${cname}:invalid icon`;
		if(/\.(.{3,4}|.{3,4}#.*)$/.test(name))
			return name
		
		let icon = FlowIcons[`${cname}:${name}${i?'-'+i:''}`]
			||FlowIcons[name]
			||(name.indexOf(":")>-1?name:iconFile+':'+name);

		if(/\.(.{3,4}|.{3,4}#.*)$/.test(icon))
			return icon

		let [file, hash] = icon.split(":");
		if(file == "icons")
			return `${NativeIcons}icons.svg#${hash}`;
		return `${FlowIconPath}${IconMap[file]||file}.svg#${hash}`;
	}
}

// console.log("FlowIcons", FlowIcons) 

const dpc = (delay, fn)=>{
	if(typeof delay == 'function')
		return setTimeout(delay, fn||0);
	return setTimeout(fn, delay||0);
}

const clearDPC = (dpc_)=>{
	clearTimeout(dpc_);
}

const deferred = () => {
    let methods = {};
    const p = new Promise((resolve, reject) => {
        methods = { resolve, reject };
    });
    return Object.assign(p, methods);
}

export { dpc, clearDPC, deferred };

export const setTheme = theme=>{
	const body = document.body; 
	[...body.classList]
	.filter(cls=>cls.startsWith('flow-theme'))
	.forEach(cls=>body.classList.remove(cls));

	body.classList.add(`flow-theme-${theme}`);
	localStorage.flowtheme = theme;
	//let ce = new CustomEvent("flow-theme-changed", {detail:{theme}});
	//body.dispatchEvent(ce);
	trigger("flow-theme-changed", {theme}, {bubbles:true}, body);
	//trigger("flow-theme-changed", {theme}, {bubbles:true}, window);
}

//window.setTheme = setTheme;
export const DefaultTheme = theme?.default||"light";
export const getTheme = (defaultTheme=DefaultTheme)=>{
	if(localStorage.flowtheme)
		return localStorage.flowtheme;
	const body = document.body;
	let theme = [...body.classList]
	.find(cls=>cls.startsWith('flow-theme'))
	if(!theme)
		return defaultTheme;

	return theme.replace("flow-theme-", "");
}
export {IconMap, FlowIcons, NativeIcons, isSmallScreen, FlowStates};
export {baseUrl, debug, FlowIconPath, flow, UID, storage, resolveIcon};


export const styleAppendTo = (style, defaultSelector="head")=>selector=>{
	let p = document.querySelector(selector||defaultSelector);
	if(!p)
		p = document.head;
	if(p.matches("style")){
		p.innerHTML = style.cssText;
	}else{
		let sBar = document.createElement('style');
		sBar.innerHTML = style.cssText;
		p.appendChild(sBar);
	}
}

const bs58e = (function() {
    var alphabet = '123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ',
        base = alphabet.length;
    return function(enc) {
		var encoded = '';
		while(enc) {
			var remainder = enc % base;
			enc = Math.floor(enc / base);
			encoded = alphabet[remainder].toString() + encoded;        
		}
		return encoded;
	}
})();

const resolveConditions = (src) => {
	src = Object.entries(src).map(([k,v]) => {
		return eval(k) ? undefined : v;
	}).filter(v=>v!==undefined);
	if(!src.length)
		return null;
	return src;
}

export const DeferComponent = (ctor, name, deps) => {

	if(deps && typeof deps == 'object' && !Array.isArray(deps))
		deps = resolveConditions(deps);

    if(deps && Array.isArray(deps)) {
		let count = 0;
		deps = deps.slice();		
		while(deps.length) {
			let src = deps.shift();

			switch(typeof src) {
				case 'function': {
					src = src();
					if(!src)
						continue;
					if(Array.isArray(src)) {
						deps = deps.concat(src);
						continue;
					}
				} break;
				case 'object': {
					src = resolveConditions(src);
					if(!src)
						continue;
					else
					if(src.length == 1)
						src = src.shift();
					else {
						deps = deps.concat(src);
						continue;
					}
				} break;
			}

			console.log('Flow - loading dep:', src);
            count++;
            let el
            if(/\.css$/.test(src)){
            	el = document.createElement("link");
            	el.setAttribute("rel", "stylesheet");
            	el.setAttribute("type", "text/css")
            	el.href = src;
            }else{
            	el = document.createElement("script");
            	el.src = src;
            }
            document.head.appendChild(el);
            el.onload = ()=>{
                count--;
                if(!count)
                    ctor.defineElement(name);
            }        
        }
	}
	else
		ctor.defineElement(name);
}

export const sizeClsMap = new Map();
sizeClsMap.set("TINY", 400);
sizeClsMap.set("XXS", 550);
sizeClsMap.set("XS", 768);
sizeClsMap.set("SM", 992);
sizeClsMap.set("MD", 1200);

export const isArray = o=>Array.isArray(o);
export const isObject = o=>Object.prototype.toString.call(o)=='[object Object]';

export const camelCase = str=>{
  // Lower cases the string
  return (str+"").toLowerCase()
    // Replaces any - or _ characters with a space 
    .replace( /[-_\.]+/g, ' ')
    // Removes any non alphanumeric characters 
    .replace( /[^\w\s]/g, '')
    // Uppercases the first character in each group immediately following a space 
    // (delimited by spaces) 
    .replace( / (.)/g, function($1) { return $1.toUpperCase(); })
    // Removes spaces 
    .replace( / /g, '' );
}

export const humanize = str=>{
	// Lower cases the string
  	str = (str+"").toLowerCase()
    // Replaces any - or _ characters with a space 
    .replace( /[-_\.]+/g, ' ')
    // Removes any non alphanumeric characters 
    .replace( /[^\w\s]/g, '')
    // Uppercases the first character in each group immediately following a space 
    // (delimited by spaces) 
    .replace( / (.)/g, function($1) { return $1.toUpperCase(); })
    return str[0].toUpperCase()+str.substring(1);
    // Removes spaces 
    //.replace( / /g, '' );
}

export const deepClone = (obj, debug)=>{
	if(debug)
		console.log("deepClone:", obj, obj instanceof HTMLElement)
	if((typeof jQuery !='undefined' && obj instanceof jQuery) || (obj?.constructor?.prototype.jquery))
		return obj.clone();//$("___xxxxxx____");
	if(obj instanceof HTMLElement)
		return obj.cloneNode(true);
	if(isArray(obj))
		return obj.map(e=>deepClone(e, debug))
	if(isObject(obj)){
		let r = {};
		Object.entries(obj).forEach(([key, value])=>{
			r[key] = deepClone(value, debug);
		})
		return r;
	}
	return obj;
}

export class ExtendedMap extends Map{
	constructor(...args){
		super(...args)
		this.handlers = new Map();
	}
	setListener(type, handler){
		if(!["set", "clear", "delete"].includes(type)){
			return false
		}
		let handlers = this.handlers.get(type)
		if(!handlers)
			handlers = []
		if(!handlers.includes(handler))
			handlers.push(handler);
		this.handlers.set(type, handlers);
	}
	callHandlers(type, ...args){
		let handlers = this.handlers.get(type);
		if(!handlers)
			return
		handlers.forEach(fn=>{
			fn(...args);
		})
	}
	set(key, value){
		super.set(key, value);
		this.callHandlers("set", key, value);
	}
}

export const trigger = (eventName, detail={}, options={}, el=null, returnEvent=false)=>{
	let ev = new CustomEvent(eventName, Object.assign({}, options, {detail}));
	let result = (el || window).dispatchEvent(ev);
	return returnEvent?ev:result
}

export const buildReCaptcha = root=>{
	if(!window.grecaptcha)
		return
	grecaptcha.ready(()=>{
		root = root||document;
		root.querySelectorAll('.g-recaptcha').forEach((el)=>{
			let id = el.dataset.grecaptchaId;
			if(id !== undefined){
				grecaptcha.reset(id)
				return
			}

			id = grecaptcha.render(el, {
				'sitekey' : el.dataset.sitekey || document.body.dataset.recaptchaKey,
				callback(data){
					trigger("g-recaptcha", {code:"success", data}, {bubbles:true}, el)
				},
				'expired-callback':()=>{
					trigger("g-recaptcha", {code:"expired"}, {bubbles:true}, el)
				},
				'error-callback':()=>{
					trigger("g-recaptcha", {code:"error"}, {bubbles:true}, el)
				}
			});
			el.dataset.grecaptchaId = id;
		});
	})
}

export const chunks = (arr, size) =>{
	return arr.length ? [arr.slice(0, size), ...chunks(arr.slice(size), size)] : []
}
export const getRandomInt = (min, max)=>{
	min = Math.ceil(min);
	max = Math.floor(max);
	return Math.floor(Math.random() * (max + 1 - min) + min); //The maximum and minimum are inclusive
}

export const abbreviateNumber = (num, fixed)=>{
	if (num === null) { return null; } // terminate early
	if (num === 0 || !num) { return '0'; } // terminate early
	fixed = (!fixed || fixed < 0) ? 2 : fixed; // number of decimal places to show
	var b = (num).toPrecision(2).split("e"), // get power
		k = b.length === 1 ? 0 : Math.floor(Math.min(b[1].slice(1), 14) / 3), // floor at decimals, ceiling at trillions
		c = k < 1 ? num.toFixed(0 + fixed) : (num / Math.pow(10, k * 3) ).toFixed(1 + fixed), // divide by power
		d = c < 0 ? c : Math.abs(c), // enforce -0 is 0
		e = d + ['', 'K', 'M', 'B', 'T'][k]; // append power
	return e;
}

export const shuffle = (array)=>{
	let currentIndex = array.length, temporaryValue, randomIndex;

	// While there remain elements to shuffle...
	while (0 !== currentIndex) {
		// Pick a remaining element...
		randomIndex = Math.floor(Math.random() * currentIndex);
		currentIndex -= 1;

		// And swap it with the current element.
		temporaryValue = array[currentIndex];
		array[currentIndex] = array[randomIndex];
		array[randomIndex] = temporaryValue;
	}

	return array;
}

const fit = (contains)=>{
	return (parentWidth, parentHeight, childWidth, childHeight, scale = 1, offsetX = 0.5, offsetY = 0.5) => {
		const childRatio = childWidth / childHeight
		const parentRatio = parentWidth / parentHeight
		let width = parentWidth * scale
		let height = parentHeight * scale

		if (contains ? (childRatio > parentRatio) : (childRatio < parentRatio)) {
			height = width / childRatio
		} else {
			width = height * childRatio
		}

		return {
			width,
			height,
			offsetX: (parentWidth - width) * offsetX,
			offsetY: (parentHeight - height) * offsetY
		}
	}
}

export const contain = fit(true)
export const cover = fit(false)

"###;

const LIT_HTML_DIRECTIVES_REPEAT_505 : &'static str = r###"

/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */
const u=(e,s,t)=>{const r=new Map;for(let l=s;l<=t;l++)r.set(e[l],l);return r},c=s(class extends t{constructor(e){if(super(e),e.type!==r.CHILD)throw Error("repeat() can only be used in text expressions")}ht(e,s,t){let r;void 0===t?t=s:void 0!==s&&(r=s);const l=[],o=[];let i=0;for(const s of e)l[i]=r?r(s,i):i,o[i]=t(s,i),i++;return{values:o,keys:l}}render(e,s,t){return this.ht(e,s,t).values}update(s,[t,r,c]){var d;const a=l(s),{values:p,keys:v}=this.ht(t,r,c);if(!Array.isArray(a))return this.ut=v,p;const h=null!==(d=this.ut)&&void 0!==d?d:this.ut=[],m=[];let y,x,j=0,k=a.length-1,w=0,A=p.length-1;for(;j<=k&&w<=A;)if(null===a[j])j++;else if(null===a[k])k--;else if(h[j]===v[w])m[w]=o(a[j],p[w]),j++,w++;else if(h[k]===v[A])m[A]=o(a[k],p[A]),k--,A--;else if(h[j]===v[A])m[A]=o(a[j],p[A]),i(s,m[A+1],a[j]),j++,A--;else if(h[k]===v[w])m[w]=o(a[k],p[w]),i(s,a[j],a[k]),k--,w++;else if(void 0===y&&(y=u(v,w,A),x=u(h,j,k)),y.has(h[j]))if(y.has(h[k])){const e=x.get(v[w]),t=void 0!==e?a[e]:null;if(null===t){const e=i(s,a[j]);o(e,p[w]),m[w]=e}else m[w]=o(t,p[w]),i(s,a[j],t),a[e]=null;w++}else n(a[k]),k--;else n(a[j]),j++;for(;w<=A;){const e=i(s,m[A+1]);o(e,p[w]),m[w++]=e}for(;j<=k;){const e=a[j++];null!==e&&n(e)}return this.ut=v,f(s,m),e}});export{c as repeat};
//# sourceMappingURL=repeat.js.map

"###;

const LIT_HTML_LIT_HTML_613 : &'static str = r###"
/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */
var t;const i=window,s=i.trustedTypes,e=s?s.createPolicy("lit-html",{createHTML:t=>t}):void 0,o=`lit$${(Math.random()+"").slice(9)}$`,n="?"+o,l=`<${n}>`,h=document,r=(t="")=>h.createComment(t),d=t=>null===t||"object"!=typeof t&&"function"!=typeof t,u=Array.isArray,c=t=>u(t)||"function"==typeof(null==t?void 0:t[Symbol.iterator]),v=/<(?:(!--|\/[^a-zA-Z])|(\/?[a-zA-Z][^>\s]*)|(\/?$))/g,a=/-->/g,f=/>/g,_=RegExp(">|[ \t\n\f\r](?:([^\\s\"'>=/]+)([ \t\n\f\r]*=[ \t\n\f\r]*(?:[^ \t\n\f\r\"'`<>=]|(\"|')|))|$)","g"),m=/'/g,p=/"/g,$=/^(?:script|style|textarea|title)$/i,g=t=>(i,...s)=>({_$litType$:t,strings:i,values:s}),y=g(1),w=g(2),x=Symbol.for("lit-noChange"),b=Symbol.for("lit-nothing"),T=new WeakMap,A=h.createTreeWalker(h,129,null,!1),E=(t,i)=>{const s=t.length-1,n=[];let h,r=2===i?"<svg>":"",d=v;for(let i=0;i<s;i++){const s=t[i];let e,u,c=-1,g=0;for(;g<s.length&&(d.lastIndex=g,u=d.exec(s),null!==u);)g=d.lastIndex,d===v?"!--"===u[1]?d=a:void 0!==u[1]?d=f:void 0!==u[2]?($.test(u[2])&&(h=RegExp("</"+u[2],"g")),d=_):void 0!==u[3]&&(d=_):d===_?">"===u[0]?(d=null!=h?h:v,c=-1):void 0===u[1]?c=-2:(c=d.lastIndex-u[2].length,e=u[1],d=void 0===u[3]?_:'"'===u[3]?p:m):d===p||d===m?d=_:d===a||d===f?d=v:(d=_,h=void 0);const y=d===_&&t[i+1].startsWith("/>")?" ":"";r+=d===v?s+l:c>=0?(n.push(e),s.slice(0,c)+"$lit$"+s.slice(c)+o+y):s+o+(-2===c?(n.push(void 0),i):y)}const u=r+(t[s]||"<?>")+(2===i?"</svg>":"");if(!Array.isArray(t)||!t.hasOwnProperty("raw"))throw Error("invalid template strings array");return[void 0!==e?e.createHTML(u):u,n]};class C{constructor({strings:t,_$litType$:i},e){let l;this.parts=[];let h=0,d=0;const u=t.length-1,c=this.parts,[v,a]=E(t,i);if(this.el=C.createElement(v,e),A.currentNode=this.el.content,2===i){const t=this.el.content,i=t.firstChild;i.remove(),t.append(...i.childNodes)}for(;null!==(l=A.nextNode())&&c.length<u;){if(1===l.nodeType){if(l.hasAttributes()){const t=[];for(const i of l.getAttributeNames())if(i.endsWith("$lit$")||i.startsWith(o)){const s=a[d++];if(t.push(i),void 0!==s){const t=l.getAttribute(s.toLowerCase()+"$lit$").split(o),i=/([.?@])?(.*)/.exec(s);c.push({type:1,index:h,name:i[2],strings:t,ctor:"."===i[1]?M:"?"===i[1]?k:"@"===i[1]?H:S})}else c.push({type:6,index:h})}for(const i of t)l.removeAttribute(i)}if($.test(l.tagName)){const t=l.textContent.split(o),i=t.length-1;if(i>0){l.textContent=s?s.emptyScript:"";for(let s=0;s<i;s++)l.append(t[s],r()),A.nextNode(),c.push({type:2,index:++h});l.append(t[i],r())}}}else if(8===l.nodeType)if(l.data===n)c.push({type:2,index:h});else{let t=-1;for(;-1!==(t=l.data.indexOf(o,t+1));)c.push({type:7,index:h}),t+=o.length-1}h++}}static createElement(t,i){const s=h.createElement("template");return s.innerHTML=t,s}}function P(t,i,s=t,e){var o,n,l,h;if(i===x)return i;let r=void 0!==e?null===(o=s._$Co)||void 0===o?void 0:o[e]:s._$Cl;const u=d(i)?void 0:i._$litDirective$;return(null==r?void 0:r.constructor)!==u&&(null===(n=null==r?void 0:r._$AO)||void 0===n||n.call(r,!1),void 0===u?r=void 0:(r=new u(t),r._$AT(t,s,e)),void 0!==e?(null!==(l=(h=s)._$Co)&&void 0!==l?l:h._$Co=[])[e]=r:s._$Cl=r),void 0!==r&&(i=P(t,r._$AS(t,i.values),r,e)),i}class V{constructor(t,i){this.u=[],this._$AN=void 0,this._$AD=t,this._$AM=i}get parentNode(){return this._$AM.parentNode}get _$AU(){return this._$AM._$AU}v(t){var i;const{el:{content:s},parts:e}=this._$AD,o=(null!==(i=null==t?void 0:t.creationScope)&&void 0!==i?i:h).importNode(s,!0);A.currentNode=o;let n=A.nextNode(),l=0,r=0,d=e[0];for(;void 0!==d;){if(l===d.index){let i;2===d.type?i=new N(n,n.nextSibling,this,t):1===d.type?i=new d.ctor(n,d.name,d.strings,this,t):6===d.type&&(i=new I(n,this,t)),this.u.push(i),d=e[++r]}l!==(null==d?void 0:d.index)&&(n=A.nextNode(),l++)}return o}p(t){let i=0;for(const s of this.u)void 0!==s&&(void 0!==s.strings?(s._$AI(t,s,i),i+=s.strings.length-2):s._$AI(t[i])),i++}}class N{constructor(t,i,s,e){var o;this.type=2,this._$AH=b,this._$AN=void 0,this._$AA=t,this._$AB=i,this._$AM=s,this.options=e,this._$Cm=null===(o=null==e?void 0:e.isConnected)||void 0===o||o}get _$AU(){var t,i;return null!==(i=null===(t=this._$AM)||void 0===t?void 0:t._$AU)&&void 0!==i?i:this._$Cm}get parentNode(){let t=this._$AA.parentNode;const i=this._$AM;return void 0!==i&&11===t.nodeType&&(t=i.parentNode),t}get startNode(){return this._$AA}get endNode(){return this._$AB}_$AI(t,i=this){t=P(this,t,i),d(t)?t===b||null==t||""===t?(this._$AH!==b&&this._$AR(),this._$AH=b):t!==this._$AH&&t!==x&&this.g(t):void 0!==t._$litType$?this.$(t):void 0!==t.nodeType?this.T(t):c(t)?this.k(t):this.g(t)}O(t,i=this._$AB){return this._$AA.parentNode.insertBefore(t,i)}T(t){this._$AH!==t&&(this._$AR(),this._$AH=this.O(t))}g(t){this._$AH!==b&&d(this._$AH)?this._$AA.nextSibling.data=t:this.T(h.createTextNode(t)),this._$AH=t}$(t){var i;const{values:s,_$litType$:e}=t,o="number"==typeof e?this._$AC(t):(void 0===e.el&&(e.el=C.createElement(e.h,this.options)),e);if((null===(i=this._$AH)||void 0===i?void 0:i._$AD)===o)this._$AH.p(s);else{const t=new V(o,this),i=t.v(this.options);t.p(s),this.T(i),this._$AH=t}}_$AC(t){let i=T.get(t.strings);return void 0===i&&T.set(t.strings,i=new C(t)),i}k(t){u(this._$AH)||(this._$AH=[],this._$AR());const i=this._$AH;let s,e=0;for(const o of t)e===i.length?i.push(s=new N(this.O(r()),this.O(r()),this,this.options)):s=i[e],s._$AI(o),e++;e<i.length&&(this._$AR(s&&s._$AB.nextSibling,e),i.length=e)}_$AR(t=this._$AA.nextSibling,i){var s;for(null===(s=this._$AP)||void 0===s||s.call(this,!1,!0,i);t&&t!==this._$AB;){const i=t.nextSibling;t.remove(),t=i}}setConnected(t){var i;void 0===this._$AM&&(this._$Cm=t,null===(i=this._$AP)||void 0===i||i.call(this,t))}}class S{constructor(t,i,s,e,o){this.type=1,this._$AH=b,this._$AN=void 0,this.element=t,this.name=i,this._$AM=e,this.options=o,s.length>2||""!==s[0]||""!==s[1]?(this._$AH=Array(s.length-1).fill(new String),this.strings=s):this._$AH=b}get tagName(){return this.element.tagName}get _$AU(){return this._$AM._$AU}_$AI(t,i=this,s,e){const o=this.strings;let n=!1;if(void 0===o)t=P(this,t,i,0),n=!d(t)||t!==this._$AH&&t!==x,n&&(this._$AH=t);else{const e=t;let l,h;for(t=o[0],l=0;l<o.length-1;l++)h=P(this,e[s+l],i,l),h===x&&(h=this._$AH[l]),n||(n=!d(h)||h!==this._$AH[l]),h===b?t=b:t!==b&&(t+=(null!=h?h:"")+o[l+1]),this._$AH[l]=h}n&&!e&&this.j(t)}j(t){t===b?this.element.removeAttribute(this.name):this.element.setAttribute(this.name,null!=t?t:"")}}class M extends S{constructor(){super(...arguments),this.type=3}j(t){this.element[this.name]=t===b?void 0:t}}const R=s?s.emptyScript:"";class k extends S{constructor(){super(...arguments),this.type=4}j(t){t&&t!==b?this.element.setAttribute(this.name,R):this.element.removeAttribute(this.name)}}class H extends S{constructor(t,i,s,e,o){super(t,i,s,e,o),this.type=5}_$AI(t,i=this){var s;if((t=null!==(s=P(this,t,i,0))&&void 0!==s?s:b)===x)return;const e=this._$AH,o=t===b&&e!==b||t.capture!==e.capture||t.once!==e.once||t.passive!==e.passive,n=t!==b&&(e===b||o);o&&this.element.removeEventListener(this.name,this,e),n&&this.element.addEventListener(this.name,this,t),this._$AH=t}handleEvent(t){var i,s;"function"==typeof this._$AH?this._$AH.call(null!==(s=null===(i=this.options)||void 0===i?void 0:i.host)&&void 0!==s?s:this.element,t):this._$AH.handleEvent(t)}}class I{constructor(t,i,s){this.element=t,this.type=6,this._$AN=void 0,this._$AM=i,this.options=s}get _$AU(){return this._$AM._$AU}_$AI(t){P(this,t)}}const L={P:"$lit$",A:o,M:n,C:1,L:E,R:V,D:c,V:P,I:N,H:S,N:k,U:H,B:M,F:I},z=i.litHtmlPolyfillSupport;null==z||z(C,N),(null!==(t=i.litHtmlVersions)&&void 0!==t?t:i.litHtmlVersions=[]).push("2.5.0");const Z=(t,i,s)=>{var e,o;const n=null!==(e=null==s?void 0:s.renderBefore)&&void 0!==e?e:i;let l=n._$litPart$;if(void 0===l){const t=null!==(o=null==s?void 0:s.renderBefore)&&void 0!==o?o:null;n._$litPart$=l=new N(i.insertBefore(r(),t),t,void 0,null!=s?s:{})}return l._$AI(t),l};export{L as _$LH,y as html,x as noChange,b as nothing,Z as render,w as svg};
//# sourceMappingURL=lit-html.js.map

"###;

const LIT_HTML_DIRECTIVE_611 : &'static str = r###"
/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */
const t={ATTRIBUTE:1,CHILD:2,PROPERTY:3,BOOLEAN_ATTRIBUTE:4,EVENT:5,ELEMENT:6},e=t=>(...e)=>({_$litDirective$:t,values:e});class i{constructor(t){}get _$AU(){return this._$AM._$AU}_$AT(t,e,i){this._$Ct=t,this._$AM=e,this._$Ci=i}_$AS(t,e){return this.update(t,e)}update(t,e){return this.render(...e)}}export{i as Directive,t as PartType,e as directive};
//# sourceMappingURL=directive.js.map

"###;

const LIT_HTML_DIRECTIVE_HELPERS_546 : &'static str = r###"

/**
 * @license
 * Copyright 2020 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */const{I:l}=o,t=o=>null===o||"object"!=typeof o&&"function"!=typeof o,i={HTML:1,SVG:2},n=(o,l)=>void 0===l?void 0!==(null==o?void 0:o._$litType$):(null==o?void 0:o._$litType$)===l,d=o=>void 0!==(null==o?void 0:o._$litDirective$),v=o=>null==o?void 0:o._$litDirective$,e=o=>void 0===o.strings,c=()=>document.createComment(""),r=(o,t,i)=>{var n;const d=o._$AA.parentNode,v=void 0===t?o._$AB:t._$AA;if(void 0===i){const t=d.insertBefore(c(),v),n=d.insertBefore(c(),v);i=new l(t,n,o,o.options)}else{const l=i._$AB.nextSibling,t=i._$AM,e=t!==o;if(e){let l;null===(n=i._$AQ)||void 0===n||n.call(i,o),i._$AM=o,void 0!==i._$AP&&(l=o._$AU)!==t._$AU&&i._$AP(l)}if(l!==v||e){let o=i._$AA;for(;o!==l;){const l=o.nextSibling;d.insertBefore(o,v),o=l}}}return i},u=(o,l,t=o)=>(o._$AI(l,t),o),f={},s=(o,l=f)=>o._$AH=l,m=o=>o._$AH,p=o=>{var l;null===(l=o._$AP)||void 0===l||l.call(o,!1,!0);let t=o._$AA;const i=o._$AB.nextSibling;for(;t!==i;){const o=t.nextSibling;t.remove(),t=o}},a=o=>{o._$AR()};export{i as TemplateResultType,a as clearPart,m as getCommittedValue,v as getDirectiveClass,r as insertPart,d as isDirectiveResult,t as isPrimitive,e as isSingleExpression,n as isTemplateResult,p as removePart,u as setChildPartValue,s as setCommittedValue};
//# sourceMappingURL=directive-helpers.js.map

"###;

const LIT_HTML_DIRECTIVES_IF_DEFINED_501 : &'static str = r###"

/**
 * @license
 * Copyright 2018 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */const l=l=>null!=l?l:t;export{l as ifDefined};
//# sourceMappingURL=if-defined.js.map

"###;

const ASPECTRON_FLOW_UX_RESOURCES_EXTERNQRQR_265 : &'static str = r###"
let root = {};
//root = typeof self !== 'undefined' ? self : this

(function webpackUniversalModuleDefinition(root, factory) {
	if(typeof exports === 'object' && typeof module === 'object')
		module.exports = factory();
	else if(typeof define === 'function' && define.amd)
		define([], factory);
	else if(typeof exports === 'object')
		exports["jsQR"] = factory();
	else
		root["jsQR"] = factory();
})(root, function() {
return /******/ (function(modules) { // webpackBootstrap
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, {
/******/ 				configurable: false,
/******/ 				enumerable: true,
/******/ 				get: getter
/******/ 			});
/******/ 		}
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = 3);
/******/ })
/************************************************************************/
/******/ ([
/* 0 */
/***/ (function(module, exports, __webpack_require__) {

"use strict";

Object.defineProperty(exports, "__esModule", { value: true });
var BitMatrix = /** @class */ (function () {
    function BitMatrix(data, width) {
        this.width = width;
        this.height = data.length / width;
        this.data = data;
    }
    BitMatrix.createEmpty = function (width, height) {
        return new BitMatrix(new Uint8ClampedArray(width * height), width);
    };
    BitMatrix.prototype.get = function (x, y) {
        if (x < 0 || x >= this.width || y < 0 || y >= this.height) {
            return false;
        }
        return !!this.data[y * this.width + x];
    };
    BitMatrix.prototype.set = function (x, y, v) {
        this.data[y * this.width + x] = v ? 1 : 0;
    };
    BitMatrix.prototype.setRegion = function (left, top, width, height, v) {
        for (var y = top; y < top + height; y++) {
            for (var x = left; x < left + width; x++) {
                this.set(x, y, !!v);
            }
        }
    };
    return BitMatrix;
}());
exports.BitMatrix = BitMatrix;


/***/ }),
/* 1 */
/***/ (function(module, exports, __webpack_require__) {

"use strict";

Object.defineProperty(exports, "__esModule", { value: true });
var GenericGFPoly_1 = __webpack_require__(2);
function addOrSubtractGF(a, b) {
    return a ^ b; // tslint:disable-line:no-bitwise
}
exports.addOrSubtractGF = addOrSubtractGF;
var GenericGF = /** @class */ (function () {
    function GenericGF(primitive, size, genBase) {
        this.primitive = primitive;
        this.size = size;
        this.generatorBase = genBase;
        this.expTable = new Array(this.size);
        this.logTable = new Array(this.size);
        var x = 1;
        for (var i = 0; i < this.size; i++) {
            this.expTable[i] = x;
            x = x * 2;
            if (x >= this.size) {
                x = (x ^ this.primitive) & (this.size - 1); // tslint:disable-line:no-bitwise
            }
        }
        for (var i = 0; i < this.size - 1; i++) {
            this.logTable[this.expTable[i]] = i;
        }
        this.zero = new GenericGFPoly_1.default(this, Uint8ClampedArray.from([0]));
        this.one = new GenericGFPoly_1.default(this, Uint8ClampedArray.from([1]));
    }
    GenericGF.prototype.multiply = function (a, b) {
        if (a === 0 || b === 0) {
            return 0;
        }
        return this.expTable[(this.logTable[a] + this.logTable[b]) % (this.size - 1)];
    };
    GenericGF.prototype.inverse = function (a) {
        if (a === 0) {
            throw new Error("Can't invert 0");
        }
        return this.expTable[this.size - this.logTable[a] - 1];
    };
    GenericGF.prototype.buildMonomial = function (degree, coefficient) {
        if (degree < 0) {
            throw new Error("Invalid monomial degree less than 0");
        }
        if (coefficient === 0) {
            return this.zero;
        }
        var coefficients = new Uint8ClampedArray(degree + 1);
        coefficients[0] = coefficient;
        return new GenericGFPoly_1.default(this, coefficients);
    };
    GenericGF.prototype.log = function (a) {
        if (a === 0) {
            throw new Error("Can't take log(0)");
        }
        return this.logTable[a];
    };
    GenericGF.prototype.exp = function (a) {
        return this.expTable[a];
    };
    return GenericGF;
}());
exports.default = GenericGF;


/***/ }),
/* 2 */
/***/ (function(module, exports, __webpack_require__) {

"use strict";

Object.defineProperty(exports, "__esModule", { value: true });
var GenericGF_1 = __webpack_require__(1);
var GenericGFPoly = /** @class */ (function () {
    function GenericGFPoly(field, coefficients) {
        if (coefficients.length === 0) {
            throw new Error("No coefficients.");
        }
        this.field = field;
        var coefficientsLength = coefficients.length;
        if (coefficientsLength > 1 && coefficients[0] === 0) {
            // Leading term must be non-zero for anything except the constant polynomial "0"
            var firstNonZero = 1;
            while (firstNonZero < coefficientsLength && coefficients[firstNonZero] === 0) {
                firstNonZero++;
            }
            if (firstNonZero === coefficientsLength) {
                this.coefficients = field.zero.coefficients;
            }
            else {
                this.coefficients = new Uint8ClampedArray(coefficientsLength - firstNonZero);
                for (var i = 0; i < this.coefficients.length; i++) {
                    this.coefficients[i] = coefficients[firstNonZero + i];
                }
            }
        }
        else {
            this.coefficients = coefficients;
        }
    }
    GenericGFPoly.prototype.degree = function () {
        return this.coefficients.length - 1;
    };
    GenericGFPoly.prototype.isZero = function () {
        return this.coefficients[0] === 0;
    };
    GenericGFPoly.prototype.getCoefficient = function (degree) {
        return this.coefficients[this.coefficients.length - 1 - degree];
    };
    GenericGFPoly.prototype.addOrSubtract = function (other) {
        var _a;
        if (this.isZero()) {
            return other;
        }
        if (other.isZero()) {
            return this;
        }
        var smallerCoefficients = this.coefficients;
        var largerCoefficients = other.coefficients;
        if (smallerCoefficients.length > largerCoefficients.length) {
            _a = [largerCoefficients, smallerCoefficients], smallerCoefficients = _a[0], largerCoefficients = _a[1];
        }
        var sumDiff = new Uint8ClampedArray(largerCoefficients.length);
        var lengthDiff = largerCoefficients.length - smallerCoefficients.length;
        for (var i = 0; i < lengthDiff; i++) {
            sumDiff[i] = largerCoefficients[i];
        }
        for (var i = lengthDiff; i < largerCoefficients.length; i++) {
            sumDiff[i] = GenericGF_1.addOrSubtractGF(smallerCoefficients[i - lengthDiff], largerCoefficients[i]);
        }
        return new GenericGFPoly(this.field, sumDiff);
    };
    GenericGFPoly.prototype.multiply = function (scalar) {
        if (scalar === 0) {
            return this.field.zero;
        }
        if (scalar === 1) {
            return this;
        }
        var size = this.coefficients.length;
        var product = new Uint8ClampedArray(size);
        for (var i = 0; i < size; i++) {
            product[i] = this.field.multiply(this.coefficients[i], scalar);
        }
        return new GenericGFPoly(this.field, product);
    };
    GenericGFPoly.prototype.multiplyPoly = function (other) {
        if (this.isZero() || other.isZero()) {
            return this.field.zero;
        }
        var aCoefficients = this.coefficients;
        var aLength = aCoefficients.length;
        var bCoefficients = other.coefficients;
        var bLength = bCoefficients.length;
        var product = new Uint8ClampedArray(aLength + bLength - 1);
        for (var i = 0; i < aLength; i++) {
            var aCoeff = aCoefficients[i];
            for (var j = 0; j < bLength; j++) {
                product[i + j] = GenericGF_1.addOrSubtractGF(product[i + j], this.field.multiply(aCoeff, bCoefficients[j]));
            }
        }
        return new GenericGFPoly(this.field, product);
    };
    GenericGFPoly.prototype.multiplyByMonomial = function (degree, coefficient) {
        if (degree < 0) {
            throw new Error("Invalid degree less than 0");
        }
        if (coefficient === 0) {
            return this.field.zero;
        }
        var size = this.coefficients.length;
        var product = new Uint8ClampedArray(size + degree);
        for (var i = 0; i < size; i++) {
            product[i] = this.field.multiply(this.coefficients[i], coefficient);
        }
        return new GenericGFPoly(this.field, product);
    };
    GenericGFPoly.prototype.evaluateAt = function (a) {
        var result = 0;
        if (a === 0) {
            // Just return the x^0 coefficient
            return this.getCoefficient(0);
        }
        var size = this.coefficients.length;
        if (a === 1) {
            // Just the sum of the coefficients
            this.coefficients.forEach(function (coefficient) {
                result = GenericGF_1.addOrSubtractGF(result, coefficient);
            });
            return result;
        }
        result = this.coefficients[0];
        for (var i = 1; i < size; i++) {
            result = GenericGF_1.addOrSubtractGF(this.field.multiply(a, result), this.coefficients[i]);
        }
        return result;
    };
    return GenericGFPoly;
}());
exports.default = GenericGFPoly;


/***/ }),
/* 3 */
/***/ (function(module, exports, __webpack_require__) {

"use strict";

Object.defineProperty(exports, "__esModule", { value: true });
var binarizer_1 = __webpack_require__(4);
var decoder_1 = __webpack_require__(5);
var extractor_1 = __webpack_require__(11);
var locator_1 = __webpack_require__(12);
function scan(matrix) {
    var locations = locator_1.locate(matrix);
    if (!locations) {
        return null;
    }
    for (var _i = 0, locations_1 = locations; _i < locations_1.length; _i++) {
        var location_1 = locations_1[_i];
        var extracted = extractor_1.extract(matrix, location_1);
        var decoded = decoder_1.decode(extracted.matrix);
        if (decoded) {
            return {
                binaryData: decoded.bytes,
                data: decoded.text,
                chunks: decoded.chunks,
                location: {
                    topRightCorner: extracted.mappingFunction(location_1.dimension, 0),
                    topLeftCorner: extracted.mappingFunction(0, 0),
                    bottomRightCorner: extracted.mappingFunction(location_1.dimension, location_1.dimension),
                    bottomLeftCorner: extracted.mappingFunction(0, location_1.dimension),
                    topRightFinderPattern: location_1.topRight,
                    topLeftFinderPattern: location_1.topLeft,
                    bottomLeftFinderPattern: location_1.bottomLeft,
                    bottomRightAlignmentPattern: location_1.alignmentPattern,
                },
            };
        }
    }
    return null;
}
var defaultOptions = {
    inversionAttempts: "attemptBoth",
};
function jsQR(data, width, height, providedOptions) {
    if (providedOptions === void 0) { providedOptions = {}; }
    var options = defaultOptions;
    Object.keys(options || {}).forEach(function (opt) {
        options[opt] = providedOptions[opt] || options[opt];
    });
    var shouldInvert = options.inversionAttempts === "attemptBoth" || options.inversionAttempts === "invertFirst";
    var tryInvertedFirst = options.inversionAttempts === "onlyInvert" || options.inversionAttempts === "invertFirst";
    var _a = binarizer_1.binarize(data, width, height, shouldInvert), binarized = _a.binarized, inverted = _a.inverted;
    var result = scan(tryInvertedFirst ? inverted : binarized);
    if (!result && (options.inversionAttempts === "attemptBoth" || options.inversionAttempts === "invertFirst")) {
        result = scan(tryInvertedFirst ? binarized : inverted);
    }
    return result;
}
jsQR.default = jsQR;
exports.default = jsQR;


/***/ }),
/* 4 */
/***/ (function(module, exports, __webpack_require__) {

"use strict";

Object.defineProperty(exports, "__esModule", { value: true });
var BitMatrix_1 = __webpack_require__(0);
var REGION_SIZE = 8;
var MIN_DYNAMIC_RANGE = 24;
function numBetween(value, min, max) {
    return value < min ? min : value > max ? max : value;
}
// Like BitMatrix but accepts arbitry Uint8 values
var Matrix = /** @class */ (function () {
    function Matrix(width, height) {
        this.width = width;
        this.data = new Uint8ClampedArray(width * height);
    }
    Matrix.prototype.get = function (x, y) {
        return this.data[y * this.width + x];
    };
    Matrix.prototype.set = function (x, y, value) {
        this.data[y * this.width + x] = value;
    };
    return Matrix;
}());
function binarize(data, width, height, returnInverted) {
    if (data.length !== width * height * 4) {
        throw new Error("Malformed data passed to binarizer.");
    }
    // Convert image to greyscale
    var greyscalePixels = new Matrix(width, height);
    for (var x = 0; x < width; x++) {
        for (var y = 0; y < height; y++) {
            var r = data[((y * width + x) * 4) + 0];
            var g = data[((y * width + x) * 4) + 1];
            var b = data[((y * width + x) * 4) + 2];
            greyscalePixels.set(x, y, 0.2126 * r + 0.7152 * g + 0.0722 * b);
        }
    }
    var horizontalRegionCount = Math.ceil(width / REGION_SIZE);
    var verticalRegionCount = Math.ceil(height / REGION_SIZE);
    var blackPoints = new Matrix(horizontalRegionCount, verticalRegionCount);
    for (var verticalRegion = 0; verticalRegion < verticalRegionCount; verticalRegion++) {
        for (var hortizontalRegion = 0; hortizontalRegion < horizontalRegionCount; hortizontalRegion++) {
            var sum = 0;
            var min = Infinity;
            var max = 0;
            for (var y = 0; y < REGION_SIZE; y++) {
                for (var x = 0; x < REGION_SIZE; x++) {
                    var pixelLumosity = greyscalePixels.get(hortizontalRegion * REGION_SIZE + x, verticalRegion * REGION_SIZE + y);
                    sum += pixelLumosity;
                    min = Math.min(min, pixelLumosity);
                    max = Math.max(max, pixelLumosity);
                }
            }
            var average = sum / (Math.pow(REGION_SIZE, 2));
            if (max - min <= MIN_DYNAMIC_RANGE) {
                // If variation within the block is low, assume this is a block with only light or only
                // dark pixels. In that case we do not want to use the average, as it would divide this
                // low contrast area into black and white pixels, essentially creating data out of noise.
                //
                // Default the blackpoint for these blocks to be half the min - effectively white them out
                average = min / 2;
                if (verticalRegion > 0 && hortizontalRegion > 0) {
                    // Correct the "white background" assumption for blocks that have neighbors by comparing
                    // the pixels in this block to the previously calculated black points. This is based on
                    // the fact that dark barcode symbology is always surrounded by some amount of light
                    // background for which reasonable black point estimates were made. The bp estimated at
                    // the boundaries is used for the interior.
                    // The (min < bp) is arbitrary but works better than other heuristics that were tried.
                    var averageNeighborBlackPoint = (blackPoints.get(hortizontalRegion, verticalRegion - 1) +
                        (2 * blackPoints.get(hortizontalRegion - 1, verticalRegion)) +
                        blackPoints.get(hortizontalRegion - 1, verticalRegion - 1)) / 4;
                    if (min < averageNeighborBlackPoint) {
                        average = averageNeighborBlackPoint;
                    }
                }
            }
            blackPoints.set(hortizontalRegion, verticalRegion, average);
        }
    }
    var binarized = BitMatrix_1.BitMatrix.createEmpty(width, height);
    var inverted = null;
    if (returnInverted) {
        inverted = BitMatrix_1.BitMatrix.createEmpty(width, height);
    }
    for (var verticalRegion = 0; verticalRegion < verticalRegionCount; verticalRegion++) {
        for (var hortizontalRegion = 0; hortizontalRegion < horizontalRegionCount; hortizontalRegion++) {
            var left = numBetween(hortizontalRegion, 2, horizontalRegionCount - 3);
            var top_1 = numBetween(verticalRegion, 2, verticalRegionCount - 3);
            var sum = 0;
            for (var xRegion = -2; xRegion <= 2; xRegion++) {
                for (var yRegion = -2; yRegion <= 2; yRegion++) {
                    sum += blackPoints.get(left + xRegion, top_1 + yRegion);
                }
            }
            var threshold = sum / 25;
            for (var xRegion = 0; xRegion < REGION_SIZE; xRegion++) {
                for (var yRegion = 0; yRegion < REGION_SIZE; yRegion++) {
                    var x = hortizontalRegion * REGION_SIZE + xRegion;
                    var y = verticalRegion * REGION_SIZE + yRegion;
                    var lum = greyscalePixels.get(x, y);
                    binarized.set(x, y, lum <= threshold);
                    if (returnInverted) {
                        inverted.set(x, y, !(lum <= threshold));
                    }
                }
            }
        }
    }
    if (returnInverted) {
        return { binarized: binarized, inverted: inverted };
    }
    return { binarized: binarized };
}
exports.binarize = binarize;


/***/ }),
/* 5 */
/***/ (function(module, exports, __webpack_require__) {

"use strict";

Object.defineProperty(exports, "__esModule", { value: true });
var BitMatrix_1 = __webpack_require__(0);
var decodeData_1 = __webpack_require__(6);
var reedsolomon_1 = __webpack_require__(9);
var version_1 = __webpack_require__(10);
// tslint:disable:no-bitwise
function numBitsDiffering(x, y) {
    var z = x ^ y;
    var bitCount = 0;
    while (z) {
        bitCount++;
        z &= z - 1;
    }
    return bitCount;
}
function pushBit(bit, byte) {
    return (byte << 1) | bit;
}
// tslint:enable:no-bitwise
var FORMAT_INFO_TABLE = [
    { bits: 0x5412, formatInfo: { errorCorrectionLevel: 1, dataMask: 0 } },
    { bits: 0x5125, formatInfo: { errorCorrectionLevel: 1, dataMask: 1 } },
    { bits: 0x5E7C, formatInfo: { errorCorrectionLevel: 1, dataMask: 2 } },
    { bits: 0x5B4B, formatInfo: { errorCorrectionLevel: 1, dataMask: 3 } },
    { bits: 0x45F9, formatInfo: { errorCorrectionLevel: 1, dataMask: 4 } },
    { bits: 0x40CE, formatInfo: { errorCorrectionLevel: 1, dataMask: 5 } },
    { bits: 0x4F97, formatInfo: { errorCorrectionLevel: 1, dataMask: 6 } },
    { bits: 0x4AA0, formatInfo: { errorCorrectionLevel: 1, dataMask: 7 } },
    { bits: 0x77C4, formatInfo: { errorCorrectionLevel: 0, dataMask: 0 } },
    { bits: 0x72F3, formatInfo: { errorCorrectionLevel: 0, dataMask: 1 } },
    { bits: 0x7DAA, formatInfo: { errorCorrectionLevel: 0, dataMask: 2 } },
    { bits: 0x789D, formatInfo: { errorCorrectionLevel: 0, dataMask: 3 } },
    { bits: 0x662F, formatInfo: { errorCorrectionLevel: 0, dataMask: 4 } },
    { bits: 0x6318, formatInfo: { errorCorrectionLevel: 0, dataMask: 5 } },
    { bits: 0x6C41, formatInfo: { errorCorrectionLevel: 0, dataMask: 6 } },
    { bits: 0x6976, formatInfo: { errorCorrectionLevel: 0, dataMask: 7 } },
    { bits: 0x1689, formatInfo: { errorCorrectionLevel: 3, dataMask: 0 } },
    { bits: 0x13BE, formatInfo: { errorCorrectionLevel: 3, dataMask: 1 } },
    { bits: 0x1CE7, formatInfo: { errorCorrectionLevel: 3, dataMask: 2 } },
    { bits: 0x19D0, formatInfo: { errorCorrectionLevel: 3, dataMask: 3 } },
    { bits: 0x0762, formatInfo: { errorCorrectionLevel: 3, dataMask: 4 } },
    { bits: 0x0255, formatInfo: { errorCorrectionLevel: 3, dataMask: 5 } },
    { bits: 0x0D0C, formatInfo: { errorCorrectionLevel: 3, dataMask: 6 } },
    { bits: 0x083B, formatInfo: { errorCorrectionLevel: 3, dataMask: 7 } },
    { bits: 0x355F, formatInfo: { errorCorrectionLevel: 2, dataMask: 0 } },
    { bits: 0x3068, formatInfo: { errorCorrectionLevel: 2, dataMask: 1 } },
    { bits: 0x3F31, formatInfo: { errorCorrectionLevel: 2, dataMask: 2 } },
    { bits: 0x3A06, formatInfo: { errorCorrectionLevel: 2, dataMask: 3 } },
    { bits: 0x24B4, formatInfo: { errorCorrectionLevel: 2, dataMask: 4 } },
    { bits: 0x2183, formatInfo: { errorCorrectionLevel: 2, dataMask: 5 } },
    { bits: 0x2EDA, formatInfo: { errorCorrectionLevel: 2, dataMask: 6 } },
    { bits: 0x2BED, formatInfo: { errorCorrectionLevel: 2, dataMask: 7 } },
];
var DATA_MASKS = [
    function (p) { return ((p.y + p.x) % 2) === 0; },
    function (p) { return (p.y % 2) === 0; },
    function (p) { return p.x % 3 === 0; },
    function (p) { return (p.y + p.x) % 3 === 0; },
    function (p) { return (Math.floor(p.y / 2) + Math.floor(p.x / 3)) % 2 === 0; },
    function (p) { return ((p.x * p.y) % 2) + ((p.x * p.y) % 3) === 0; },
    function (p) { return ((((p.y * p.x) % 2) + (p.y * p.x) % 3) % 2) === 0; },
    function (p) { return ((((p.y + p.x) % 2) + (p.y * p.x) % 3) % 2) === 0; },
];
function buildFunctionPatternMask(version) {
    var dimension = 17 + 4 * version.versionNumber;
    var matrix = BitMatrix_1.BitMatrix.createEmpty(dimension, dimension);
    matrix.setRegion(0, 0, 9, 9, true); // Top left finder pattern + separator + format
    matrix.setRegion(dimension - 8, 0, 8, 9, true); // Top right finder pattern + separator + format
    matrix.setRegion(0, dimension - 8, 9, 8, true); // Bottom left finder pattern + separator + format
    // Alignment patterns
    for (var _i = 0, _a = version.alignmentPatternCenters; _i < _a.length; _i++) {
        var x = _a[_i];
        for (var _b = 0, _c = version.alignmentPatternCenters; _b < _c.length; _b++) {
            var y = _c[_b];
            if (!(x === 6 && y === 6 || x === 6 && y === dimension - 7 || x === dimension - 7 && y === 6)) {
                matrix.setRegion(x - 2, y - 2, 5, 5, true);
            }
        }
    }
    matrix.setRegion(6, 9, 1, dimension - 17, true); // Vertical timing pattern
    matrix.setRegion(9, 6, dimension - 17, 1, true); // Horizontal timing pattern
    if (version.versionNumber > 6) {
        matrix.setRegion(dimension - 11, 0, 3, 6, true); // Version info, top right
        matrix.setRegion(0, dimension - 11, 6, 3, true); // Version info, bottom left
    }
    return matrix;
}
function readCodewords(matrix, version, formatInfo) {
    var dataMask = DATA_MASKS[formatInfo.dataMask];
    var dimension = matrix.height;
    var functionPatternMask = buildFunctionPatternMask(version);
    var codewords = [];
    var currentByte = 0;
    var bitsRead = 0;
    // Read columns in pairs, from right to left
    var readingUp = true;
    for (var columnIndex = dimension - 1; columnIndex > 0; columnIndex -= 2) {
        if (columnIndex === 6) { // Skip whole column with vertical alignment pattern;
            columnIndex--;
        }
        for (var i = 0; i < dimension; i++) {
            var y = readingUp ? dimension - 1 - i : i;
            for (var columnOffset = 0; columnOffset < 2; columnOffset++) {
                var x = columnIndex - columnOffset;
                if (!functionPatternMask.get(x, y)) {
                    bitsRead++;
                    var bit = matrix.get(x, y);
                    if (dataMask({ y: y, x: x })) {
                        bit = !bit;
                    }
                    currentByte = pushBit(bit, currentByte);
                    if (bitsRead === 8) { // Whole bytes
                        codewords.push(currentByte);
                        bitsRead = 0;
                        currentByte = 0;
                    }
                }
            }
        }
        readingUp = !readingUp;
    }
    return codewords;
}
function readVersion(matrix) {
    var dimension = matrix.height;
    var provisionalVersion = Math.floor((dimension - 17) / 4);
    if (provisionalVersion <= 6) { // 6 and under dont have version info in the QR code
        return version_1.VERSIONS[provisionalVersion - 1];
    }
    var topRightVersionBits = 0;
    for (var y = 5; y >= 0; y--) {
        for (var x = dimension - 9; x >= dimension - 11; x--) {
            topRightVersionBits = pushBit(matrix.get(x, y), topRightVersionBits);
        }
    }
    var bottomLeftVersionBits = 0;
    for (var x = 5; x >= 0; x--) {
        for (var y = dimension - 9; y >= dimension - 11; y--) {
            bottomLeftVersionBits = pushBit(matrix.get(x, y), bottomLeftVersionBits);
        }
    }
    var bestDifference = Infinity;
    var bestVersion;
    for (var _i = 0, VERSIONS_1 = version_1.VERSIONS; _i < VERSIONS_1.length; _i++) {
        var version = VERSIONS_1[_i];
        if (version.infoBits === topRightVersionBits || version.infoBits === bottomLeftVersionBits) {
            return version;
        }
        var difference = numBitsDiffering(topRightVersionBits, version.infoBits);
        if (difference < bestDifference) {
            bestVersion = version;
            bestDifference = difference;
        }
        difference = numBitsDiffering(bottomLeftVersionBits, version.infoBits);
        if (difference < bestDifference) {
            bestVersion = version;
            bestDifference = difference;
        }
    }
    // We can tolerate up to 3 bits of error since no two version info codewords will
    // differ in less than 8 bits.
    if (bestDifference <= 3) {
        return bestVersion;
    }
}
function readFormatInformation(matrix) {
    var topLeftFormatInfoBits = 0;
    for (var x = 0; x <= 8; x++) {
        if (x !== 6) { // Skip timing pattern bit
            topLeftFormatInfoBits = pushBit(matrix.get(x, 8), topLeftFormatInfoBits);
        }
    }
    for (var y = 7; y >= 0; y--) {
        if (y !== 6) { // Skip timing pattern bit
            topLeftFormatInfoBits = pushBit(matrix.get(8, y), topLeftFormatInfoBits);
        }
    }
    var dimension = matrix.height;
    var topRightBottomRightFormatInfoBits = 0;
    for (var y = dimension - 1; y >= dimension - 7; y--) { // bottom left
        topRightBottomRightFormatInfoBits = pushBit(matrix.get(8, y), topRightBottomRightFormatInfoBits);
    }
    for (var x = dimension - 8; x < dimension; x++) { // top right
        topRightBottomRightFormatInfoBits = pushBit(matrix.get(x, 8), topRightBottomRightFormatInfoBits);
    }
    var bestDifference = Infinity;
    var bestFormatInfo = null;
    for (var _i = 0, FORMAT_INFO_TABLE_1 = FORMAT_INFO_TABLE; _i < FORMAT_INFO_TABLE_1.length; _i++) {
        var _a = FORMAT_INFO_TABLE_1[_i], bits = _a.bits, formatInfo = _a.formatInfo;
        if (bits === topLeftFormatInfoBits || bits === topRightBottomRightFormatInfoBits) {
            return formatInfo;
        }
        var difference = numBitsDiffering(topLeftFormatInfoBits, bits);
        if (difference < bestDifference) {
            bestFormatInfo = formatInfo;
            bestDifference = difference;
        }
        if (topLeftFormatInfoBits !== topRightBottomRightFormatInfoBits) { // also try the other option
            difference = numBitsDiffering(topRightBottomRightFormatInfoBits, bits);
            if (difference < bestDifference) {
                bestFormatInfo = formatInfo;
                bestDifference = difference;
            }
        }
    }
    // Hamming distance of the 32 masked codes is 7, by construction, so <= 3 bits differing means we found a match
    if (bestDifference <= 3) {
        return bestFormatInfo;
    }
    return null;
}
function getDataBlocks(codewords, version, ecLevel) {
    var ecInfo = version.errorCorrectionLevels[ecLevel];
    var dataBlocks = [];
    var totalCodewords = 0;
    ecInfo.ecBlocks.forEach(function (block) {
        for (var i = 0; i < block.numBlocks; i++) {
            dataBlocks.push({ numDataCodewords: block.dataCodewordsPerBlock, codewords: [] });
            totalCodewords += block.dataCodewordsPerBlock + ecInfo.ecCodewordsPerBlock;
        }
    });
    // In some cases the QR code will be malformed enough that we pull off more or less than we should.
    // If we pull off less there's nothing we can do.
    // If we pull off more we can safely truncate
    if (codewords.length < totalCodewords) {
        return null;
    }
    codewords = codewords.slice(0, totalCodewords);
    var shortBlockSize = ecInfo.ecBlocks[0].dataCodewordsPerBlock;
    // Pull codewords to fill the blocks up to the minimum size
    for (var i = 0; i < shortBlockSize; i++) {
        for (var _i = 0, dataBlocks_1 = dataBlocks; _i < dataBlocks_1.length; _i++) {
            var dataBlock = dataBlocks_1[_i];
            dataBlock.codewords.push(codewords.shift());
        }
    }
    // If there are any large blocks, pull codewords to fill the last element of those
    if (ecInfo.ecBlocks.length > 1) {
        var smallBlockCount = ecInfo.ecBlocks[0].numBlocks;
        var largeBlockCount = ecInfo.ecBlocks[1].numBlocks;
        for (var i = 0; i < largeBlockCount; i++) {
            dataBlocks[smallBlockCount + i].codewords.push(codewords.shift());
        }
    }
    // Add the rest of the codewords to the blocks. These are the error correction codewords.
    while (codewords.length > 0) {
        for (var _a = 0, dataBlocks_2 = dataBlocks; _a < dataBlocks_2.length; _a++) {
            var dataBlock = dataBlocks_2[_a];
            dataBlock.codewords.push(codewords.shift());
        }
    }
    return dataBlocks;
}
function decodeMatrix(matrix) {
    var version = readVersion(matrix);
    if (!version) {
        return null;
    }
    var formatInfo = readFormatInformation(matrix);
    if (!formatInfo) {
        return null;
    }
    var codewords = readCodewords(matrix, version, formatInfo);
    var dataBlocks = getDataBlocks(codewords, version, formatInfo.errorCorrectionLevel);
    if (!dataBlocks) {
        return null;
    }
    // Count total number of data bytes
    var totalBytes = dataBlocks.reduce(function (a, b) { return a + b.numDataCodewords; }, 0);
    var resultBytes = new Uint8ClampedArray(totalBytes);
    var resultIndex = 0;
    for (var _i = 0, dataBlocks_3 = dataBlocks; _i < dataBlocks_3.length; _i++) {
        var dataBlock = dataBlocks_3[_i];
        var correctedBytes = reedsolomon_1.decode(dataBlock.codewords, dataBlock.codewords.length - dataBlock.numDataCodewords);
        if (!correctedBytes) {
            return null;
        }
        for (var i = 0; i < dataBlock.numDataCodewords; i++) {
            resultBytes[resultIndex++] = correctedBytes[i];
        }
    }
    try {
        return decodeData_1.decode(resultBytes, version.versionNumber);
    }
    catch (_a) {
        return null;
    }
}
function decode(matrix) {
    if (matrix == null) {
        return null;
    }
    var result = decodeMatrix(matrix);
    if (result) {
        return result;
    }
    // Decoding didn't work, try mirroring the QR across the topLeft -> bottomRight line.
    for (var x = 0; x < matrix.width; x++) {
        for (var y = x + 1; y < matrix.height; y++) {
            if (matrix.get(x, y) !== matrix.get(y, x)) {
                matrix.set(x, y, !matrix.get(x, y));
                matrix.set(y, x, !matrix.get(y, x));
            }
        }
    }
    return decodeMatrix(matrix);
}
exports.decode = decode;


/***/ }),
/* 6 */
/***/ (function(module, exports, __webpack_require__) {

"use strict";

Object.defineProperty(exports, "__esModule", { value: true });
// tslint:disable:no-bitwise
var BitStream_1 = __webpack_require__(7);
var shiftJISTable_1 = __webpack_require__(8);
var Mode;
(function (Mode) {
    Mode["Numeric"] = "numeric";
    Mode["Alphanumeric"] = "alphanumeric";
    Mode["Byte"] = "byte";
    Mode["Kanji"] = "kanji";
    Mode["ECI"] = "eci";
})(Mode = exports.Mode || (exports.Mode = {}));
var ModeByte;
(function (ModeByte) {
    ModeByte[ModeByte["Terminator"] = 0] = "Terminator";
    ModeByte[ModeByte["Numeric"] = 1] = "Numeric";
    ModeByte[ModeByte["Alphanumeric"] = 2] = "Alphanumeric";
    ModeByte[ModeByte["Byte"] = 4] = "Byte";
    ModeByte[ModeByte["Kanji"] = 8] = "Kanji";
    ModeByte[ModeByte["ECI"] = 7] = "ECI";
    // StructuredAppend = 0x3,
    // FNC1FirstPosition = 0x5,
    // FNC1SecondPosition = 0x9,
})(ModeByte || (ModeByte = {}));
function decodeNumeric(stream, size) {
    var bytes = [];
    var text = "";
    var characterCountSize = [10, 12, 14][size];
    var length = stream.readBits(characterCountSize);
    // Read digits in groups of 3
    while (length >= 3) {
        var num = stream.readBits(10);
        if (num >= 1000) {
            throw new Error("Invalid numeric value above 999");
        }
        var a = Math.floor(num / 100);
        var b = Math.floor(num / 10) % 10;
        var c = num % 10;
        bytes.push(48 + a, 48 + b, 48 + c);
        text += a.toString() + b.toString() + c.toString();
        length -= 3;
    }
    // If the number of digits aren't a multiple of 3, the remaining digits are special cased.
    if (length === 2) {
        var num = stream.readBits(7);
        if (num >= 100) {
            throw new Error("Invalid numeric value above 99");
        }
        var a = Math.floor(num / 10);
        var b = num % 10;
        bytes.push(48 + a, 48 + b);
        text += a.toString() + b.toString();
    }
    else if (length === 1) {
        var num = stream.readBits(4);
        if (num >= 10) {
            throw new Error("Invalid numeric value above 9");
        }
        bytes.push(48 + num);
        text += num.toString();
    }
    return { bytes: bytes, text: text };
}
var AlphanumericCharacterCodes = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8",
    "9", "A", "B", "C", "D", "E", "F", "G", "H",
    "I", "J", "K", "L", "M", "N", "O", "P", "Q",
    "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
    " ", "$", "%", "*", "+", "-", ".", "/", ":",
];
function decodeAlphanumeric(stream, size) {
    var bytes = [];
    var text = "";
    var characterCountSize = [9, 11, 13][size];
    var length = stream.readBits(characterCountSize);
    while (length >= 2) {
        var v = stream.readBits(11);
        var a = Math.floor(v / 45);
        var b = v % 45;
        bytes.push(AlphanumericCharacterCodes[a].charCodeAt(0), AlphanumericCharacterCodes[b].charCodeAt(0));
        text += AlphanumericCharacterCodes[a] + AlphanumericCharacterCodes[b];
        length -= 2;
    }
    if (length === 1) {
        var a = stream.readBits(6);
        bytes.push(AlphanumericCharacterCodes[a].charCodeAt(0));
        text += AlphanumericCharacterCodes[a];
    }
    return { bytes: bytes, text: text };
}
function decodeByte(stream, size) {
    var bytes = [];
    var text = "";
    var characterCountSize = [8, 16, 16][size];
    var length = stream.readBits(characterCountSize);
    for (var i = 0; i < length; i++) {
        var b = stream.readBits(8);
        bytes.push(b);
    }
    try {
        text += decodeURIComponent(bytes.map(function (b) { return "%" + ("0" + b.toString(16)).substr(-2); }).join(""));
    }
    catch (_a) {
        // failed to decode
    }
    return { bytes: bytes, text: text };
}
function decodeKanji(stream, size) {
    var bytes = [];
    var text = "";
    var characterCountSize = [8, 10, 12][size];
    var length = stream.readBits(characterCountSize);
    for (var i = 0; i < length; i++) {
        var k = stream.readBits(13);
        var c = (Math.floor(k / 0xC0) << 8) | (k % 0xC0);
        if (c < 0x1F00) {
            c += 0x8140;
        }
        else {
            c += 0xC140;
        }
        bytes.push(c >> 8, c & 0xFF);
        text += String.fromCharCode(shiftJISTable_1.shiftJISTable[c]);
    }
    return { bytes: bytes, text: text };
}
function decode(data, version) {
    var _a, _b, _c, _d;
    var stream = new BitStream_1.BitStream(data);
    // There are 3 'sizes' based on the version. 1-9 is small (0), 10-26 is medium (1) and 27-40 is large (2).
    var size = version <= 9 ? 0 : version <= 26 ? 1 : 2;
    var result = {
        text: "",
        bytes: [],
        chunks: [],
    };
    while (stream.available() >= 4) {
        var mode = stream.readBits(4);
        if (mode === ModeByte.Terminator) {
            return result;
        }
        else if (mode === ModeByte.ECI) {
            if (stream.readBits(1) === 0) {
                result.chunks.push({
                    type: Mode.ECI,
                    assignmentNumber: stream.readBits(7),
                });
            }
            else if (stream.readBits(1) === 0) {
                result.chunks.push({
                    type: Mode.ECI,
                    assignmentNumber: stream.readBits(14),
                });
            }
            else if (stream.readBits(1) === 0) {
                result.chunks.push({
                    type: Mode.ECI,
                    assignmentNumber: stream.readBits(21),
                });
            }
            else {
                // ECI data seems corrupted
                result.chunks.push({
                    type: Mode.ECI,
                    assignmentNumber: -1,
                });
            }
        }
        else if (mode === ModeByte.Numeric) {
            var numericResult = decodeNumeric(stream, size);
            result.text += numericResult.text;
            (_a = result.bytes).push.apply(_a, numericResult.bytes);
            result.chunks.push({
                type: Mode.Numeric,
                text: numericResult.text,
            });
        }
        else if (mode === ModeByte.Alphanumeric) {
            var alphanumericResult = decodeAlphanumeric(stream, size);
            result.text += alphanumericResult.text;
            (_b = result.bytes).push.apply(_b, alphanumericResult.bytes);
            result.chunks.push({
                type: Mode.Alphanumeric,
                text: alphanumericResult.text,
            });
        }
        else if (mode === ModeByte.Byte) {
            var byteResult = decodeByte(stream, size);
            result.text += byteResult.text;
            (_c = result.bytes).push.apply(_c, byteResult.bytes);
            result.chunks.push({
                type: Mode.Byte,
                bytes: byteResult.bytes,
                text: byteResult.text,
            });
        }
        else if (mode === ModeByte.Kanji) {
            var kanjiResult = decodeKanji(stream, size);
            result.text += kanjiResult.text;
            (_d = result.bytes).push.apply(_d, kanjiResult.bytes);
            result.chunks.push({
                type: Mode.Kanji,
                bytes: kanjiResult.bytes,
                text: kanjiResult.text,
            });
        }
    }
    // If there is no data left, or the remaining bits are all 0, then that counts as a termination marker
    if (stream.available() === 0 || stream.readBits(stream.available()) === 0) {
        return result;
    }
}
exports.decode = decode;


/***/ }),
/* 7 */
/***/ (function(module, exports, __webpack_require__) {

"use strict";

// tslint:disable:no-bitwise
Object.defineProperty(exports, "__esModule", { value: true });
var BitStream = /** @class */ (function () {
    function BitStream(bytes) {
        this.byteOffset = 0;
        this.bitOffset = 0;
        this.bytes = bytes;
    }
    BitStream.prototype.readBits = function (numBits) {
        if (numBits < 1 || numBits > 32 || numBits > this.available()) {
            throw new Error("Cannot read " + numBits.toString() + " bits");
        }
        var result = 0;
        // First, read remainder from current byte
        if (this.bitOffset > 0) {
            var bitsLeft = 8 - this.bitOffset;
            var toRead = numBits < bitsLeft ? numBits : bitsLeft;
            var bitsToNotRead = bitsLeft - toRead;
            var mask = (0xFF >> (8 - toRead)) << bitsToNotRead;
            result = (this.bytes[this.byteOffset] & mask) >> bitsToNotRead;
            numBits -= toRead;
            this.bitOffset += toRead;
            if (this.bitOffset === 8) {
                this.bitOffset = 0;
                this.byteOffset++;
            }
        }
        // Next read whole bytes
        if (numBits > 0) {
            while (numBits >= 8) {
                result = (result << 8) | (this.bytes[this.byteOffset] & 0xFF);
                this.byteOffset++;
                numBits -= 8;
            }
            // Finally read a partial byte
            if (numBits > 0) {
                var bitsToNotRead = 8 - numBits;
                var mask = (0xFF >> bitsToNotRead) << bitsToNotRead;
                result = (result << numBits) | ((this.bytes[this.byteOffset] & mask) >> bitsToNotRead);
                this.bitOffset += numBits;
            }
        }
        return result;
    };
    BitStream.prototype.available = function () {
        return 8 * (this.bytes.length - this.byteOffset) - this.bitOffset;
    };
    return BitStream;
}());
exports.BitStream = BitStream;


/***/ }),
/* 8 */
/***/ (function(module, exports, __webpack_require__) {

"use strict";

Object.defineProperty(exports, "__esModule", { value: true });
exports.shiftJISTable = {
    0x20: 0x0020,
    0x21: 0x0021,
    0x22: 0x0022,
    0x23: 0x0023,
    0x24: 0x0024,
    0x25: 0x0025,
    0x26: 0x0026,
    0x27: 0x0027,
    0x28: 0x0028,
    0x29: 0x0029,
    0x2A: 0x002A,
    0x2B: 0x002B,
    0x2C: 0x002C,
    0x2D: 0x002D,
    0x2E: 0x002E,
    0x2F: 0x002F,
    0x30: 0x0030,
    0x31: 0x0031,
    0x32: 0x0032,
    0x33: 0x0033,
    0x34: 0x0034,
    0x35: 0x0035,
    0x36: 0x0036,
    0x37: 0x0037,
    0x38: 0x0038,
    0x39: 0x0039,
    0x3A: 0x003A,
    0x3B: 0x003B,
    0x3C: 0x003C,
    0x3D: 0x003D,
    0x3E: 0x003E,
    0x3F: 0x003F,
    0x40: 0x0040,
    0x41: 0x0041,
    0x42: 0x0042,
    0x43: 0x0043,
    0x44: 0x0044,
    0x45: 0x0045,
    0x46: 0x0046,
    0x47: 0x0047,
    0x48: 0x0048,
    0x49: 0x0049,
    0x4A: 0x004A,
    0x4B: 0x004B,
    0x4C: 0x004C,
    0x4D: 0x004D,
    0x4E: 0x004E,
    0x4F: 0x004F,
    0x50: 0x0050,
    0x51: 0x0051,
    0x52: 0x0052,
    0x53: 0x0053,
    0x54: 0x0054,
    0x55: 0x0055,
    0x56: 0x0056,
    0x57: 0x0057,
    0x58: 0x0058,
    0x59: 0x0059,
    0x5A: 0x005A,
    0x5B: 0x005B,
    0x5C: 0x00A5,
    0x5D: 0x005D,
    0x5E: 0x005E,
    0x5F: 0x005F,
    0x60: 0x0060,
    0x61: 0x0061,
    0x62: 0x0062,
    0x63: 0x0063,
    0x64: 0x0064,
    0x65: 0x0065,
    0x66: 0x0066,
    0x67: 0x0067,
    0x68: 0x0068,
    0x69: 0x0069,
    0x6A: 0x006A,
    0x6B: 0x006B,
    0x6C: 0x006C,
    0x6D: 0x006D,
    0x6E: 0x006E,
    0x6F: 0x006F,
    0x70: 0x0070,
    0x71: 0x0071,
    0x72: 0x0072,
    0x73: 0x0073,
    0x74: 0x0074,
    0x75: 0x0075,
    0x76: 0x0076,
    0x77: 0x0077,
    0x78: 0x0078,
    0x79: 0x0079,
    0x7A: 0x007A,
    0x7B: 0x007B,
    0x7C: 0x007C,
    0x7D: 0x007D,
    0x7E: 0x203E,
    0x8140: 0x3000,
    0x8141: 0x3001,
    0x8142: 0x3002,
    0x8143: 0xFF0C,
    0x8144: 0xFF0E,
    0x8145: 0x30FB,
    0x8146: 0xFF1A,
    0x8147: 0xFF1B,
    0x8148: 0xFF1F,
    0x8149: 0xFF01,
    0x814A: 0x309B,
    0x814B: 0x309C,
    0x814C: 0x00B4,
    0x814D: 0xFF40,
    0x814E: 0x00A8,
    0x814F: 0xFF3E,
    0x8150: 0xFFE3,
    0x8151: 0xFF3F,
    0x8152: 0x30FD,
    0x8153: 0x30FE,
    0x8154: 0x309D,
    0x8155: 0x309E,
    0x8156: 0x3003,
    0x8157: 0x4EDD,
    0x8158: 0x3005,
    0x8159: 0x3006,
    0x815A: 0x3007,
    0x815B: 0x30FC,
    0x815C: 0x2015,
    0x815D: 0x2010,
    0x815E: 0xFF0F,
    0x815F: 0x005C,
    0x8160: 0x301C,
    0x8161: 0x2016,
    0x8162: 0xFF5C,
    0x8163: 0x2026,
    0x8164: 0x2025,
    0x8165: 0x2018,
    0x8166: 0x2019,
    0x8167: 0x201C,
    0x8168: 0x201D,
    0x8169: 0xFF08,
    0x816A: 0xFF09,
    0x816B: 0x3014,
    0x816C: 0x3015,
    0x816D: 0xFF3B,
    0x816E: 0xFF3D,
    0x816F: 0xFF5B,
    0x8170: 0xFF5D,
    0x8171: 0x3008,
    0x8172: 0x3009,
    0x8173: 0x300A,
    0x8174: 0x300B,
    0x8175: 0x300C,
    0x8176: 0x300D,
    0x8177: 0x300E,
    0x8178: 0x300F,
    0x8179: 0x3010,
    0x817A: 0x3011,
    0x817B: 0xFF0B,
    0x817C: 0x2212,
    0x817D: 0x00B1,
    0x817E: 0x00D7,
    0x8180: 0x00F7,
    0x8181: 0xFF1D,
    0x8182: 0x2260,
    0x8183: 0xFF1C,
    0x8184: 0xFF1E,
    0x8185: 0x2266,
    0x8186: 0x2267,
    0x8187: 0x221E,
    0x8188: 0x2234,
    0x8189: 0x2642,
    0x818A: 0x2640,
    0x818B: 0x00B0,
    0x818C: 0x2032,
    0x818D: 0x2033,
    0x818E: 0x2103,
    0x818F: 0xFFE5,
    0x8190: 0xFF04,
    0x8191: 0x00A2,
    0x8192: 0x00A3,
    0x8193: 0xFF05,
    0x8194: 0xFF03,
    0x8195: 0xFF06,
    0x8196: 0xFF0A,
    0x8197: 0xFF20,
    0x8198: 0x00A7,
    0x8199: 0x2606,
    0x819A: 0x2605,
    0x819B: 0x25CB,
    0x819C: 0x25CF,
    0x819D: 0x25CE,
    0x819E: 0x25C7,
    0x819F: 0x25C6,
    0x81A0: 0x25A1,
    0x81A1: 0x25A0,
    0x81A2: 0x25B3,
    0x81A3: 0x25B2,
    0x81A4: 0x25BD,
    0x81A5: 0x25BC,
    0x81A6: 0x203B,
    0x81A7: 0x3012,
    0x81A8: 0x2192,
    0x81A9: 0x2190,
    0x81AA: 0x2191,
    0x81AB: 0x2193,
    0x81AC: 0x3013,
    0x81B8: 0x2208,
    0x81B9: 0x220B,
    0x81BA: 0x2286,
    0x81BB: 0x2287,
    0x81BC: 0x2282,
    0x81BD: 0x2283,
    0x81BE: 0x222A,
    0x81BF: 0x2229,
    0x81C8: 0x2227,
    0x81C9: 0x2228,
    0x81CA: 0x00AC,
    0x81CB: 0x21D2,
    0x81CC: 0x21D4,
    0x81CD: 0x2200,
    0x81CE: 0x2203,
    0x81DA: 0x2220,
    0x81DB: 0x22A5,
    0x81DC: 0x2312,
    0x81DD: 0x2202,
    0x81DE: 0x2207,
    0x81DF: 0x2261,
    0x81E0: 0x2252,
    0x81E1: 0x226A,
    0x81E2: 0x226B,
    0x81E3: 0x221A,
    0x81E4: 0x223D,
    0x81E5: 0x221D,
    0x81E6: 0x2235,
    0x81E7: 0x222B,
    0x81E8: 0x222C,
    0x81F0: 0x212B,
    0x81F1: 0x2030,
    0x81F2: 0x266F,
    0x81F3: 0x266D,
    0x81F4: 0x266A,
    0x81F5: 0x2020,
    0x81F6: 0x2021,
    0x81F7: 0x00B6,
    0x81FC: 0x25EF,
    0x824F: 0xFF10,
    0x8250: 0xFF11,
    0x8251: 0xFF12,
    0x8252: 0xFF13,
    0x8253: 0xFF14,
    0x8254: 0xFF15,
    0x8255: 0xFF16,
    0x8256: 0xFF17,
    0x8257: 0xFF18,
    0x8258: 0xFF19,
    0x8260: 0xFF21,
    0x8261: 0xFF22,
    0x8262: 0xFF23,
    0x8263: 0xFF24,
    0x8264: 0xFF25,
    0x8265: 0xFF26,
    0x8266: 0xFF27,
    0x8267: 0xFF28,
    0x8268: 0xFF29,
    0x8269: 0xFF2A,
    0x826A: 0xFF2B,
    0x826B: 0xFF2C,
    0x826C: 0xFF2D,
    0x826D: 0xFF2E,
    0x826E: 0xFF2F,
    0x826F: 0xFF30,
    0x8270: 0xFF31,
    0x8271: 0xFF32,
    0x8272: 0xFF33,
    0x8273: 0xFF34,
    0x8274: 0xFF35,
    0x8275: 0xFF36,
    0x8276: 0xFF37,
    0x8277: 0xFF38,
    0x8278: 0xFF39,
    0x8279: 0xFF3A,
    0x8281: 0xFF41,
    0x8282: 0xFF42,
    0x8283: 0xFF43,
    0x8284: 0xFF44,
    0x8285: 0xFF45,
    0x8286: 0xFF46,
    0x8287: 0xFF47,
    0x8288: 0xFF48,
    0x8289: 0xFF49,
    0x828A: 0xFF4A,
    0x828B: 0xFF4B,
    0x828C: 0xFF4C,
    0x828D: 0xFF4D,
    0x828E: 0xFF4E,
    0x828F: 0xFF4F,
    0x8290: 0xFF50,
    0x8291: 0xFF51,
    0x8292: 0xFF52,
    0x8293: 0xFF53,
    0x8294: 0xFF54,
    0x8295: 0xFF55,
    0x8296: 0xFF56,
    0x8297: 0xFF57,
    0x8298: 0xFF58,
    0x8299: 0xFF59,
    0x829A: 0xFF5A,
    0x829F: 0x3041,
    0x82A0: 0x3042,
    0x82A1: 0x3043,
    0x82A2: 0x3044,
    0x82A3: 0x3045,
    0x82A4: 0x3046,
    0x82A5: 0x3047,
    0x82A6: 0x3048,
    0x82A7: 0x3049,
    0x82A8: 0x304A,
    0x82A9: 0x304B,
    0x82AA: 0x304C,
    0x82AB: 0x304D,
    0x82AC: 0x304E,
    0x82AD: 0x304F,
    0x82AE: 0x3050,
    0x82AF: 0x3051,
    0x82B0: 0x3052,
    0x82B1: 0x3053,
    0x82B2: 0x3054,
    0x82B3: 0x3055,
    0x82B4: 0x3056,
    0x82B5: 0x3057,
    0x82B6: 0x3058,
    0x82B7: 0x3059,
    0x82B8: 0x305A,
    0x82B9: 0x305B,
    0x82BA: 0x305C,
    0x82BB: 0x305D,
    0x82BC: 0x305E,
    0x82BD: 0x305F,
    0x82BE: 0x3060,
    0x82BF: 0x3061,
    0x82C0: 0x3062,
    0x82C1: 0x3063,
    0x82C2: 0x3064,
    0x82C3: 0x3065,
    0x82C4: 0x3066,
    0x82C5: 0x3067,
    0x82C6: 0x3068,
    0x82C7: 0x3069,
    0x82C8: 0x306A,
    0x82C9: 0x306B,
    0x82CA: 0x306C,
    0x82CB: 0x306D,
    0x82CC: 0x306E,
    0x82CD: 0x306F,
    0x82CE: 0x3070,
    0x82CF: 0x3071,
    0x82D0: 0x3072,
    0x82D1: 0x3073,
    0x82D2: 0x3074,
    0x82D3: 0x3075,
    0x82D4: 0x3076,
    0x82D5: 0x3077,
    0x82D6: 0x3078,
    0x82D7: 0x3079,
    0x82D8: 0x307A,
    0x82D9: 0x307B,
    0x82DA: 0x307C,
    0x82DB: 0x307D,
    0x82DC: 0x307E,
    0x82DD: 0x307F,
    0x82DE: 0x3080,
    0x82DF: 0x3081,
    0x82E0: 0x3082,
    0x82E1: 0x3083,
    0x82E2: 0x3084,
    0x82E3: 0x3085,
    0x82E4: 0x3086,
    0x82E5: 0x3087,
    0x82E6: 0x3088,
    0x82E7: 0x3089,
    0x82E8: 0x308A,
    0x82E9: 0x308B,
    0x82EA: 0x308C,
    0x82EB: 0x308D,
    0x82EC: 0x308E,
    0x82ED: 0x308F,
    0x82EE: 0x3090,
    0x82EF: 0x3091,
    0x82F0: 0x3092,
    0x82F1: 0x3093,
    0x8340: 0x30A1,
    0x8341: 0x30A2,
    0x8342: 0x30A3,
    0x8343: 0x30A4,
    0x8344: 0x30A5,
    0x8345: 0x30A6,
    0x8346: 0x30A7,
    0x8347: 0x30A8,
    0x8348: 0x30A9,
    0x8349: 0x30AA,
    0x834A: 0x30AB,
    0x834B: 0x30AC,
    0x834C: 0x30AD,
    0x834D: 0x30AE,
    0x834E: 0x30AF,
    0x834F: 0x30B0,
    0x8350: 0x30B1,
    0x8351: 0x30B2,
    0x8352: 0x30B3,
    0x8353: 0x30B4,
    0x8354: 0x30B5,
    0x8355: 0x30B6,
    0x8356: 0x30B7,
    0x8357: 0x30B8,
    0x8358: 0x30B9,
    0x8359: 0x30BA,
    0x835A: 0x30BB,
    0x835B: 0x30BC,
    0x835C: 0x30BD,
    0x835D: 0x30BE,
    0x835E: 0x30BF,
    0x835F: 0x30C0,
    0x8360: 0x30C1,
    0x8361: 0x30C2,
    0x8362: 0x30C3,
    0x8363: 0x30C4,
    0x8364: 0x30C5,
    0x8365: 0x30C6,
    0x8366: 0x30C7,
    0x8367: 0x30C8,
    0x8368: 0x30C9,
    0x8369: 0x30CA,
    0x836A: 0x30CB,
    0x836B: 0x30CC,
    0x836C: 0x30CD,
    0x836D: 0x30CE,
    0x836E: 0x30CF,
    0x836F: 0x30D0,
    0x8370: 0x30D1,
    0x8371: 0x30D2,
    0x8372: 0x30D3,
    0x8373: 0x30D4,
    0x8374: 0x30D5,
    0x8375: 0x30D6,
    0x8376: 0x30D7,
    0x8377: 0x30D8,
    0x8378: 0x30D9,
    0x8379: 0x30DA,
    0x837A: 0x30DB,
    0x837B: 0x30DC,
    0x837C: 0x30DD,
    0x837D: 0x30DE,
    0x837E: 0x30DF,
    0x8380: 0x30E0,
    0x8381: 0x30E1,
    0x8382: 0x30E2,
    0x8383: 0x30E3,
    0x8384: 0x30E4,
    0x8385: 0x30E5,
    0x8386: 0x30E6,
    0x8387: 0x30E7,
    0x8388: 0x30E8,
    0x8389: 0x30E9,
    0x838A: 0x30EA,
    0x838B: 0x30EB,
    0x838C: 0x30EC,
    0x838D: 0x30ED,
    0x838E: 0x30EE,
    0x838F: 0x30EF,
    0x8390: 0x30F0,
    0x8391: 0x30F1,
    0x8392: 0x30F2,
    0x8393: 0x30F3,
    0x8394: 0x30F4,
    0x8395: 0x30F5,
    0x8396: 0x30F6,
    0x839F: 0x0391,
    0x83A0: 0x0392,
    0x83A1: 0x0393,
    0x83A2: 0x0394,
    0x83A3: 0x0395,
    0x83A4: 0x0396,
    0x83A5: 0x0397,
    0x83A6: 0x0398,
    0x83A7: 0x0399,
    0x83A8: 0x039A,
    0x83A9: 0x039B,
    0x83AA: 0x039C,
    0x83AB: 0x039D,
    0x83AC: 0x039E,
    0x83AD: 0x039F,
    0x83AE: 0x03A0,
    0x83AF: 0x03A1,
    0x83B0: 0x03A3,
    0x83B1: 0x03A4,
    0x83B2: 0x03A5,
    0x83B3: 0x03A6,
    0x83B4: 0x03A7,
    0x83B5: 0x03A8,
    0x83B6: 0x03A9,
    0x83BF: 0x03B1,
    0x83C0: 0x03B2,
    0x83C1: 0x03B3,
    0x83C2: 0x03B4,
    0x83C3: 0x03B5,
    0x83C4: 0x03B6,
    0x83C5: 0x03B7,
    0x83C6: 0x03B8,
    0x83C7: 0x03B9,
    0x83C8: 0x03BA,
    0x83C9: 0x03BB,
    0x83CA: 0x03BC,
    0x83CB: 0x03BD,
    0x83CC: 0x03BE,
    0x83CD: 0x03BF,
    0x83CE: 0x03C0,
    0x83CF: 0x03C1,
    0x83D0: 0x03C3,
    0x83D1: 0x03C4,
    0x83D2: 0x03C5,
    0x83D3: 0x03C6,
    0x83D4: 0x03C7,
    0x83D5: 0x03C8,
    0x83D6: 0x03C9,
    0x8440: 0x0410,
    0x8441: 0x0411,
    0x8442: 0x0412,
    0x8443: 0x0413,
    0x8444: 0x0414,
    0x8445: 0x0415,
    0x8446: 0x0401,
    0x8447: 0x0416,
    0x8448: 0x0417,
    0x8449: 0x0418,
    0x844A: 0x0419,
    0x844B: 0x041A,
    0x844C: 0x041B,
    0x844D: 0x041C,
    0x844E: 0x041D,
    0x844F: 0x041E,
    0x8450: 0x041F,
    0x8451: 0x0420,
    0x8452: 0x0421,
    0x8453: 0x0422,
    0x8454: 0x0423,
    0x8455: 0x0424,
    0x8456: 0x0425,
    0x8457: 0x0426,
    0x8458: 0x0427,
    0x8459: 0x0428,
    0x845A: 0x0429,
    0x845B: 0x042A,
    0x845C: 0x042B,
    0x845D: 0x042C,
    0x845E: 0x042D,
    0x845F: 0x042E,
    0x8460: 0x042F,
    0x8470: 0x0430,
    0x8471: 0x0431,
    0x8472: 0x0432,
    0x8473: 0x0433,
    0x8474: 0x0434,
    0x8475: 0x0435,
    0x8476: 0x0451,
    0x8477: 0x0436,
    0x8478: 0x0437,
    0x8479: 0x0438,
    0x847A: 0x0439,
    0x847B: 0x043A,
    0x847C: 0x043B,
    0x847D: 0x043C,
    0x847E: 0x043D,
    0x8480: 0x043E,
    0x8481: 0x043F,
    0x8482: 0x0440,
    0x8483: 0x0441,
    0x8484: 0x0442,
    0x8485: 0x0443,
    0x8486: 0x0444,
    0x8487: 0x0445,
    0x8488: 0x0446,
    0x8489: 0x0447,
    0x848A: 0x0448,
    0x848B: 0x0449,
    0x848C: 0x044A,
    0x848D: 0x044B,
    0x848E: 0x044C,
    0x848F: 0x044D,
    0x8490: 0x044E,
    0x8491: 0x044F,
    0x849F: 0x2500,
    0x84A0: 0x2502,
    0x84A1: 0x250C,
    0x84A2: 0x2510,
    0x84A3: 0x2518,
    0x84A4: 0x2514,
    0x84A5: 0x251C,
    0x84A6: 0x252C,
    0x84A7: 0x2524,
    0x84A8: 0x2534,
    0x84A9: 0x253C,
    0x84AA: 0x2501,
    0x84AB: 0x2503,
    0x84AC: 0x250F,
    0x84AD: 0x2513,
    0x84AE: 0x251B,
    0x84AF: 0x2517,
    0x84B0: 0x2523,
    0x84B1: 0x2533,
    0x84B2: 0x252B,
    0x84B3: 0x253B,
    0x84B4: 0x254B,
    0x84B5: 0x2520,
    0x84B6: 0x252F,
    0x84B7: 0x2528,
    0x84B8: 0x2537,
    0x84B9: 0x253F,
    0x84BA: 0x251D,
    0x84BB: 0x2530,
    0x84BC: 0x2525,
    0x84BD: 0x2538,
    0x84BE: 0x2542,
    0x889F: 0x4E9C,
    0x88A0: 0x5516,
    0x88A1: 0x5A03,
    0x88A2: 0x963F,
    0x88A3: 0x54C0,
    0x88A4: 0x611B,
    0x88A5: 0x6328,
    0x88A6: 0x59F6,
    0x88A7: 0x9022,
    0x88A8: 0x8475,
    0x88A9: 0x831C,
    0x88AA: 0x7A50,
    0x88AB: 0x60AA,
    0x88AC: 0x63E1,
    0x88AD: 0x6E25,
    0x88AE: 0x65ED,
    0x88AF: 0x8466,
    0x88B0: 0x82A6,
    0x88B1: 0x9BF5,
    0x88B2: 0x6893,
    0x88B3: 0x5727,
    0x88B4: 0x65A1,
    0x88B5: 0x6271,
    0x88B6: 0x5B9B,
    0x88B7: 0x59D0,
    0x88B8: 0x867B,
    0x88B9: 0x98F4,
    0x88BA: 0x7D62,
    0x88BB: 0x7DBE,
    0x88BC: 0x9B8E,
    0x88BD: 0x6216,
    0x88BE: 0x7C9F,
    0x88BF: 0x88B7,
    0x88C0: 0x5B89,
    0x88C1: 0x5EB5,
    0x88C2: 0x6309,
    0x88C3: 0x6697,
    0x88C4: 0x6848,
    0x88C5: 0x95C7,
    0x88C6: 0x978D,
    0x88C7: 0x674F,
    0x88C8: 0x4EE5,
    0x88C9: 0x4F0A,
    0x88CA: 0x4F4D,
    0x88CB: 0x4F9D,
    0x88CC: 0x5049,
    0x88CD: 0x56F2,
    0x88CE: 0x5937,
    0x88CF: 0x59D4,
    0x88D0: 0x5A01,
    0x88D1: 0x5C09,
    0x88D2: 0x60DF,
    0x88D3: 0x610F,
    0x88D4: 0x6170,
    0x88D5: 0x6613,
    0x88D6: 0x6905,
    0x88D7: 0x70BA,
    0x88D8: 0x754F,
    0x88D9: 0x7570,
    0x88DA: 0x79FB,
    0x88DB: 0x7DAD,
    0x88DC: 0x7DEF,
    0x88DD: 0x80C3,
    0x88DE: 0x840E,
    0x88DF: 0x8863,
    0x88E0: 0x8B02,
    0x88E1: 0x9055,
    0x88E2: 0x907A,
    0x88E3: 0x533B,
    0x88E4: 0x4E95,
    0x88E5: 0x4EA5,
    0x88E6: 0x57DF,
    0x88E7: 0x80B2,
    0x88E8: 0x90C1,
    0x88E9: 0x78EF,
    0x88EA: 0x4E00,
    0x88EB: 0x58F1,
    0x88EC: 0x6EA2,
    0x88ED: 0x9038,
    0x88EE: 0x7A32,
    0x88EF: 0x8328,
    0x88F0: 0x828B,
    0x88F1: 0x9C2F,
    0x88F2: 0x5141,
    0x88F3: 0x5370,
    0x88F4: 0x54BD,
    0x88F5: 0x54E1,
    0x88F6: 0x56E0,
    0x88F7: 0x59FB,
    0x88F8: 0x5F15,
    0x88F9: 0x98F2,
    0x88FA: 0x6DEB,
    0x88FB: 0x80E4,
    0x88FC: 0x852D,
    0x8940: 0x9662,
    0x8941: 0x9670,
    0x8942: 0x96A0,
    0x8943: 0x97FB,
    0x8944: 0x540B,
    0x8945: 0x53F3,
    0x8946: 0x5B87,
    0x8947: 0x70CF,
    0x8948: 0x7FBD,
    0x8949: 0x8FC2,
    0x894A: 0x96E8,
    0x894B: 0x536F,
    0x894C: 0x9D5C,
    0x894D: 0x7ABA,
    0x894E: 0x4E11,
    0x894F: 0x7893,
    0x8950: 0x81FC,
    0x8951: 0x6E26,
    0x8952: 0x5618,
    0x8953: 0x5504,
    0x8954: 0x6B1D,
    0x8955: 0x851A,
    0x8956: 0x9C3B,
    0x8957: 0x59E5,
    0x8958: 0x53A9,
    0x8959: 0x6D66,
    0x895A: 0x74DC,
    0x895B: 0x958F,
    0x895C: 0x5642,
    0x895D: 0x4E91,
    0x895E: 0x904B,
    0x895F: 0x96F2,
    0x8960: 0x834F,
    0x8961: 0x990C,
    0x8962: 0x53E1,
    0x8963: 0x55B6,
    0x8964: 0x5B30,
    0x8965: 0x5F71,
    0x8966: 0x6620,
    0x8967: 0x66F3,
    0x8968: 0x6804,
    0x8969: 0x6C38,
    0x896A: 0x6CF3,
    0x896B: 0x6D29,
    0x896C: 0x745B,
    0x896D: 0x76C8,
    0x896E: 0x7A4E,
    0x896F: 0x9834,
    0x8970: 0x82F1,
    0x8971: 0x885B,
    0x8972: 0x8A60,
    0x8973: 0x92ED,
    0x8974: 0x6DB2,
    0x8975: 0x75AB,
    0x8976: 0x76CA,
    0x8977: 0x99C5,
    0x8978: 0x60A6,
    0x8979: 0x8B01,
    0x897A: 0x8D8A,
    0x897B: 0x95B2,
    0x897C: 0x698E,
    0x897D: 0x53AD,
    0x897E: 0x5186,
    0x8980: 0x5712,
    0x8981: 0x5830,
    0x8982: 0x5944,
    0x8983: 0x5BB4,
    0x8984: 0x5EF6,
    0x8985: 0x6028,
    0x8986: 0x63A9,
    0x8987: 0x63F4,
    0x8988: 0x6CBF,
    0x8989: 0x6F14,
    0x898A: 0x708E,
    0x898B: 0x7114,
    0x898C: 0x7159,
    0x898D: 0x71D5,
    0x898E: 0x733F,
    0x898F: 0x7E01,
    0x8990: 0x8276,
    0x8991: 0x82D1,
    0x8992: 0x8597,
    0x8993: 0x9060,
    0x8994: 0x925B,
    0x8995: 0x9D1B,
    0x8996: 0x5869,
    0x8997: 0x65BC,
    0x8998: 0x6C5A,
    0x8999: 0x7525,
    0x899A: 0x51F9,
    0x899B: 0x592E,
    0x899C: 0x5965,
    0x899D: 0x5F80,
    0x899E: 0x5FDC,
    0x899F: 0x62BC,
    0x89A0: 0x65FA,
    0x89A1: 0x6A2A,
    0x89A2: 0x6B27,
    0x89A3: 0x6BB4,
    0x89A4: 0x738B,
    0x89A5: 0x7FC1,
    0x89A6: 0x8956,
    0x89A7: 0x9D2C,
    0x89A8: 0x9D0E,
    0x89A9: 0x9EC4,
    0x89AA: 0x5CA1,
    0x89AB: 0x6C96,
    0x89AC: 0x837B,
    0x89AD: 0x5104,
    0x89AE: 0x5C4B,
    0x89AF: 0x61B6,
    0x89B0: 0x81C6,
    0x89B1: 0x6876,
    0x89B2: 0x7261,
    0x89B3: 0x4E59,
    0x89B4: 0x4FFA,
    0x89B5: 0x5378,
    0x89B6: 0x6069,
    0x89B7: 0x6E29,
    0x89B8: 0x7A4F,
    0x89B9: 0x97F3,
    0x89BA: 0x4E0B,
    0x89BB: 0x5316,
    0x89BC: 0x4EEE,
    0x89BD: 0x4F55,
    0x89BE: 0x4F3D,
    0x89BF: 0x4FA1,
    0x89C0: 0x4F73,
    0x89C1: 0x52A0,
    0x89C2: 0x53EF,
    0x89C3: 0x5609,
    0x89C4: 0x590F,
    0x89C5: 0x5AC1,
    0x89C6: 0x5BB6,
    0x89C7: 0x5BE1,
    0x89C8: 0x79D1,
    0x89C9: 0x6687,
    0x89CA: 0x679C,
    0x89CB: 0x67B6,
    0x89CC: 0x6B4C,
    0x89CD: 0x6CB3,
    0x89CE: 0x706B,
    0x89CF: 0x73C2,
    0x89D0: 0x798D,
    0x89D1: 0x79BE,
    0x89D2: 0x7A3C,
    0x89D3: 0x7B87,
    0x89D4: 0x82B1,
    0x89D5: 0x82DB,
    0x89D6: 0x8304,
    0x89D7: 0x8377,
    0x89D8: 0x83EF,
    0x89D9: 0x83D3,
    0x89DA: 0x8766,
    0x89DB: 0x8AB2,
    0x89DC: 0x5629,
    0x89DD: 0x8CA8,
    0x89DE: 0x8FE6,
    0x89DF: 0x904E,
    0x89E0: 0x971E,
    0x89E1: 0x868A,
    0x89E2: 0x4FC4,
    0x89E3: 0x5CE8,
    0x89E4: 0x6211,
    0x89E5: 0x7259,
    0x89E6: 0x753B,
    0x89E7: 0x81E5,
    0x89E8: 0x82BD,
    0x89E9: 0x86FE,
    0x89EA: 0x8CC0,
    0x89EB: 0x96C5,
    0x89EC: 0x9913,
    0x89ED: 0x99D5,
    0x89EE: 0x4ECB,
    0x89EF: 0x4F1A,
    0x89F0: 0x89E3,
    0x89F1: 0x56DE,
    0x89F2: 0x584A,
    0x89F3: 0x58CA,
    0x89F4: 0x5EFB,
    0x89F5: 0x5FEB,
    0x89F6: 0x602A,
    0x89F7: 0x6094,
    0x89F8: 0x6062,
    0x89F9: 0x61D0,
    0x89FA: 0x6212,
    0x89FB: 0x62D0,
    0x89FC: 0x6539,
    0x8A40: 0x9B41,
    0x8A41: 0x6666,
    0x8A42: 0x68B0,
    0x8A43: 0x6D77,
    0x8A44: 0x7070,
    0x8A45: 0x754C,
    0x8A46: 0x7686,
    0x8A47: 0x7D75,
    0x8A48: 0x82A5,
    0x8A49: 0x87F9,
    0x8A4A: 0x958B,
    0x8A4B: 0x968E,
    0x8A4C: 0x8C9D,
    0x8A4D: 0x51F1,
    0x8A4E: 0x52BE,
    0x8A4F: 0x5916,
    0x8A50: 0x54B3,
    0x8A51: 0x5BB3,
    0x8A52: 0x5D16,
    0x8A53: 0x6168,
    0x8A54: 0x6982,
    0x8A55: 0x6DAF,
    0x8A56: 0x788D,
    0x8A57: 0x84CB,
    0x8A58: 0x8857,
    0x8A59: 0x8A72,
    0x8A5A: 0x93A7,
    0x8A5B: 0x9AB8,
    0x8A5C: 0x6D6C,
    0x8A5D: 0x99A8,
    0x8A5E: 0x86D9,
    0x8A5F: 0x57A3,
    0x8A60: 0x67FF,
    0x8A61: 0x86CE,
    0x8A62: 0x920E,
    0x8A63: 0x5283,
    0x8A64: 0x5687,
    0x8A65: 0x5404,
    0x8A66: 0x5ED3,
    0x8A67: 0x62E1,
    0x8A68: 0x64B9,
    0x8A69: 0x683C,
    0x8A6A: 0x6838,
    0x8A6B: 0x6BBB,
    0x8A6C: 0x7372,
    0x8A6D: 0x78BA,
    0x8A6E: 0x7A6B,
    0x8A6F: 0x899A,
    0x8A70: 0x89D2,
    0x8A71: 0x8D6B,
    0x8A72: 0x8F03,
    0x8A73: 0x90ED,
    0x8A74: 0x95A3,
    0x8A75: 0x9694,
    0x8A76: 0x9769,
    0x8A77: 0x5B66,
    0x8A78: 0x5CB3,
    0x8A79: 0x697D,
    0x8A7A: 0x984D,
    0x8A7B: 0x984E,
    0x8A7C: 0x639B,
    0x8A7D: 0x7B20,
    0x8A7E: 0x6A2B,
    0x8A80: 0x6A7F,
    0x8A81: 0x68B6,
    0x8A82: 0x9C0D,
    0x8A83: 0x6F5F,
    0x8A84: 0x5272,
    0x8A85: 0x559D,
    0x8A86: 0x6070,
    0x8A87: 0x62EC,
    0x8A88: 0x6D3B,
    0x8A89: 0x6E07,
    0x8A8A: 0x6ED1,
    0x8A8B: 0x845B,
    0x8A8C: 0x8910,
    0x8A8D: 0x8F44,
    0x8A8E: 0x4E14,
    0x8A8F: 0x9C39,
    0x8A90: 0x53F6,
    0x8A91: 0x691B,
    0x8A92: 0x6A3A,
    0x8A93: 0x9784,
    0x8A94: 0x682A,
    0x8A95: 0x515C,
    0x8A96: 0x7AC3,
    0x8A97: 0x84B2,
    0x8A98: 0x91DC,
    0x8A99: 0x938C,
    0x8A9A: 0x565B,
    0x8A9B: 0x9D28,
    0x8A9C: 0x6822,
    0x8A9D: 0x8305,
    0x8A9E: 0x8431,
    0x8A9F: 0x7CA5,
    0x8AA0: 0x5208,
    0x8AA1: 0x82C5,
    0x8AA2: 0x74E6,
    0x8AA3: 0x4E7E,
    0x8AA4: 0x4F83,
    0x8AA5: 0x51A0,
    0x8AA6: 0x5BD2,
    0x8AA7: 0x520A,
    0x8AA8: 0x52D8,
    0x8AA9: 0x52E7,
    0x8AAA: 0x5DFB,
    0x8AAB: 0x559A,
    0x8AAC: 0x582A,
    0x8AAD: 0x59E6,
    0x8AAE: 0x5B8C,
    0x8AAF: 0x5B98,
    0x8AB0: 0x5BDB,
    0x8AB1: 0x5E72,
    0x8AB2: 0x5E79,
    0x8AB3: 0x60A3,
    0x8AB4: 0x611F,
    0x8AB5: 0x6163,
    0x8AB6: 0x61BE,
    0x8AB7: 0x63DB,
    0x8AB8: 0x6562,
    0x8AB9: 0x67D1,
    0x8ABA: 0x6853,
    0x8ABB: 0x68FA,
    0x8ABC: 0x6B3E,
    0x8ABD: 0x6B53,
    0x8ABE: 0x6C57,
    0x8ABF: 0x6F22,
    0x8AC0: 0x6F97,
    0x8AC1: 0x6F45,
    0x8AC2: 0x74B0,
    0x8AC3: 0x7518,
    0x8AC4: 0x76E3,
    0x8AC5: 0x770B,
    0x8AC6: 0x7AFF,
    0x8AC7: 0x7BA1,
    0x8AC8: 0x7C21,
    0x8AC9: 0x7DE9,
    0x8ACA: 0x7F36,
    0x8ACB: 0x7FF0,
    0x8ACC: 0x809D,
    0x8ACD: 0x8266,
    0x8ACE: 0x839E,
    0x8ACF: 0x89B3,
    0x8AD0: 0x8ACC,
    0x8AD1: 0x8CAB,
    0x8AD2: 0x9084,
    0x8AD3: 0x9451,
    0x8AD4: 0x9593,
    0x8AD5: 0x9591,
    0x8AD6: 0x95A2,
    0x8AD7: 0x9665,
    0x8AD8: 0x97D3,
    0x8AD9: 0x9928,
    0x8ADA: 0x8218,
    0x8ADB: 0x4E38,
    0x8ADC: 0x542B,
    0x8ADD: 0x5CB8,
    0x8ADE: 0x5DCC,
    0x8ADF: 0x73A9,
    0x8AE0: 0x764C,
    0x8AE1: 0x773C,
    0x8AE2: 0x5CA9,
    0x8AE3: 0x7FEB,
    0x8AE4: 0x8D0B,
    0x8AE5: 0x96C1,
    0x8AE6: 0x9811,
    0x8AE7: 0x9854,
    0x8AE8: 0x9858,
    0x8AE9: 0x4F01,
    0x8AEA: 0x4F0E,
    0x8AEB: 0x5371,
    0x8AEC: 0x559C,
    0x8AED: 0x5668,
    0x8AEE: 0x57FA,
    0x8AEF: 0x5947,
    0x8AF0: 0x5B09,
    0x8AF1: 0x5BC4,
    0x8AF2: 0x5C90,
    0x8AF3: 0x5E0C,
    0x8AF4: 0x5E7E,
    0x8AF5: 0x5FCC,
    0x8AF6: 0x63EE,
    0x8AF7: 0x673A,
    0x8AF8: 0x65D7,
    0x8AF9: 0x65E2,
    0x8AFA: 0x671F,
    0x8AFB: 0x68CB,
    0x8AFC: 0x68C4,
    0x8B40: 0x6A5F,
    0x8B41: 0x5E30,
    0x8B42: 0x6BC5,
    0x8B43: 0x6C17,
    0x8B44: 0x6C7D,
    0x8B45: 0x757F,
    0x8B46: 0x7948,
    0x8B47: 0x5B63,
    0x8B48: 0x7A00,
    0x8B49: 0x7D00,
    0x8B4A: 0x5FBD,
    0x8B4B: 0x898F,
    0x8B4C: 0x8A18,
    0x8B4D: 0x8CB4,
    0x8B4E: 0x8D77,
    0x8B4F: 0x8ECC,
    0x8B50: 0x8F1D,
    0x8B51: 0x98E2,
    0x8B52: 0x9A0E,
    0x8B53: 0x9B3C,
    0x8B54: 0x4E80,
    0x8B55: 0x507D,
    0x8B56: 0x5100,
    0x8B57: 0x5993,
    0x8B58: 0x5B9C,
    0x8B59: 0x622F,
    0x8B5A: 0x6280,
    0x8B5B: 0x64EC,
    0x8B5C: 0x6B3A,
    0x8B5D: 0x72A0,
    0x8B5E: 0x7591,
    0x8B5F: 0x7947,
    0x8B60: 0x7FA9,
    0x8B61: 0x87FB,
    0x8B62: 0x8ABC,
    0x8B63: 0x8B70,
    0x8B64: 0x63AC,
    0x8B65: 0x83CA,
    0x8B66: 0x97A0,
    0x8B67: 0x5409,
    0x8B68: 0x5403,
    0x8B69: 0x55AB,
    0x8B6A: 0x6854,
    0x8B6B: 0x6A58,
    0x8B6C: 0x8A70,
    0x8B6D: 0x7827,
    0x8B6E: 0x6775,
    0x8B6F: 0x9ECD,
    0x8B70: 0x5374,
    0x8B71: 0x5BA2,
    0x8B72: 0x811A,
    0x8B73: 0x8650,
    0x8B74: 0x9006,
    0x8B75: 0x4E18,
    0x8B76: 0x4E45,
    0x8B77: 0x4EC7,
    0x8B78: 0x4F11,
    0x8B79: 0x53CA,
    0x8B7A: 0x5438,
    0x8B7B: 0x5BAE,
    0x8B7C: 0x5F13,
    0x8B7D: 0x6025,
    0x8B7E: 0x6551,
    0x8B80: 0x673D,
    0x8B81: 0x6C42,
    0x8B82: 0x6C72,
    0x8B83: 0x6CE3,
    0x8B84: 0x7078,
    0x8B85: 0x7403,
    0x8B86: 0x7A76,
    0x8B87: 0x7AAE,
    0x8B88: 0x7B08,
    0x8B89: 0x7D1A,
    0x8B8A: 0x7CFE,
    0x8B8B: 0x7D66,
    0x8B8C: 0x65E7,
    0x8B8D: 0x725B,
    0x8B8E: 0x53BB,
    0x8B8F: 0x5C45,
    0x8B90: 0x5DE8,
    0x8B91: 0x62D2,
    0x8B92: 0x62E0,
    0x8B93: 0x6319,
    0x8B94: 0x6E20,
    0x8B95: 0x865A,
    0x8B96: 0x8A31,
    0x8B97: 0x8DDD,
    0x8B98: 0x92F8,
    0x8B99: 0x6F01,
    0x8B9A: 0x79A6,
    0x8B9B: 0x9B5A,
    0x8B9C: 0x4EA8,
    0x8B9D: 0x4EAB,
    0x8B9E: 0x4EAC,
    0x8B9F: 0x4F9B,
    0x8BA0: 0x4FA0,
    0x8BA1: 0x50D1,
    0x8BA2: 0x5147,
    0x8BA3: 0x7AF6,
    0x8BA4: 0x5171,
    0x8BA5: 0x51F6,
    0x8BA6: 0x5354,
    0x8BA7: 0x5321,
    0x8BA8: 0x537F,
    0x8BA9: 0x53EB,
    0x8BAA: 0x55AC,
    0x8BAB: 0x5883,
    0x8BAC: 0x5CE1,
    0x8BAD: 0x5F37,
    0x8BAE: 0x5F4A,
    0x8BAF: 0x602F,
    0x8BB0: 0x6050,
    0x8BB1: 0x606D,
    0x8BB2: 0x631F,
    0x8BB3: 0x6559,
    0x8BB4: 0x6A4B,
    0x8BB5: 0x6CC1,
    0x8BB6: 0x72C2,
    0x8BB7: 0x72ED,
    0x8BB8: 0x77EF,
    0x8BB9: 0x80F8,
    0x8BBA: 0x8105,
    0x8BBB: 0x8208,
    0x8BBC: 0x854E,
    0x8BBD: 0x90F7,
    0x8BBE: 0x93E1,
    0x8BBF: 0x97FF,
    0x8BC0: 0x9957,
    0x8BC1: 0x9A5A,
    0x8BC2: 0x4EF0,
    0x8BC3: 0x51DD,
    0x8BC4: 0x5C2D,
    0x8BC5: 0x6681,
    0x8BC6: 0x696D,
    0x8BC7: 0x5C40,
    0x8BC8: 0x66F2,
    0x8BC9: 0x6975,
    0x8BCA: 0x7389,
    0x8BCB: 0x6850,
    0x8BCC: 0x7C81,
    0x8BCD: 0x50C5,
    0x8BCE: 0x52E4,
    0x8BCF: 0x5747,
    0x8BD0: 0x5DFE,
    0x8BD1: 0x9326,
    0x8BD2: 0x65A4,
    0x8BD3: 0x6B23,
    0x8BD4: 0x6B3D,
    0x8BD5: 0x7434,
    0x8BD6: 0x7981,
    0x8BD7: 0x79BD,
    0x8BD8: 0x7B4B,
    0x8BD9: 0x7DCA,
    0x8BDA: 0x82B9,
    0x8BDB: 0x83CC,
    0x8BDC: 0x887F,
    0x8BDD: 0x895F,
    0x8BDE: 0x8B39,
    0x8BDF: 0x8FD1,
    0x8BE0: 0x91D1,
    0x8BE1: 0x541F,
    0x8BE2: 0x9280,
    0x8BE3: 0x4E5D,
    0x8BE4: 0x5036,
    0x8BE5: 0x53E5,
    0x8BE6: 0x533A,
    0x8BE7: 0x72D7,
    0x8BE8: 0x7396,
    0x8BE9: 0x77E9,
    0x8BEA: 0x82E6,
    0x8BEB: 0x8EAF,
    0x8BEC: 0x99C6,
    0x8BED: 0x99C8,
    0x8BEE: 0x99D2,
    0x8BEF: 0x5177,
    0x8BF0: 0x611A,
    0x8BF1: 0x865E,
    0x8BF2: 0x55B0,
    0x8BF3: 0x7A7A,
    0x8BF4: 0x5076,
    0x8BF5: 0x5BD3,
    0x8BF6: 0x9047,
    0x8BF7: 0x9685,
    0x8BF8: 0x4E32,
    0x8BF9: 0x6ADB,
    0x8BFA: 0x91E7,
    0x8BFB: 0x5C51,
    0x8BFC: 0x5C48,
    0x8C40: 0x6398,
    0x8C41: 0x7A9F,
    0x8C42: 0x6C93,
    0x8C43: 0x9774,
    0x8C44: 0x8F61,
    0x8C45: 0x7AAA,
    0x8C46: 0x718A,
    0x8C47: 0x9688,
    0x8C48: 0x7C82,
    0x8C49: 0x6817,
    0x8C4A: 0x7E70,
    0x8C4B: 0x6851,
    0x8C4C: 0x936C,
    0x8C4D: 0x52F2,
    0x8C4E: 0x541B,
    0x8C4F: 0x85AB,
    0x8C50: 0x8A13,
    0x8C51: 0x7FA4,
    0x8C52: 0x8ECD,
    0x8C53: 0x90E1,
    0x8C54: 0x5366,
    0x8C55: 0x8888,
    0x8C56: 0x7941,
    0x8C57: 0x4FC2,
    0x8C58: 0x50BE,
    0x8C59: 0x5211,
    0x8C5A: 0x5144,
    0x8C5B: 0x5553,
    0x8C5C: 0x572D,
    0x8C5D: 0x73EA,
    0x8C5E: 0x578B,
    0x8C5F: 0x5951,
    0x8C60: 0x5F62,
    0x8C61: 0x5F84,
    0x8C62: 0x6075,
    0x8C63: 0x6176,
    0x8C64: 0x6167,
    0x8C65: 0x61A9,
    0x8C66: 0x63B2,
    0x8C67: 0x643A,
    0x8C68: 0x656C,
    0x8C69: 0x666F,
    0x8C6A: 0x6842,
    0x8C6B: 0x6E13,
    0x8C6C: 0x7566,
    0x8C6D: 0x7A3D,
    0x8C6E: 0x7CFB,
    0x8C6F: 0x7D4C,
    0x8C70: 0x7D99,
    0x8C71: 0x7E4B,
    0x8C72: 0x7F6B,
    0x8C73: 0x830E,
    0x8C74: 0x834A,
    0x8C75: 0x86CD,
    0x8C76: 0x8A08,
    0x8C77: 0x8A63,
    0x8C78: 0x8B66,
    0x8C79: 0x8EFD,
    0x8C7A: 0x981A,
    0x8C7B: 0x9D8F,
    0x8C7C: 0x82B8,
    0x8C7D: 0x8FCE,
    0x8C7E: 0x9BE8,
    0x8C80: 0x5287,
    0x8C81: 0x621F,
    0x8C82: 0x6483,
    0x8C83: 0x6FC0,
    0x8C84: 0x9699,
    0x8C85: 0x6841,
    0x8C86: 0x5091,
    0x8C87: 0x6B20,
    0x8C88: 0x6C7A,
    0x8C89: 0x6F54,
    0x8C8A: 0x7A74,
    0x8C8B: 0x7D50,
    0x8C8C: 0x8840,
    0x8C8D: 0x8A23,
    0x8C8E: 0x6708,
    0x8C8F: 0x4EF6,
    0x8C90: 0x5039,
    0x8C91: 0x5026,
    0x8C92: 0x5065,
    0x8C93: 0x517C,
    0x8C94: 0x5238,
    0x8C95: 0x5263,
    0x8C96: 0x55A7,
    0x8C97: 0x570F,
    0x8C98: 0x5805,
    0x8C99: 0x5ACC,
    0x8C9A: 0x5EFA,
    0x8C9B: 0x61B2,
    0x8C9C: 0x61F8,
    0x8C9D: 0x62F3,
    0x8C9E: 0x6372,
    0x8C9F: 0x691C,
    0x8CA0: 0x6A29,
    0x8CA1: 0x727D,
    0x8CA2: 0x72AC,
    0x8CA3: 0x732E,
    0x8CA4: 0x7814,
    0x8CA5: 0x786F,
    0x8CA6: 0x7D79,
    0x8CA7: 0x770C,
    0x8CA8: 0x80A9,
    0x8CA9: 0x898B,
    0x8CAA: 0x8B19,
    0x8CAB: 0x8CE2,
    0x8CAC: 0x8ED2,
    0x8CAD: 0x9063,
    0x8CAE: 0x9375,
    0x8CAF: 0x967A,
    0x8CB0: 0x9855,
    0x8CB1: 0x9A13,
    0x8CB2: 0x9E78,
    0x8CB3: 0x5143,
    0x8CB4: 0x539F,
    0x8CB5: 0x53B3,
    0x8CB6: 0x5E7B,
    0x8CB7: 0x5F26,
    0x8CB8: 0x6E1B,
    0x8CB9: 0x6E90,
    0x8CBA: 0x7384,
    0x8CBB: 0x73FE,
    0x8CBC: 0x7D43,
    0x8CBD: 0x8237,
    0x8CBE: 0x8A00,
    0x8CBF: 0x8AFA,
    0x8CC0: 0x9650,
    0x8CC1: 0x4E4E,
    0x8CC2: 0x500B,
    0x8CC3: 0x53E4,
    0x8CC4: 0x547C,
    0x8CC5: 0x56FA,
    0x8CC6: 0x59D1,
    0x8CC7: 0x5B64,
    0x8CC8: 0x5DF1,
    0x8CC9: 0x5EAB,
    0x8CCA: 0x5F27,
    0x8CCB: 0x6238,
    0x8CCC: 0x6545,
    0x8CCD: 0x67AF,
    0x8CCE: 0x6E56,
    0x8CCF: 0x72D0,
    0x8CD0: 0x7CCA,
    0x8CD1: 0x88B4,
    0x8CD2: 0x80A1,
    0x8CD3: 0x80E1,
    0x8CD4: 0x83F0,
    0x8CD5: 0x864E,
    0x8CD6: 0x8A87,
    0x8CD7: 0x8DE8,
    0x8CD8: 0x9237,
    0x8CD9: 0x96C7,
    0x8CDA: 0x9867,
    0x8CDB: 0x9F13,
    0x8CDC: 0x4E94,
    0x8CDD: 0x4E92,
    0x8CDE: 0x4F0D,
    0x8CDF: 0x5348,
    0x8CE0: 0x5449,
    0x8CE1: 0x543E,
    0x8CE2: 0x5A2F,
    0x8CE3: 0x5F8C,
    0x8CE4: 0x5FA1,
    0x8CE5: 0x609F,
    0x8CE6: 0x68A7,
    0x8CE7: 0x6A8E,
    0x8CE8: 0x745A,
    0x8CE9: 0x7881,
    0x8CEA: 0x8A9E,
    0x8CEB: 0x8AA4,
    0x8CEC: 0x8B77,
    0x8CED: 0x9190,
    0x8CEE: 0x4E5E,
    0x8CEF: 0x9BC9,
    0x8CF0: 0x4EA4,
    0x8CF1: 0x4F7C,
    0x8CF2: 0x4FAF,
    0x8CF3: 0x5019,
    0x8CF4: 0x5016,
    0x8CF5: 0x5149,
    0x8CF6: 0x516C,
    0x8CF7: 0x529F,
    0x8CF8: 0x52B9,
    0x8CF9: 0x52FE,
    0x8CFA: 0x539A,
    0x8CFB: 0x53E3,
    0x8CFC: 0x5411,
    0x8D40: 0x540E,
    0x8D41: 0x5589,
    0x8D42: 0x5751,
    0x8D43: 0x57A2,
    0x8D44: 0x597D,
    0x8D45: 0x5B54,
    0x8D46: 0x5B5D,
    0x8D47: 0x5B8F,
    0x8D48: 0x5DE5,
    0x8D49: 0x5DE7,
    0x8D4A: 0x5DF7,
    0x8D4B: 0x5E78,
    0x8D4C: 0x5E83,
    0x8D4D: 0x5E9A,
    0x8D4E: 0x5EB7,
    0x8D4F: 0x5F18,
    0x8D50: 0x6052,
    0x8D51: 0x614C,
    0x8D52: 0x6297,
    0x8D53: 0x62D8,
    0x8D54: 0x63A7,
    0x8D55: 0x653B,
    0x8D56: 0x6602,
    0x8D57: 0x6643,
    0x8D58: 0x66F4,
    0x8D59: 0x676D,
    0x8D5A: 0x6821,
    0x8D5B: 0x6897,
    0x8D5C: 0x69CB,
    0x8D5D: 0x6C5F,
    0x8D5E: 0x6D2A,
    0x8D5F: 0x6D69,
    0x8D60: 0x6E2F,
    0x8D61: 0x6E9D,
    0x8D62: 0x7532,
    0x8D63: 0x7687,
    0x8D64: 0x786C,
    0x8D65: 0x7A3F,
    0x8D66: 0x7CE0,
    0x8D67: 0x7D05,
    0x8D68: 0x7D18,
    0x8D69: 0x7D5E,
    0x8D6A: 0x7DB1,
    0x8D6B: 0x8015,
    0x8D6C: 0x8003,
    0x8D6D: 0x80AF,
    0x8D6E: 0x80B1,
    0x8D6F: 0x8154,
    0x8D70: 0x818F,
    0x8D71: 0x822A,
    0x8D72: 0x8352,
    0x8D73: 0x884C,
    0x8D74: 0x8861,
    0x8D75: 0x8B1B,
    0x8D76: 0x8CA2,
    0x8D77: 0x8CFC,
    0x8D78: 0x90CA,
    0x8D79: 0x9175,
    0x8D7A: 0x9271,
    0x8D7B: 0x783F,
    0x8D7C: 0x92FC,
    0x8D7D: 0x95A4,
    0x8D7E: 0x964D,
    0x8D80: 0x9805,
    0x8D81: 0x9999,
    0x8D82: 0x9AD8,
    0x8D83: 0x9D3B,
    0x8D84: 0x525B,
    0x8D85: 0x52AB,
    0x8D86: 0x53F7,
    0x8D87: 0x5408,
    0x8D88: 0x58D5,
    0x8D89: 0x62F7,
    0x8D8A: 0x6FE0,
    0x8D8B: 0x8C6A,
    0x8D8C: 0x8F5F,
    0x8D8D: 0x9EB9,
    0x8D8E: 0x514B,
    0x8D8F: 0x523B,
    0x8D90: 0x544A,
    0x8D91: 0x56FD,
    0x8D92: 0x7A40,
    0x8D93: 0x9177,
    0x8D94: 0x9D60,
    0x8D95: 0x9ED2,
    0x8D96: 0x7344,
    0x8D97: 0x6F09,
    0x8D98: 0x8170,
    0x8D99: 0x7511,
    0x8D9A: 0x5FFD,
    0x8D9B: 0x60DA,
    0x8D9C: 0x9AA8,
    0x8D9D: 0x72DB,
    0x8D9E: 0x8FBC,
    0x8D9F: 0x6B64,
    0x8DA0: 0x9803,
    0x8DA1: 0x4ECA,
    0x8DA2: 0x56F0,
    0x8DA3: 0x5764,
    0x8DA4: 0x58BE,
    0x8DA5: 0x5A5A,
    0x8DA6: 0x6068,
    0x8DA7: 0x61C7,
    0x8DA8: 0x660F,
    0x8DA9: 0x6606,
    0x8DAA: 0x6839,
    0x8DAB: 0x68B1,
    0x8DAC: 0x6DF7,
    0x8DAD: 0x75D5,
    0x8DAE: 0x7D3A,
    0x8DAF: 0x826E,
    0x8DB0: 0x9B42,
    0x8DB1: 0x4E9B,
    0x8DB2: 0x4F50,
    0x8DB3: 0x53C9,
    0x8DB4: 0x5506,
    0x8DB5: 0x5D6F,
    0x8DB6: 0x5DE6,
    0x8DB7: 0x5DEE,
    0x8DB8: 0x67FB,
    0x8DB9: 0x6C99,
    0x8DBA: 0x7473,
    0x8DBB: 0x7802,
    0x8DBC: 0x8A50,
    0x8DBD: 0x9396,
    0x8DBE: 0x88DF,
    0x8DBF: 0x5750,
    0x8DC0: 0x5EA7,
    0x8DC1: 0x632B,
    0x8DC2: 0x50B5,
    0x8DC3: 0x50AC,
    0x8DC4: 0x518D,
    0x8DC5: 0x6700,
    0x8DC6: 0x54C9,
    0x8DC7: 0x585E,
    0x8DC8: 0x59BB,
    0x8DC9: 0x5BB0,
    0x8DCA: 0x5F69,
    0x8DCB: 0x624D,
    0x8DCC: 0x63A1,
    0x8DCD: 0x683D,
    0x8DCE: 0x6B73,
    0x8DCF: 0x6E08,
    0x8DD0: 0x707D,
    0x8DD1: 0x91C7,
    0x8DD2: 0x7280,
    0x8DD3: 0x7815,
    0x8DD4: 0x7826,
    0x8DD5: 0x796D,
    0x8DD6: 0x658E,
    0x8DD7: 0x7D30,
    0x8DD8: 0x83DC,
    0x8DD9: 0x88C1,
    0x8DDA: 0x8F09,
    0x8DDB: 0x969B,
    0x8DDC: 0x5264,
    0x8DDD: 0x5728,
    0x8DDE: 0x6750,
    0x8DDF: 0x7F6A,
    0x8DE0: 0x8CA1,
    0x8DE1: 0x51B4,
    0x8DE2: 0x5742,
    0x8DE3: 0x962A,
    0x8DE4: 0x583A,
    0x8DE5: 0x698A,
    0x8DE6: 0x80B4,
    0x8DE7: 0x54B2,
    0x8DE8: 0x5D0E,
    0x8DE9: 0x57FC,
    0x8DEA: 0x7895,
    0x8DEB: 0x9DFA,
    0x8DEC: 0x4F5C,
    0x8DED: 0x524A,
    0x8DEE: 0x548B,
    0x8DEF: 0x643E,
    0x8DF0: 0x6628,
    0x8DF1: 0x6714,
    0x8DF2: 0x67F5,
    0x8DF3: 0x7A84,
    0x8DF4: 0x7B56,
    0x8DF5: 0x7D22,
    0x8DF6: 0x932F,
    0x8DF7: 0x685C,
    0x8DF8: 0x9BAD,
    0x8DF9: 0x7B39,
    0x8DFA: 0x5319,
    0x8DFB: 0x518A,
    0x8DFC: 0x5237,
    0x8E40: 0x5BDF,
    0x8E41: 0x62F6,
    0x8E42: 0x64AE,
    0x8E43: 0x64E6,
    0x8E44: 0x672D,
    0x8E45: 0x6BBA,
    0x8E46: 0x85A9,
    0x8E47: 0x96D1,
    0x8E48: 0x7690,
    0x8E49: 0x9BD6,
    0x8E4A: 0x634C,
    0x8E4B: 0x9306,
    0x8E4C: 0x9BAB,
    0x8E4D: 0x76BF,
    0x8E4E: 0x6652,
    0x8E4F: 0x4E09,
    0x8E50: 0x5098,
    0x8E51: 0x53C2,
    0x8E52: 0x5C71,
    0x8E53: 0x60E8,
    0x8E54: 0x6492,
    0x8E55: 0x6563,
    0x8E56: 0x685F,
    0x8E57: 0x71E6,
    0x8E58: 0x73CA,
    0x8E59: 0x7523,
    0x8E5A: 0x7B97,
    0x8E5B: 0x7E82,
    0x8E5C: 0x8695,
    0x8E5D: 0x8B83,
    0x8E5E: 0x8CDB,
    0x8E5F: 0x9178,
    0x8E60: 0x9910,
    0x8E61: 0x65AC,
    0x8E62: 0x66AB,
    0x8E63: 0x6B8B,
    0x8E64: 0x4ED5,
    0x8E65: 0x4ED4,
    0x8E66: 0x4F3A,
    0x8E67: 0x4F7F,
    0x8E68: 0x523A,
    0x8E69: 0x53F8,
    0x8E6A: 0x53F2,
    0x8E6B: 0x55E3,
    0x8E6C: 0x56DB,
    0x8E6D: 0x58EB,
    0x8E6E: 0x59CB,
    0x8E6F: 0x59C9,
    0x8E70: 0x59FF,
    0x8E71: 0x5B50,
    0x8E72: 0x5C4D,
    0x8E73: 0x5E02,
    0x8E74: 0x5E2B,
    0x8E75: 0x5FD7,
    0x8E76: 0x601D,
    0x8E77: 0x6307,
    0x8E78: 0x652F,
    0x8E79: 0x5B5C,
    0x8E7A: 0x65AF,
    0x8E7B: 0x65BD,
    0x8E7C: 0x65E8,
    0x8E7D: 0x679D,
    0x8E7E: 0x6B62,
    0x8E80: 0x6B7B,
    0x8E81: 0x6C0F,
    0x8E82: 0x7345,
    0x8E83: 0x7949,
    0x8E84: 0x79C1,
    0x8E85: 0x7CF8,
    0x8E86: 0x7D19,
    0x8E87: 0x7D2B,
    0x8E88: 0x80A2,
    0x8E89: 0x8102,
    0x8E8A: 0x81F3,
    0x8E8B: 0x8996,
    0x8E8C: 0x8A5E,
    0x8E8D: 0x8A69,
    0x8E8E: 0x8A66,
    0x8E8F: 0x8A8C,
    0x8E90: 0x8AEE,
    0x8E91: 0x8CC7,
    0x8E92: 0x8CDC,
    0x8E93: 0x96CC,
    0x8E94: 0x98FC,
    0x8E95: 0x6B6F,
    0x8E96: 0x4E8B,
    0x8E97: 0x4F3C,
    0x8E98: 0x4F8D,
    0x8E99: 0x5150,
    0x8E9A: 0x5B57,
    0x8E9B: 0x5BFA,
    0x8E9C: 0x6148,
    0x8E9D: 0x6301,
    0x8E9E: 0x6642,
    0x8E9F: 0x6B21,
    0x8EA0: 0x6ECB,
    0x8EA1: 0x6CBB,
    0x8EA2: 0x723E,
    0x8EA3: 0x74BD,
    0x8EA4: 0x75D4,
    0x8EA5: 0x78C1,
    0x8EA6: 0x793A,
    0x8EA7: 0x800C,
    0x8EA8: 0x8033,
    0x8EA9: 0x81EA,
    0x8EAA: 0x8494,
    0x8EAB: 0x8F9E,
    0x8EAC: 0x6C50,
    0x8EAD: 0x9E7F,
    0x8EAE: 0x5F0F,
    0x8EAF: 0x8B58,
    0x8EB0: 0x9D2B,
    0x8EB1: 0x7AFA,
    0x8EB2: 0x8EF8,
    0x8EB3: 0x5B8D,
    0x8EB4: 0x96EB,
    0x8EB5: 0x4E03,
    0x8EB6: 0x53F1,
    0x8EB7: 0x57F7,
    0x8EB8: 0x5931,
    0x8EB9: 0x5AC9,
    0x8EBA: 0x5BA4,
    0x8EBB: 0x6089,
    0x8EBC: 0x6E7F,
    0x8EBD: 0x6F06,
    0x8EBE: 0x75BE,
    0x8EBF: 0x8CEA,
    0x8EC0: 0x5B9F,
    0x8EC1: 0x8500,
    0x8EC2: 0x7BE0,
    0x8EC3: 0x5072,
    0x8EC4: 0x67F4,
    0x8EC5: 0x829D,
    0x8EC6: 0x5C61,
    0x8EC7: 0x854A,
    0x8EC8: 0x7E1E,
    0x8EC9: 0x820E,
    0x8ECA: 0x5199,
    0x8ECB: 0x5C04,
    0x8ECC: 0x6368,
    0x8ECD: 0x8D66,
    0x8ECE: 0x659C,
    0x8ECF: 0x716E,
    0x8ED0: 0x793E,
    0x8ED1: 0x7D17,
    0x8ED2: 0x8005,
    0x8ED3: 0x8B1D,
    0x8ED4: 0x8ECA,
    0x8ED5: 0x906E,
    0x8ED6: 0x86C7,
    0x8ED7: 0x90AA,
    0x8ED8: 0x501F,
    0x8ED9: 0x52FA,
    0x8EDA: 0x5C3A,
    0x8EDB: 0x6753,
    0x8EDC: 0x707C,
    0x8EDD: 0x7235,
    0x8EDE: 0x914C,
    0x8EDF: 0x91C8,
    0x8EE0: 0x932B,
    0x8EE1: 0x82E5,
    0x8EE2: 0x5BC2,
    0x8EE3: 0x5F31,
    0x8EE4: 0x60F9,
    0x8EE5: 0x4E3B,
    0x8EE6: 0x53D6,
    0x8EE7: 0x5B88,
    0x8EE8: 0x624B,
    0x8EE9: 0x6731,
    0x8EEA: 0x6B8A,
    0x8EEB: 0x72E9,
    0x8EEC: 0x73E0,
    0x8EED: 0x7A2E,
    0x8EEE: 0x816B,
    0x8EEF: 0x8DA3,
    0x8EF0: 0x9152,
    0x8EF1: 0x9996,
    0x8EF2: 0x5112,
    0x8EF3: 0x53D7,
    0x8EF4: 0x546A,
    0x8EF5: 0x5BFF,
    0x8EF6: 0x6388,
    0x8EF7: 0x6A39,
    0x8EF8: 0x7DAC,
    0x8EF9: 0x9700,
    0x8EFA: 0x56DA,
    0x8EFB: 0x53CE,
    0x8EFC: 0x5468,
    0x8F40: 0x5B97,
    0x8F41: 0x5C31,
    0x8F42: 0x5DDE,
    0x8F43: 0x4FEE,
    0x8F44: 0x6101,
    0x8F45: 0x62FE,
    0x8F46: 0x6D32,
    0x8F47: 0x79C0,
    0x8F48: 0x79CB,
    0x8F49: 0x7D42,
    0x8F4A: 0x7E4D,
    0x8F4B: 0x7FD2,
    0x8F4C: 0x81ED,
    0x8F4D: 0x821F,
    0x8F4E: 0x8490,
    0x8F4F: 0x8846,
    0x8F50: 0x8972,
    0x8F51: 0x8B90,
    0x8F52: 0x8E74,
    0x8F53: 0x8F2F,
    0x8F54: 0x9031,
    0x8F55: 0x914B,
    0x8F56: 0x916C,
    0x8F57: 0x96C6,
    0x8F58: 0x919C,
    0x8F59: 0x4EC0,
    0x8F5A: 0x4F4F,
    0x8F5B: 0x5145,
    0x8F5C: 0x5341,
    0x8F5D: 0x5F93,
    0x8F5E: 0x620E,
    0x8F5F: 0x67D4,
    0x8F60: 0x6C41,
    0x8F61: 0x6E0B,
    0x8F62: 0x7363,
    0x8F63: 0x7E26,
    0x8F64: 0x91CD,
    0x8F65: 0x9283,
    0x8F66: 0x53D4,
    0x8F67: 0x5919,
    0x8F68: 0x5BBF,
    0x8F69: 0x6DD1,
    0x8F6A: 0x795D,
    0x8F6B: 0x7E2E,
    0x8F6C: 0x7C9B,
    0x8F6D: 0x587E,
    0x8F6E: 0x719F,
    0x8F6F: 0x51FA,
    0x8F70: 0x8853,
    0x8F71: 0x8FF0,
    0x8F72: 0x4FCA,
    0x8F73: 0x5CFB,
    0x8F74: 0x6625,
    0x8F75: 0x77AC,
    0x8F76: 0x7AE3,
    0x8F77: 0x821C,
    0x8F78: 0x99FF,
    0x8F79: 0x51C6,
    0x8F7A: 0x5FAA,
    0x8F7B: 0x65EC,
    0x8F7C: 0x696F,
    0x8F7D: 0x6B89,
    0x8F7E: 0x6DF3,
    0x8F80: 0x6E96,
    0x8F81: 0x6F64,
    0x8F82: 0x76FE,
    0x8F83: 0x7D14,
    0x8F84: 0x5DE1,
    0x8F85: 0x9075,
    0x8F86: 0x9187,
    0x8F87: 0x9806,
    0x8F88: 0x51E6,
    0x8F89: 0x521D,
    0x8F8A: 0x6240,
    0x8F8B: 0x6691,
    0x8F8C: 0x66D9,
    0x8F8D: 0x6E1A,
    0x8F8E: 0x5EB6,
    0x8F8F: 0x7DD2,
    0x8F90: 0x7F72,
    0x8F91: 0x66F8,
    0x8F92: 0x85AF,
    0x8F93: 0x85F7,
    0x8F94: 0x8AF8,
    0x8F95: 0x52A9,
    0x8F96: 0x53D9,
    0x8F97: 0x5973,
    0x8F98: 0x5E8F,
    0x8F99: 0x5F90,
    0x8F9A: 0x6055,
    0x8F9B: 0x92E4,
    0x8F9C: 0x9664,
    0x8F9D: 0x50B7,
    0x8F9E: 0x511F,
    0x8F9F: 0x52DD,
    0x8FA0: 0x5320,
    0x8FA1: 0x5347,
    0x8FA2: 0x53EC,
    0x8FA3: 0x54E8,
    0x8FA4: 0x5546,
    0x8FA5: 0x5531,
    0x8FA6: 0x5617,
    0x8FA7: 0x5968,
    0x8FA8: 0x59BE,
    0x8FA9: 0x5A3C,
    0x8FAA: 0x5BB5,
    0x8FAB: 0x5C06,
    0x8FAC: 0x5C0F,
    0x8FAD: 0x5C11,
    0x8FAE: 0x5C1A,
    0x8FAF: 0x5E84,
    0x8FB0: 0x5E8A,
    0x8FB1: 0x5EE0,
    0x8FB2: 0x5F70,
    0x8FB3: 0x627F,
    0x8FB4: 0x6284,
    0x8FB5: 0x62DB,
    0x8FB6: 0x638C,
    0x8FB7: 0x6377,
    0x8FB8: 0x6607,
    0x8FB9: 0x660C,
    0x8FBA: 0x662D,
    0x8FBB: 0x6676,
    0x8FBC: 0x677E,
    0x8FBD: 0x68A2,
    0x8FBE: 0x6A1F,
    0x8FBF: 0x6A35,
    0x8FC0: 0x6CBC,
    0x8FC1: 0x6D88,
    0x8FC2: 0x6E09,
    0x8FC3: 0x6E58,
    0x8FC4: 0x713C,
    0x8FC5: 0x7126,
    0x8FC6: 0x7167,
    0x8FC7: 0x75C7,
    0x8FC8: 0x7701,
    0x8FC9: 0x785D,
    0x8FCA: 0x7901,
    0x8FCB: 0x7965,
    0x8FCC: 0x79F0,
    0x8FCD: 0x7AE0,
    0x8FCE: 0x7B11,
    0x8FCF: 0x7CA7,
    0x8FD0: 0x7D39,
    0x8FD1: 0x8096,
    0x8FD2: 0x83D6,
    0x8FD3: 0x848B,
    0x8FD4: 0x8549,
    0x8FD5: 0x885D,
    0x8FD6: 0x88F3,
    0x8FD7: 0x8A1F,
    0x8FD8: 0x8A3C,
    0x8FD9: 0x8A54,
    0x8FDA: 0x8A73,
    0x8FDB: 0x8C61,
    0x8FDC: 0x8CDE,
    0x8FDD: 0x91A4,
    0x8FDE: 0x9266,
    0x8FDF: 0x937E,
    0x8FE0: 0x9418,
    0x8FE1: 0x969C,
    0x8FE2: 0x9798,
    0x8FE3: 0x4E0A,
    0x8FE4: 0x4E08,
    0x8FE5: 0x4E1E,
    0x8FE6: 0x4E57,
    0x8FE7: 0x5197,
    0x8FE8: 0x5270,
    0x8FE9: 0x57CE,
    0x8FEA: 0x5834,
    0x8FEB: 0x58CC,
    0x8FEC: 0x5B22,
    0x8FED: 0x5E38,
    0x8FEE: 0x60C5,
    0x8FEF: 0x64FE,
    0x8FF0: 0x6761,
    0x8FF1: 0x6756,
    0x8FF2: 0x6D44,
    0x8FF3: 0x72B6,
    0x8FF4: 0x7573,
    0x8FF5: 0x7A63,
    0x8FF6: 0x84B8,
    0x8FF7: 0x8B72,
    0x8FF8: 0x91B8,
    0x8FF9: 0x9320,
    0x8FFA: 0x5631,
    0x8FFB: 0x57F4,
    0x8FFC: 0x98FE,
    0x9040: 0x62ED,
    0x9041: 0x690D,
    0x9042: 0x6B96,
    0x9043: 0x71ED,
    0x9044: 0x7E54,
    0x9045: 0x8077,
    0x9046: 0x8272,
    0x9047: 0x89E6,
    0x9048: 0x98DF,
    0x9049: 0x8755,
    0x904A: 0x8FB1,
    0x904B: 0x5C3B,
    0x904C: 0x4F38,
    0x904D: 0x4FE1,
    0x904E: 0x4FB5,
    0x904F: 0x5507,
    0x9050: 0x5A20,
    0x9051: 0x5BDD,
    0x9052: 0x5BE9,
    0x9053: 0x5FC3,
    0x9054: 0x614E,
    0x9055: 0x632F,
    0x9056: 0x65B0,
    0x9057: 0x664B,
    0x9058: 0x68EE,
    0x9059: 0x699B,
    0x905A: 0x6D78,
    0x905B: 0x6DF1,
    0x905C: 0x7533,
    0x905D: 0x75B9,
    0x905E: 0x771F,
    0x905F: 0x795E,
    0x9060: 0x79E6,
    0x9061: 0x7D33,
    0x9062: 0x81E3,
    0x9063: 0x82AF,
    0x9064: 0x85AA,
    0x9065: 0x89AA,
    0x9066: 0x8A3A,
    0x9067: 0x8EAB,
    0x9068: 0x8F9B,
    0x9069: 0x9032,
    0x906A: 0x91DD,
    0x906B: 0x9707,
    0x906C: 0x4EBA,
    0x906D: 0x4EC1,
    0x906E: 0x5203,
    0x906F: 0x5875,
    0x9070: 0x58EC,
    0x9071: 0x5C0B,
    0x9072: 0x751A,
    0x9073: 0x5C3D,
    0x9074: 0x814E,
    0x9075: 0x8A0A,
    0x9076: 0x8FC5,
    0x9077: 0x9663,
    0x9078: 0x976D,
    0x9079: 0x7B25,
    0x907A: 0x8ACF,
    0x907B: 0x9808,
    0x907C: 0x9162,
    0x907D: 0x56F3,
    0x907E: 0x53A8,
    0x9080: 0x9017,
    0x9081: 0x5439,
    0x9082: 0x5782,
    0x9083: 0x5E25,
    0x9084: 0x63A8,
    0x9085: 0x6C34,
    0x9086: 0x708A,
    0x9087: 0x7761,
    0x9088: 0x7C8B,
    0x9089: 0x7FE0,
    0x908A: 0x8870,
    0x908B: 0x9042,
    0x908C: 0x9154,
    0x908D: 0x9310,
    0x908E: 0x9318,
    0x908F: 0x968F,
    0x9090: 0x745E,
    0x9091: 0x9AC4,
    0x9092: 0x5D07,
    0x9093: 0x5D69,
    0x9094: 0x6570,
    0x9095: 0x67A2,
    0x9096: 0x8DA8,
    0x9097: 0x96DB,
    0x9098: 0x636E,
    0x9099: 0x6749,
    0x909A: 0x6919,
    0x909B: 0x83C5,
    0x909C: 0x9817,
    0x909D: 0x96C0,
    0x909E: 0x88FE,
    0x909F: 0x6F84,
    0x90A0: 0x647A,
    0x90A1: 0x5BF8,
    0x90A2: 0x4E16,
    0x90A3: 0x702C,
    0x90A4: 0x755D,
    0x90A5: 0x662F,
    0x90A6: 0x51C4,
    0x90A7: 0x5236,
    0x90A8: 0x52E2,
    0x90A9: 0x59D3,
    0x90AA: 0x5F81,
    0x90AB: 0x6027,
    0x90AC: 0x6210,
    0x90AD: 0x653F,
    0x90AE: 0x6574,
    0x90AF: 0x661F,
    0x90B0: 0x6674,
    0x90B1: 0x68F2,
    0x90B2: 0x6816,
    0x90B3: 0x6B63,
    0x90B4: 0x6E05,
    0x90B5: 0x7272,
    0x90B6: 0x751F,
    0x90B7: 0x76DB,
    0x90B8: 0x7CBE,
    0x90B9: 0x8056,
    0x90BA: 0x58F0,
    0x90BB: 0x88FD,
    0x90BC: 0x897F,
    0x90BD: 0x8AA0,
    0x90BE: 0x8A93,
    0x90BF: 0x8ACB,
    0x90C0: 0x901D,
    0x90C1: 0x9192,
    0x90C2: 0x9752,
    0x90C3: 0x9759,
    0x90C4: 0x6589,
    0x90C5: 0x7A0E,
    0x90C6: 0x8106,
    0x90C7: 0x96BB,
    0x90C8: 0x5E2D,
    0x90C9: 0x60DC,
    0x90CA: 0x621A,
    0x90CB: 0x65A5,
    0x90CC: 0x6614,
    0x90CD: 0x6790,
    0x90CE: 0x77F3,
    0x90CF: 0x7A4D,
    0x90D0: 0x7C4D,
    0x90D1: 0x7E3E,
    0x90D2: 0x810A,
    0x90D3: 0x8CAC,
    0x90D4: 0x8D64,
    0x90D5: 0x8DE1,
    0x90D6: 0x8E5F,
    0x90D7: 0x78A9,
    0x90D8: 0x5207,
    0x90D9: 0x62D9,
    0x90DA: 0x63A5,
    0x90DB: 0x6442,
    0x90DC: 0x6298,
    0x90DD: 0x8A2D,
    0x90DE: 0x7A83,
    0x90DF: 0x7BC0,
    0x90E0: 0x8AAC,
    0x90E1: 0x96EA,
    0x90E2: 0x7D76,
    0x90E3: 0x820C,
    0x90E4: 0x8749,
    0x90E5: 0x4ED9,
    0x90E6: 0x5148,
    0x90E7: 0x5343,
    0x90E8: 0x5360,
    0x90E9: 0x5BA3,
    0x90EA: 0x5C02,
    0x90EB: 0x5C16,
    0x90EC: 0x5DDD,
    0x90ED: 0x6226,
    0x90EE: 0x6247,
    0x90EF: 0x64B0,
    0x90F0: 0x6813,
    0x90F1: 0x6834,
    0x90F2: 0x6CC9,
    0x90F3: 0x6D45,
    0x90F4: 0x6D17,
    0x90F5: 0x67D3,
    0x90F6: 0x6F5C,
    0x90F7: 0x714E,
    0x90F8: 0x717D,
    0x90F9: 0x65CB,
    0x90FA: 0x7A7F,
    0x90FB: 0x7BAD,
    0x90FC: 0x7DDA,
    0x9140: 0x7E4A,
    0x9141: 0x7FA8,
    0x9142: 0x817A,
    0x9143: 0x821B,
    0x9144: 0x8239,
    0x9145: 0x85A6,
    0x9146: 0x8A6E,
    0x9147: 0x8CCE,
    0x9148: 0x8DF5,
    0x9149: 0x9078,
    0x914A: 0x9077,
    0x914B: 0x92AD,
    0x914C: 0x9291,
    0x914D: 0x9583,
    0x914E: 0x9BAE,
    0x914F: 0x524D,
    0x9150: 0x5584,
    0x9151: 0x6F38,
    0x9152: 0x7136,
    0x9153: 0x5168,
    0x9154: 0x7985,
    0x9155: 0x7E55,
    0x9156: 0x81B3,
    0x9157: 0x7CCE,
    0x9158: 0x564C,
    0x9159: 0x5851,
    0x915A: 0x5CA8,
    0x915B: 0x63AA,
    0x915C: 0x66FE,
    0x915D: 0x66FD,
    0x915E: 0x695A,
    0x915F: 0x72D9,
    0x9160: 0x758F,
    0x9161: 0x758E,
    0x9162: 0x790E,
    0x9163: 0x7956,
    0x9164: 0x79DF,
    0x9165: 0x7C97,
    0x9166: 0x7D20,
    0x9167: 0x7D44,
    0x9168: 0x8607,
    0x9169: 0x8A34,
    0x916A: 0x963B,
    0x916B: 0x9061,
    0x916C: 0x9F20,
    0x916D: 0x50E7,
    0x916E: 0x5275,
    0x916F: 0x53CC,
    0x9170: 0x53E2,
    0x9171: 0x5009,
    0x9172: 0x55AA,
    0x9173: 0x58EE,
    0x9174: 0x594F,
    0x9175: 0x723D,
    0x9176: 0x5B8B,
    0x9177: 0x5C64,
    0x9178: 0x531D,
    0x9179: 0x60E3,
    0x917A: 0x60F3,
    0x917B: 0x635C,
    0x917C: 0x6383,
    0x917D: 0x633F,
    0x917E: 0x63BB,
    0x9180: 0x64CD,
    0x9181: 0x65E9,
    0x9182: 0x66F9,
    0x9183: 0x5DE3,
    0x9184: 0x69CD,
    0x9185: 0x69FD,
    0x9186: 0x6F15,
    0x9187: 0x71E5,
    0x9188: 0x4E89,
    0x9189: 0x75E9,
    0x918A: 0x76F8,
    0x918B: 0x7A93,
    0x918C: 0x7CDF,
    0x918D: 0x7DCF,
    0x918E: 0x7D9C,
    0x918F: 0x8061,
    0x9190: 0x8349,
    0x9191: 0x8358,
    0x9192: 0x846C,
    0x9193: 0x84BC,
    0x9194: 0x85FB,
    0x9195: 0x88C5,
    0x9196: 0x8D70,
    0x9197: 0x9001,
    0x9198: 0x906D,
    0x9199: 0x9397,
    0x919A: 0x971C,
    0x919B: 0x9A12,
    0x919C: 0x50CF,
    0x919D: 0x5897,
    0x919E: 0x618E,
    0x919F: 0x81D3,
    0x91A0: 0x8535,
    0x91A1: 0x8D08,
    0x91A2: 0x9020,
    0x91A3: 0x4FC3,
    0x91A4: 0x5074,
    0x91A5: 0x5247,
    0x91A6: 0x5373,
    0x91A7: 0x606F,
    0x91A8: 0x6349,
    0x91A9: 0x675F,
    0x91AA: 0x6E2C,
    0x91AB: 0x8DB3,
    0x91AC: 0x901F,
    0x91AD: 0x4FD7,
    0x91AE: 0x5C5E,
    0x91AF: 0x8CCA,
    0x91B0: 0x65CF,
    0x91B1: 0x7D9A,
    0x91B2: 0x5352,
    0x91B3: 0x8896,
    0x91B4: 0x5176,
    0x91B5: 0x63C3,
    0x91B6: 0x5B58,
    0x91B7: 0x5B6B,
    0x91B8: 0x5C0A,
    0x91B9: 0x640D,
    0x91BA: 0x6751,
    0x91BB: 0x905C,
    0x91BC: 0x4ED6,
    0x91BD: 0x591A,
    0x91BE: 0x592A,
    0x91BF: 0x6C70,
    0x91C0: 0x8A51,
    0x91C1: 0x553E,
    0x91C2: 0x5815,
    0x91C3: 0x59A5,
    0x91C4: 0x60F0,
    0x91C5: 0x6253,
    0x91C6: 0x67C1,
    0x91C7: 0x8235,
    0x91C8: 0x6955,
    0x91C9: 0x9640,
    0x91CA: 0x99C4,
    0x91CB: 0x9A28,
    0x91CC: 0x4F53,
    0x91CD: 0x5806,
    0x91CE: 0x5BFE,
    0x91CF: 0x8010,
    0x91D0: 0x5CB1,
    0x91D1: 0x5E2F,
    0x91D2: 0x5F85,
    0x91D3: 0x6020,
    0x91D4: 0x614B,
    0x91D5: 0x6234,
    0x91D6: 0x66FF,
    0x91D7: 0x6CF0,
    0x91D8: 0x6EDE,
    0x91D9: 0x80CE,
    0x91DA: 0x817F,
    0x91DB: 0x82D4,
    0x91DC: 0x888B,
    0x91DD: 0x8CB8,
    0x91DE: 0x9000,
    0x91DF: 0x902E,
    0x91E0: 0x968A,
    0x91E1: 0x9EDB,
    0x91E2: 0x9BDB,
    0x91E3: 0x4EE3,
    0x91E4: 0x53F0,
    0x91E5: 0x5927,
    0x91E6: 0x7B2C,
    0x91E7: 0x918D,
    0x91E8: 0x984C,
    0x91E9: 0x9DF9,
    0x91EA: 0x6EDD,
    0x91EB: 0x7027,
    0x91EC: 0x5353,
    0x91ED: 0x5544,
    0x91EE: 0x5B85,
    0x91EF: 0x6258,
    0x91F0: 0x629E,
    0x91F1: 0x62D3,
    0x91F2: 0x6CA2,
    0x91F3: 0x6FEF,
    0x91F4: 0x7422,
    0x91F5: 0x8A17,
    0x91F6: 0x9438,
    0x91F7: 0x6FC1,
    0x91F8: 0x8AFE,
    0x91F9: 0x8338,
    0x91FA: 0x51E7,
    0x91FB: 0x86F8,
    0x91FC: 0x53EA,
    0x9240: 0x53E9,
    0x9241: 0x4F46,
    0x9242: 0x9054,
    0x9243: 0x8FB0,
    0x9244: 0x596A,
    0x9245: 0x8131,
    0x9246: 0x5DFD,
    0x9247: 0x7AEA,
    0x9248: 0x8FBF,
    0x9249: 0x68DA,
    0x924A: 0x8C37,
    0x924B: 0x72F8,
    0x924C: 0x9C48,
    0x924D: 0x6A3D,
    0x924E: 0x8AB0,
    0x924F: 0x4E39,
    0x9250: 0x5358,
    0x9251: 0x5606,
    0x9252: 0x5766,
    0x9253: 0x62C5,
    0x9254: 0x63A2,
    0x9255: 0x65E6,
    0x9256: 0x6B4E,
    0x9257: 0x6DE1,
    0x9258: 0x6E5B,
    0x9259: 0x70AD,
    0x925A: 0x77ED,
    0x925B: 0x7AEF,
    0x925C: 0x7BAA,
    0x925D: 0x7DBB,
    0x925E: 0x803D,
    0x925F: 0x80C6,
    0x9260: 0x86CB,
    0x9261: 0x8A95,
    0x9262: 0x935B,
    0x9263: 0x56E3,
    0x9264: 0x58C7,
    0x9265: 0x5F3E,
    0x9266: 0x65AD,
    0x9267: 0x6696,
    0x9268: 0x6A80,
    0x9269: 0x6BB5,
    0x926A: 0x7537,
    0x926B: 0x8AC7,
    0x926C: 0x5024,
    0x926D: 0x77E5,
    0x926E: 0x5730,
    0x926F: 0x5F1B,
    0x9270: 0x6065,
    0x9271: 0x667A,
    0x9272: 0x6C60,
    0x9273: 0x75F4,
    0x9274: 0x7A1A,
    0x9275: 0x7F6E,
    0x9276: 0x81F4,
    0x9277: 0x8718,
    0x9278: 0x9045,
    0x9279: 0x99B3,
    0x927A: 0x7BC9,
    0x927B: 0x755C,
    0x927C: 0x7AF9,
    0x927D: 0x7B51,
    0x927E: 0x84C4,
    0x9280: 0x9010,
    0x9281: 0x79E9,
    0x9282: 0x7A92,
    0x9283: 0x8336,
    0x9284: 0x5AE1,
    0x9285: 0x7740,
    0x9286: 0x4E2D,
    0x9287: 0x4EF2,
    0x9288: 0x5B99,
    0x9289: 0x5FE0,
    0x928A: 0x62BD,
    0x928B: 0x663C,
    0x928C: 0x67F1,
    0x928D: 0x6CE8,
    0x928E: 0x866B,
    0x928F: 0x8877,
    0x9290: 0x8A3B,
    0x9291: 0x914E,
    0x9292: 0x92F3,
    0x9293: 0x99D0,
    0x9294: 0x6A17,
    0x9295: 0x7026,
    0x9296: 0x732A,
    0x9297: 0x82E7,
    0x9298: 0x8457,
    0x9299: 0x8CAF,
    0x929A: 0x4E01,
    0x929B: 0x5146,
    0x929C: 0x51CB,
    0x929D: 0x558B,
    0x929E: 0x5BF5,
    0x929F: 0x5E16,
    0x92A0: 0x5E33,
    0x92A1: 0x5E81,
    0x92A2: 0x5F14,
    0x92A3: 0x5F35,
    0x92A4: 0x5F6B,
    0x92A5: 0x5FB4,
    0x92A6: 0x61F2,
    0x92A7: 0x6311,
    0x92A8: 0x66A2,
    0x92A9: 0x671D,
    0x92AA: 0x6F6E,
    0x92AB: 0x7252,
    0x92AC: 0x753A,
    0x92AD: 0x773A,
    0x92AE: 0x8074,
    0x92AF: 0x8139,
    0x92B0: 0x8178,
    0x92B1: 0x8776,
    0x92B2: 0x8ABF,
    0x92B3: 0x8ADC,
    0x92B4: 0x8D85,
    0x92B5: 0x8DF3,
    0x92B6: 0x929A,
    0x92B7: 0x9577,
    0x92B8: 0x9802,
    0x92B9: 0x9CE5,
    0x92BA: 0x52C5,
    0x92BB: 0x6357,
    0x92BC: 0x76F4,
    0x92BD: 0x6715,
    0x92BE: 0x6C88,
    0x92BF: 0x73CD,
    0x92C0: 0x8CC3,
    0x92C1: 0x93AE,
    0x92C2: 0x9673,
    0x92C3: 0x6D25,
    0x92C4: 0x589C,
    0x92C5: 0x690E,
    0x92C6: 0x69CC,
    0x92C7: 0x8FFD,
    0x92C8: 0x939A,
    0x92C9: 0x75DB,
    0x92CA: 0x901A,
    0x92CB: 0x585A,
    0x92CC: 0x6802,
    0x92CD: 0x63B4,
    0x92CE: 0x69FB,
    0x92CF: 0x4F43,
    0x92D0: 0x6F2C,
    0x92D1: 0x67D8,
    0x92D2: 0x8FBB,
    0x92D3: 0x8526,
    0x92D4: 0x7DB4,
    0x92D5: 0x9354,
    0x92D6: 0x693F,
    0x92D7: 0x6F70,
    0x92D8: 0x576A,
    0x92D9: 0x58F7,
    0x92DA: 0x5B2C,
    0x92DB: 0x7D2C,
    0x92DC: 0x722A,
    0x92DD: 0x540A,
    0x92DE: 0x91E3,
    0x92DF: 0x9DB4,
    0x92E0: 0x4EAD,
    0x92E1: 0x4F4E,
    0x92E2: 0x505C,
    0x92E3: 0x5075,
    0x92E4: 0x5243,
    0x92E5: 0x8C9E,
    0x92E6: 0x5448,
    0x92E7: 0x5824,
    0x92E8: 0x5B9A,
    0x92E9: 0x5E1D,
    0x92EA: 0x5E95,
    0x92EB: 0x5EAD,
    0x92EC: 0x5EF7,
    0x92ED: 0x5F1F,
    0x92EE: 0x608C,
    0x92EF: 0x62B5,
    0x92F0: 0x633A,
    0x92F1: 0x63D0,
    0x92F2: 0x68AF,
    0x92F3: 0x6C40,
    0x92F4: 0x7887,
    0x92F5: 0x798E,
    0x92F6: 0x7A0B,
    0x92F7: 0x7DE0,
    0x92F8: 0x8247,
    0x92F9: 0x8A02,
    0x92FA: 0x8AE6,
    0x92FB: 0x8E44,
    0x92FC: 0x9013,
    0x9340: 0x90B8,
    0x9341: 0x912D,
    0x9342: 0x91D8,
    0x9343: 0x9F0E,
    0x9344: 0x6CE5,
    0x9345: 0x6458,
    0x9346: 0x64E2,
    0x9347: 0x6575,
    0x9348: 0x6EF4,
    0x9349: 0x7684,
    0x934A: 0x7B1B,
    0x934B: 0x9069,
    0x934C: 0x93D1,
    0x934D: 0x6EBA,
    0x934E: 0x54F2,
    0x934F: 0x5FB9,
    0x9350: 0x64A4,
    0x9351: 0x8F4D,
    0x9352: 0x8FED,
    0x9353: 0x9244,
    0x9354: 0x5178,
    0x9355: 0x586B,
    0x9356: 0x5929,
    0x9357: 0x5C55,
    0x9358: 0x5E97,
    0x9359: 0x6DFB,
    0x935A: 0x7E8F,
    0x935B: 0x751C,
    0x935C: 0x8CBC,
    0x935D: 0x8EE2,
    0x935E: 0x985B,
    0x935F: 0x70B9,
    0x9360: 0x4F1D,
    0x9361: 0x6BBF,
    0x9362: 0x6FB1,
    0x9363: 0x7530,
    0x9364: 0x96FB,
    0x9365: 0x514E,
    0x9366: 0x5410,
    0x9367: 0x5835,
    0x9368: 0x5857,
    0x9369: 0x59AC,
    0x936A: 0x5C60,
    0x936B: 0x5F92,
    0x936C: 0x6597,
    0x936D: 0x675C,
    0x936E: 0x6E21,
    0x936F: 0x767B,
    0x9370: 0x83DF,
    0x9371: 0x8CED,
    0x9372: 0x9014,
    0x9373: 0x90FD,
    0x9374: 0x934D,
    0x9375: 0x7825,
    0x9376: 0x783A,
    0x9377: 0x52AA,
    0x9378: 0x5EA6,
    0x9379: 0x571F,
    0x937A: 0x5974,
    0x937B: 0x6012,
    0x937C: 0x5012,
    0x937D: 0x515A,
    0x937E: 0x51AC,
    0x9380: 0x51CD,
    0x9381: 0x5200,
    0x9382: 0x5510,
    0x9383: 0x5854,
    0x9384: 0x5858,
    0x9385: 0x5957,
    0x9386: 0x5B95,
    0x9387: 0x5CF6,
    0x9388: 0x5D8B,
    0x9389: 0x60BC,
    0x938A: 0x6295,
    0x938B: 0x642D,
    0x938C: 0x6771,
    0x938D: 0x6843,
    0x938E: 0x68BC,
    0x938F: 0x68DF,
    0x9390: 0x76D7,
    0x9391: 0x6DD8,
    0x9392: 0x6E6F,
    0x9393: 0x6D9B,
    0x9394: 0x706F,
    0x9395: 0x71C8,
    0x9396: 0x5F53,
    0x9397: 0x75D8,
    0x9398: 0x7977,
    0x9399: 0x7B49,
    0x939A: 0x7B54,
    0x939B: 0x7B52,
    0x939C: 0x7CD6,
    0x939D: 0x7D71,
    0x939E: 0x5230,
    0x939F: 0x8463,
    0x93A0: 0x8569,
    0x93A1: 0x85E4,
    0x93A2: 0x8A0E,
    0x93A3: 0x8B04,
    0x93A4: 0x8C46,
    0x93A5: 0x8E0F,
    0x93A6: 0x9003,
    0x93A7: 0x900F,
    0x93A8: 0x9419,
    0x93A9: 0x9676,
    0x93AA: 0x982D,
    0x93AB: 0x9A30,
    0x93AC: 0x95D8,
    0x93AD: 0x50CD,
    0x93AE: 0x52D5,
    0x93AF: 0x540C,
    0x93B0: 0x5802,
    0x93B1: 0x5C0E,
    0x93B2: 0x61A7,
    0x93B3: 0x649E,
    0x93B4: 0x6D1E,
    0x93B5: 0x77B3,
    0x93B6: 0x7AE5,
    0x93B7: 0x80F4,
    0x93B8: 0x8404,
    0x93B9: 0x9053,
    0x93BA: 0x9285,
    0x93BB: 0x5CE0,
    0x93BC: 0x9D07,
    0x93BD: 0x533F,
    0x93BE: 0x5F97,
    0x93BF: 0x5FB3,
    0x93C0: 0x6D9C,
    0x93C1: 0x7279,
    0x93C2: 0x7763,
    0x93C3: 0x79BF,
    0x93C4: 0x7BE4,
    0x93C5: 0x6BD2,
    0x93C6: 0x72EC,
    0x93C7: 0x8AAD,
    0x93C8: 0x6803,
    0x93C9: 0x6A61,
    0x93CA: 0x51F8,
    0x93CB: 0x7A81,
    0x93CC: 0x6934,
    0x93CD: 0x5C4A,
    0x93CE: 0x9CF6,
    0x93CF: 0x82EB,
    0x93D0: 0x5BC5,
    0x93D1: 0x9149,
    0x93D2: 0x701E,
    0x93D3: 0x5678,
    0x93D4: 0x5C6F,
    0x93D5: 0x60C7,
    0x93D6: 0x6566,
    0x93D7: 0x6C8C,
    0x93D8: 0x8C5A,
    0x93D9: 0x9041,
    0x93DA: 0x9813,
    0x93DB: 0x5451,
    0x93DC: 0x66C7,
    0x93DD: 0x920D,
    0x93DE: 0x5948,
    0x93DF: 0x90A3,
    0x93E0: 0x5185,
    0x93E1: 0x4E4D,
    0x93E2: 0x51EA,
    0x93E3: 0x8599,
    0x93E4: 0x8B0E,
    0x93E5: 0x7058,
    0x93E6: 0x637A,
    0x93E7: 0x934B,
    0x93E8: 0x6962,
    0x93E9: 0x99B4,
    0x93EA: 0x7E04,
    0x93EB: 0x7577,
    0x93EC: 0x5357,
    0x93ED: 0x6960,
    0x93EE: 0x8EDF,
    0x93EF: 0x96E3,
    0x93F0: 0x6C5D,
    0x93F1: 0x4E8C,
    0x93F2: 0x5C3C,
    0x93F3: 0x5F10,
    0x93F4: 0x8FE9,
    0x93F5: 0x5302,
    0x93F6: 0x8CD1,
    0x93F7: 0x8089,
    0x93F8: 0x8679,
    0x93F9: 0x5EFF,
    0x93FA: 0x65E5,
    0x93FB: 0x4E73,
    0x93FC: 0x5165,
    0x9440: 0x5982,
    0x9441: 0x5C3F,
    0x9442: 0x97EE,
    0x9443: 0x4EFB,
    0x9444: 0x598A,
    0x9445: 0x5FCD,
    0x9446: 0x8A8D,
    0x9447: 0x6FE1,
    0x9448: 0x79B0,
    0x9449: 0x7962,
    0x944A: 0x5BE7,
    0x944B: 0x8471,
    0x944C: 0x732B,
    0x944D: 0x71B1,
    0x944E: 0x5E74,
    0x944F: 0x5FF5,
    0x9450: 0x637B,
    0x9451: 0x649A,
    0x9452: 0x71C3,
    0x9453: 0x7C98,
    0x9454: 0x4E43,
    0x9455: 0x5EFC,
    0x9456: 0x4E4B,
    0x9457: 0x57DC,
    0x9458: 0x56A2,
    0x9459: 0x60A9,
    0x945A: 0x6FC3,
    0x945B: 0x7D0D,
    0x945C: 0x80FD,
    0x945D: 0x8133,
    0x945E: 0x81BF,
    0x945F: 0x8FB2,
    0x9460: 0x8997,
    0x9461: 0x86A4,
    0x9462: 0x5DF4,
    0x9463: 0x628A,
    0x9464: 0x64AD,
    0x9465: 0x8987,
    0x9466: 0x6777,
    0x9467: 0x6CE2,
    0x9468: 0x6D3E,
    0x9469: 0x7436,
    0x946A: 0x7834,
    0x946B: 0x5A46,
    0x946C: 0x7F75,
    0x946D: 0x82AD,
    0x946E: 0x99AC,
    0x946F: 0x4FF3,
    0x9470: 0x5EC3,
    0x9471: 0x62DD,
    0x9472: 0x6392,
    0x9473: 0x6557,
    0x9474: 0x676F,
    0x9475: 0x76C3,
    0x9476: 0x724C,
    0x9477: 0x80CC,
    0x9478: 0x80BA,
    0x9479: 0x8F29,
    0x947A: 0x914D,
    0x947B: 0x500D,
    0x947C: 0x57F9,
    0x947D: 0x5A92,
    0x947E: 0x6885,
    0x9480: 0x6973,
    0x9481: 0x7164,
    0x9482: 0x72FD,
    0x9483: 0x8CB7,
    0x9484: 0x58F2,
    0x9485: 0x8CE0,
    0x9486: 0x966A,
    0x9487: 0x9019,
    0x9488: 0x877F,
    0x9489: 0x79E4,
    0x948A: 0x77E7,
    0x948B: 0x8429,
    0x948C: 0x4F2F,
    0x948D: 0x5265,
    0x948E: 0x535A,
    0x948F: 0x62CD,
    0x9490: 0x67CF,
    0x9491: 0x6CCA,
    0x9492: 0x767D,
    0x9493: 0x7B94,
    0x9494: 0x7C95,
    0x9495: 0x8236,
    0x9496: 0x8584,
    0x9497: 0x8FEB,
    0x9498: 0x66DD,
    0x9499: 0x6F20,
    0x949A: 0x7206,
    0x949B: 0x7E1B,
    0x949C: 0x83AB,
    0x949D: 0x99C1,
    0x949E: 0x9EA6,
    0x949F: 0x51FD,
    0x94A0: 0x7BB1,
    0x94A1: 0x7872,
    0x94A2: 0x7BB8,
    0x94A3: 0x8087,
    0x94A4: 0x7B48,
    0x94A5: 0x6AE8,
    0x94A6: 0x5E61,
    0x94A7: 0x808C,
    0x94A8: 0x7551,
    0x94A9: 0x7560,
    0x94AA: 0x516B,
    0x94AB: 0x9262,
    0x94AC: 0x6E8C,
    0x94AD: 0x767A,
    0x94AE: 0x9197,
    0x94AF: 0x9AEA,
    0x94B0: 0x4F10,
    0x94B1: 0x7F70,
    0x94B2: 0x629C,
    0x94B3: 0x7B4F,
    0x94B4: 0x95A5,
    0x94B5: 0x9CE9,
    0x94B6: 0x567A,
    0x94B7: 0x5859,
    0x94B8: 0x86E4,
    0x94B9: 0x96BC,
    0x94BA: 0x4F34,
    0x94BB: 0x5224,
    0x94BC: 0x534A,
    0x94BD: 0x53CD,
    0x94BE: 0x53DB,
    0x94BF: 0x5E06,
    0x94C0: 0x642C,
    0x94C1: 0x6591,
    0x94C2: 0x677F,
    0x94C3: 0x6C3E,
    0x94C4: 0x6C4E,
    0x94C5: 0x7248,
    0x94C6: 0x72AF,
    0x94C7: 0x73ED,
    0x94C8: 0x7554,
    0x94C9: 0x7E41,
    0x94CA: 0x822C,
    0x94CB: 0x85E9,
    0x94CC: 0x8CA9,
    0x94CD: 0x7BC4,
    0x94CE: 0x91C6,
    0x94CF: 0x7169,
    0x94D0: 0x9812,
    0x94D1: 0x98EF,
    0x94D2: 0x633D,
    0x94D3: 0x6669,
    0x94D4: 0x756A,
    0x94D5: 0x76E4,
    0x94D6: 0x78D0,
    0x94D7: 0x8543,
    0x94D8: 0x86EE,
    0x94D9: 0x532A,
    0x94DA: 0x5351,
    0x94DB: 0x5426,
    0x94DC: 0x5983,
    0x94DD: 0x5E87,
    0x94DE: 0x5F7C,
    0x94DF: 0x60B2,
    0x94E0: 0x6249,
    0x94E1: 0x6279,
    0x94E2: 0x62AB,
    0x94E3: 0x6590,
    0x94E4: 0x6BD4,
    0x94E5: 0x6CCC,
    0x94E6: 0x75B2,
    0x94E7: 0x76AE,
    0x94E8: 0x7891,
    0x94E9: 0x79D8,
    0x94EA: 0x7DCB,
    0x94EB: 0x7F77,
    0x94EC: 0x80A5,
    0x94ED: 0x88AB,
    0x94EE: 0x8AB9,
    0x94EF: 0x8CBB,
    0x94F0: 0x907F,
    0x94F1: 0x975E,
    0x94F2: 0x98DB,
    0x94F3: 0x6A0B,
    0x94F4: 0x7C38,
    0x94F5: 0x5099,
    0x94F6: 0x5C3E,
    0x94F7: 0x5FAE,
    0x94F8: 0x6787,
    0x94F9: 0x6BD8,
    0x94FA: 0x7435,
    0x94FB: 0x7709,
    0x94FC: 0x7F8E,
    0x9540: 0x9F3B,
    0x9541: 0x67CA,
    0x9542: 0x7A17,
    0x9543: 0x5339,
    0x9544: 0x758B,
    0x9545: 0x9AED,
    0x9546: 0x5F66,
    0x9547: 0x819D,
    0x9548: 0x83F1,
    0x9549: 0x8098,
    0x954A: 0x5F3C,
    0x954B: 0x5FC5,
    0x954C: 0x7562,
    0x954D: 0x7B46,
    0x954E: 0x903C,
    0x954F: 0x6867,
    0x9550: 0x59EB,
    0x9551: 0x5A9B,
    0x9552: 0x7D10,
    0x9553: 0x767E,
    0x9554: 0x8B2C,
    0x9555: 0x4FF5,
    0x9556: 0x5F6A,
    0x9557: 0x6A19,
    0x9558: 0x6C37,
    0x9559: 0x6F02,
    0x955A: 0x74E2,
    0x955B: 0x7968,
    0x955C: 0x8868,
    0x955D: 0x8A55,
    0x955E: 0x8C79,
    0x955F: 0x5EDF,
    0x9560: 0x63CF,
    0x9561: 0x75C5,
    0x9562: 0x79D2,
    0x9563: 0x82D7,
    0x9564: 0x9328,
    0x9565: 0x92F2,
    0x9566: 0x849C,
    0x9567: 0x86ED,
    0x9568: 0x9C2D,
    0x9569: 0x54C1,
    0x956A: 0x5F6C,
    0x956B: 0x658C,
    0x956C: 0x6D5C,
    0x956D: 0x7015,
    0x956E: 0x8CA7,
    0x956F: 0x8CD3,
    0x9570: 0x983B,
    0x9571: 0x654F,
    0x9572: 0x74F6,
    0x9573: 0x4E0D,
    0x9574: 0x4ED8,
    0x9575: 0x57E0,
    0x9576: 0x592B,
    0x9577: 0x5A66,
    0x9578: 0x5BCC,
    0x9579: 0x51A8,
    0x957A: 0x5E03,
    0x957B: 0x5E9C,
    0x957C: 0x6016,
    0x957D: 0x6276,
    0x957E: 0x6577,
    0x9580: 0x65A7,
    0x9581: 0x666E,
    0x9582: 0x6D6E,
    0x9583: 0x7236,
    0x9584: 0x7B26,
    0x9585: 0x8150,
    0x9586: 0x819A,
    0x9587: 0x8299,
    0x9588: 0x8B5C,
    0x9589: 0x8CA0,
    0x958A: 0x8CE6,
    0x958B: 0x8D74,
    0x958C: 0x961C,
    0x958D: 0x9644,
    0x958E: 0x4FAE,
    0x958F: 0x64AB,
    0x9590: 0x6B66,
    0x9591: 0x821E,
    0x9592: 0x8461,
    0x9593: 0x856A,
    0x9594: 0x90E8,
    0x9595: 0x5C01,
    0x9596: 0x6953,
    0x9597: 0x98A8,
    0x9598: 0x847A,
    0x9599: 0x8557,
    0x959A: 0x4F0F,
    0x959B: 0x526F,
    0x959C: 0x5FA9,
    0x959D: 0x5E45,
    0x959E: 0x670D,
    0x959F: 0x798F,
    0x95A0: 0x8179,
    0x95A1: 0x8907,
    0x95A2: 0x8986,
    0x95A3: 0x6DF5,
    0x95A4: 0x5F17,
    0x95A5: 0x6255,
    0x95A6: 0x6CB8,
    0x95A7: 0x4ECF,
    0x95A8: 0x7269,
    0x95A9: 0x9B92,
    0x95AA: 0x5206,
    0x95AB: 0x543B,
    0x95AC: 0x5674,
    0x95AD: 0x58B3,
    0x95AE: 0x61A4,
    0x95AF: 0x626E,
    0x95B0: 0x711A,
    0x95B1: 0x596E,
    0x95B2: 0x7C89,
    0x95B3: 0x7CDE,
    0x95B4: 0x7D1B,
    0x95B5: 0x96F0,
    0x95B6: 0x6587,
    0x95B7: 0x805E,
    0x95B8: 0x4E19,
    0x95B9: 0x4F75,
    0x95BA: 0x5175,
    0x95BB: 0x5840,
    0x95BC: 0x5E63,
    0x95BD: 0x5E73,
    0x95BE: 0x5F0A,
    0x95BF: 0x67C4,
    0x95C0: 0x4E26,
    0x95C1: 0x853D,
    0x95C2: 0x9589,
    0x95C3: 0x965B,
    0x95C4: 0x7C73,
    0x95C5: 0x9801,
    0x95C6: 0x50FB,
    0x95C7: 0x58C1,
    0x95C8: 0x7656,
    0x95C9: 0x78A7,
    0x95CA: 0x5225,
    0x95CB: 0x77A5,
    0x95CC: 0x8511,
    0x95CD: 0x7B86,
    0x95CE: 0x504F,
    0x95CF: 0x5909,
    0x95D0: 0x7247,
    0x95D1: 0x7BC7,
    0x95D2: 0x7DE8,
    0x95D3: 0x8FBA,
    0x95D4: 0x8FD4,
    0x95D5: 0x904D,
    0x95D6: 0x4FBF,
    0x95D7: 0x52C9,
    0x95D8: 0x5A29,
    0x95D9: 0x5F01,
    0x95DA: 0x97AD,
    0x95DB: 0x4FDD,
    0x95DC: 0x8217,
    0x95DD: 0x92EA,
    0x95DE: 0x5703,
    0x95DF: 0x6355,
    0x95E0: 0x6B69,
    0x95E1: 0x752B,
    0x95E2: 0x88DC,
    0x95E3: 0x8F14,
    0x95E4: 0x7A42,
    0x95E5: 0x52DF,
    0x95E6: 0x5893,
    0x95E7: 0x6155,
    0x95E8: 0x620A,
    0x95E9: 0x66AE,
    0x95EA: 0x6BCD,
    0x95EB: 0x7C3F,
    0x95EC: 0x83E9,
    0x95ED: 0x5023,
    0x95EE: 0x4FF8,
    0x95EF: 0x5305,
    0x95F0: 0x5446,
    0x95F1: 0x5831,
    0x95F2: 0x5949,
    0x95F3: 0x5B9D,
    0x95F4: 0x5CF0,
    0x95F5: 0x5CEF,
    0x95F6: 0x5D29,
    0x95F7: 0x5E96,
    0x95F8: 0x62B1,
    0x95F9: 0x6367,
    0x95FA: 0x653E,
    0x95FB: 0x65B9,
    0x95FC: 0x670B,
    0x9640: 0x6CD5,
    0x9641: 0x6CE1,
    0x9642: 0x70F9,
    0x9643: 0x7832,
    0x9644: 0x7E2B,
    0x9645: 0x80DE,
    0x9646: 0x82B3,
    0x9647: 0x840C,
    0x9648: 0x84EC,
    0x9649: 0x8702,
    0x964A: 0x8912,
    0x964B: 0x8A2A,
    0x964C: 0x8C4A,
    0x964D: 0x90A6,
    0x964E: 0x92D2,
    0x964F: 0x98FD,
    0x9650: 0x9CF3,
    0x9651: 0x9D6C,
    0x9652: 0x4E4F,
    0x9653: 0x4EA1,
    0x9654: 0x508D,
    0x9655: 0x5256,
    0x9656: 0x574A,
    0x9657: 0x59A8,
    0x9658: 0x5E3D,
    0x9659: 0x5FD8,
    0x965A: 0x5FD9,
    0x965B: 0x623F,
    0x965C: 0x66B4,
    0x965D: 0x671B,
    0x965E: 0x67D0,
    0x965F: 0x68D2,
    0x9660: 0x5192,
    0x9661: 0x7D21,
    0x9662: 0x80AA,
    0x9663: 0x81A8,
    0x9664: 0x8B00,
    0x9665: 0x8C8C,
    0x9666: 0x8CBF,
    0x9667: 0x927E,
    0x9668: 0x9632,
    0x9669: 0x5420,
    0x966A: 0x982C,
    0x966B: 0x5317,
    0x966C: 0x50D5,
    0x966D: 0x535C,
    0x966E: 0x58A8,
    0x966F: 0x64B2,
    0x9670: 0x6734,
    0x9671: 0x7267,
    0x9672: 0x7766,
    0x9673: 0x7A46,
    0x9674: 0x91E6,
    0x9675: 0x52C3,
    0x9676: 0x6CA1,
    0x9677: 0x6B86,
    0x9678: 0x5800,
    0x9679: 0x5E4C,
    0x967A: 0x5954,
    0x967B: 0x672C,
    0x967C: 0x7FFB,
    0x967D: 0x51E1,
    0x967E: 0x76C6,
    0x9680: 0x6469,
    0x9681: 0x78E8,
    0x9682: 0x9B54,
    0x9683: 0x9EBB,
    0x9684: 0x57CB,
    0x9685: 0x59B9,
    0x9686: 0x6627,
    0x9687: 0x679A,
    0x9688: 0x6BCE,
    0x9689: 0x54E9,
    0x968A: 0x69D9,
    0x968B: 0x5E55,
    0x968C: 0x819C,
    0x968D: 0x6795,
    0x968E: 0x9BAA,
    0x968F: 0x67FE,
    0x9690: 0x9C52,
    0x9691: 0x685D,
    0x9692: 0x4EA6,
    0x9693: 0x4FE3,
    0x9694: 0x53C8,
    0x9695: 0x62B9,
    0x9696: 0x672B,
    0x9697: 0x6CAB,
    0x9698: 0x8FC4,
    0x9699: 0x4FAD,
    0x969A: 0x7E6D,
    0x969B: 0x9EBF,
    0x969C: 0x4E07,
    0x969D: 0x6162,
    0x969E: 0x6E80,
    0x969F: 0x6F2B,
    0x96A0: 0x8513,
    0x96A1: 0x5473,
    0x96A2: 0x672A,
    0x96A3: 0x9B45,
    0x96A4: 0x5DF3,
    0x96A5: 0x7B95,
    0x96A6: 0x5CAC,
    0x96A7: 0x5BC6,
    0x96A8: 0x871C,
    0x96A9: 0x6E4A,
    0x96AA: 0x84D1,
    0x96AB: 0x7A14,
    0x96AC: 0x8108,
    0x96AD: 0x5999,
    0x96AE: 0x7C8D,
    0x96AF: 0x6C11,
    0x96B0: 0x7720,
    0x96B1: 0x52D9,
    0x96B2: 0x5922,
    0x96B3: 0x7121,
    0x96B4: 0x725F,
    0x96B5: 0x77DB,
    0x96B6: 0x9727,
    0x96B7: 0x9D61,
    0x96B8: 0x690B,
    0x96B9: 0x5A7F,
    0x96BA: 0x5A18,
    0x96BB: 0x51A5,
    0x96BC: 0x540D,
    0x96BD: 0x547D,
    0x96BE: 0x660E,
    0x96BF: 0x76DF,
    0x96C0: 0x8FF7,
    0x96C1: 0x9298,
    0x96C2: 0x9CF4,
    0x96C3: 0x59EA,
    0x96C4: 0x725D,
    0x96C5: 0x6EC5,
    0x96C6: 0x514D,
    0x96C7: 0x68C9,
    0x96C8: 0x7DBF,
    0x96C9: 0x7DEC,
    0x96CA: 0x9762,
    0x96CB: 0x9EBA,
    0x96CC: 0x6478,
    0x96CD: 0x6A21,
    0x96CE: 0x8302,
    0x96CF: 0x5984,
    0x96D0: 0x5B5F,
    0x96D1: 0x6BDB,
    0x96D2: 0x731B,
    0x96D3: 0x76F2,
    0x96D4: 0x7DB2,
    0x96D5: 0x8017,
    0x96D6: 0x8499,
    0x96D7: 0x5132,
    0x96D8: 0x6728,
    0x96D9: 0x9ED9,
    0x96DA: 0x76EE,
    0x96DB: 0x6762,
    0x96DC: 0x52FF,
    0x96DD: 0x9905,
    0x96DE: 0x5C24,
    0x96DF: 0x623B,
    0x96E0: 0x7C7E,
    0x96E1: 0x8CB0,
    0x96E2: 0x554F,
    0x96E3: 0x60B6,
    0x96E4: 0x7D0B,
    0x96E5: 0x9580,
    0x96E6: 0x5301,
    0x96E7: 0x4E5F,
    0x96E8: 0x51B6,
    0x96E9: 0x591C,
    0x96EA: 0x723A,
    0x96EB: 0x8036,
    0x96EC: 0x91CE,
    0x96ED: 0x5F25,
    0x96EE: 0x77E2,
    0x96EF: 0x5384,
    0x96F0: 0x5F79,
    0x96F1: 0x7D04,
    0x96F2: 0x85AC,
    0x96F3: 0x8A33,
    0x96F4: 0x8E8D,
    0x96F5: 0x9756,
    0x96F6: 0x67F3,
    0x96F7: 0x85AE,
    0x96F8: 0x9453,
    0x96F9: 0x6109,
    0x96FA: 0x6108,
    0x96FB: 0x6CB9,
    0x96FC: 0x7652,
    0x9740: 0x8AED,
    0x9741: 0x8F38,
    0x9742: 0x552F,
    0x9743: 0x4F51,
    0x9744: 0x512A,
    0x9745: 0x52C7,
    0x9746: 0x53CB,
    0x9747: 0x5BA5,
    0x9748: 0x5E7D,
    0x9749: 0x60A0,
    0x974A: 0x6182,
    0x974B: 0x63D6,
    0x974C: 0x6709,
    0x974D: 0x67DA,
    0x974E: 0x6E67,
    0x974F: 0x6D8C,
    0x9750: 0x7336,
    0x9751: 0x7337,
    0x9752: 0x7531,
    0x9753: 0x7950,
    0x9754: 0x88D5,
    0x9755: 0x8A98,
    0x9756: 0x904A,
    0x9757: 0x9091,
    0x9758: 0x90F5,
    0x9759: 0x96C4,
    0x975A: 0x878D,
    0x975B: 0x5915,
    0x975C: 0x4E88,
    0x975D: 0x4F59,
    0x975E: 0x4E0E,
    0x975F: 0x8A89,
    0x9760: 0x8F3F,
    0x9761: 0x9810,
    0x9762: 0x50AD,
    0x9763: 0x5E7C,
    0x9764: 0x5996,
    0x9765: 0x5BB9,
    0x9766: 0x5EB8,
    0x9767: 0x63DA,
    0x9768: 0x63FA,
    0x9769: 0x64C1,
    0x976A: 0x66DC,
    0x976B: 0x694A,
    0x976C: 0x69D8,
    0x976D: 0x6D0B,
    0x976E: 0x6EB6,
    0x976F: 0x7194,
    0x9770: 0x7528,
    0x9771: 0x7AAF,
    0x9772: 0x7F8A,
    0x9773: 0x8000,
    0x9774: 0x8449,
    0x9775: 0x84C9,
    0x9776: 0x8981,
    0x9777: 0x8B21,
    0x9778: 0x8E0A,
    0x9779: 0x9065,
    0x977A: 0x967D,
    0x977B: 0x990A,
    0x977C: 0x617E,
    0x977D: 0x6291,
    0x977E: 0x6B32,
    0x9780: 0x6C83,
    0x9781: 0x6D74,
    0x9782: 0x7FCC,
    0x9783: 0x7FFC,
    0x9784: 0x6DC0,
    0x9785: 0x7F85,
    0x9786: 0x87BA,
    0x9787: 0x88F8,
    0x9788: 0x6765,
    0x9789: 0x83B1,
    0x978A: 0x983C,
    0x978B: 0x96F7,
    0x978C: 0x6D1B,
    0x978D: 0x7D61,
    0x978E: 0x843D,
    0x978F: 0x916A,
    0x9790: 0x4E71,
    0x9791: 0x5375,
    0x9792: 0x5D50,
    0x9793: 0x6B04,
    0x9794: 0x6FEB,
    0x9795: 0x85CD,
    0x9796: 0x862D,
    0x9797: 0x89A7,
    0x9798: 0x5229,
    0x9799: 0x540F,
    0x979A: 0x5C65,
    0x979B: 0x674E,
    0x979C: 0x68A8,
    0x979D: 0x7406,
    0x979E: 0x7483,
    0x979F: 0x75E2,
    0x97A0: 0x88CF,
    0x97A1: 0x88E1,
    0x97A2: 0x91CC,
    0x97A3: 0x96E2,
    0x97A4: 0x9678,
    0x97A5: 0x5F8B,
    0x97A6: 0x7387,
    0x97A7: 0x7ACB,
    0x97A8: 0x844E,
    0x97A9: 0x63A0,
    0x97AA: 0x7565,
    0x97AB: 0x5289,
    0x97AC: 0x6D41,
    0x97AD: 0x6E9C,
    0x97AE: 0x7409,
    0x97AF: 0x7559,
    0x97B0: 0x786B,
    0x97B1: 0x7C92,
    0x97B2: 0x9686,
    0x97B3: 0x7ADC,
    0x97B4: 0x9F8D,
    0x97B5: 0x4FB6,
    0x97B6: 0x616E,
    0x97B7: 0x65C5,
    0x97B8: 0x865C,
    0x97B9: 0x4E86,
    0x97BA: 0x4EAE,
    0x97BB: 0x50DA,
    0x97BC: 0x4E21,
    0x97BD: 0x51CC,
    0x97BE: 0x5BEE,
    0x97BF: 0x6599,
    0x97C0: 0x6881,
    0x97C1: 0x6DBC,
    0x97C2: 0x731F,
    0x97C3: 0x7642,
    0x97C4: 0x77AD,
    0x97C5: 0x7A1C,
    0x97C6: 0x7CE7,
    0x97C7: 0x826F,
    0x97C8: 0x8AD2,
    0x97C9: 0x907C,
    0x97CA: 0x91CF,
    0x97CB: 0x9675,
    0x97CC: 0x9818,
    0x97CD: 0x529B,
    0x97CE: 0x7DD1,
    0x97CF: 0x502B,
    0x97D0: 0x5398,
    0x97D1: 0x6797,
    0x97D2: 0x6DCB,
    0x97D3: 0x71D0,
    0x97D4: 0x7433,
    0x97D5: 0x81E8,
    0x97D6: 0x8F2A,
    0x97D7: 0x96A3,
    0x97D8: 0x9C57,
    0x97D9: 0x9E9F,
    0x97DA: 0x7460,
    0x97DB: 0x5841,
    0x97DC: 0x6D99,
    0x97DD: 0x7D2F,
    0x97DE: 0x985E,
    0x97DF: 0x4EE4,
    0x97E0: 0x4F36,
    0x97E1: 0x4F8B,
    0x97E2: 0x51B7,
    0x97E3: 0x52B1,
    0x97E4: 0x5DBA,
    0x97E5: 0x601C,
    0x97E6: 0x73B2,
    0x97E7: 0x793C,
    0x97E8: 0x82D3,
    0x97E9: 0x9234,
    0x97EA: 0x96B7,
    0x97EB: 0x96F6,
    0x97EC: 0x970A,
    0x97ED: 0x9E97,
    0x97EE: 0x9F62,
    0x97EF: 0x66A6,
    0x97F0: 0x6B74,
    0x97F1: 0x5217,
    0x97F2: 0x52A3,
    0x97F3: 0x70C8,
    0x97F4: 0x88C2,
    0x97F5: 0x5EC9,
    0x97F6: 0x604B,
    0x97F7: 0x6190,
    0x97F8: 0x6F23,
    0x97F9: 0x7149,
    0x97FA: 0x7C3E,
    0x97FB: 0x7DF4,
    0x97FC: 0x806F,
    0x9840: 0x84EE,
    0x9841: 0x9023,
    0x9842: 0x932C,
    0x9843: 0x5442,
    0x9844: 0x9B6F,
    0x9845: 0x6AD3,
    0x9846: 0x7089,
    0x9847: 0x8CC2,
    0x9848: 0x8DEF,
    0x9849: 0x9732,
    0x984A: 0x52B4,
    0x984B: 0x5A41,
    0x984C: 0x5ECA,
    0x984D: 0x5F04,
    0x984E: 0x6717,
    0x984F: 0x697C,
    0x9850: 0x6994,
    0x9851: 0x6D6A,
    0x9852: 0x6F0F,
    0x9853: 0x7262,
    0x9854: 0x72FC,
    0x9855: 0x7BED,
    0x9856: 0x8001,
    0x9857: 0x807E,
    0x9858: 0x874B,
    0x9859: 0x90CE,
    0x985A: 0x516D,
    0x985B: 0x9E93,
    0x985C: 0x7984,
    0x985D: 0x808B,
    0x985E: 0x9332,
    0x985F: 0x8AD6,
    0x9860: 0x502D,
    0x9861: 0x548C,
    0x9862: 0x8A71,
    0x9863: 0x6B6A,
    0x9864: 0x8CC4,
    0x9865: 0x8107,
    0x9866: 0x60D1,
    0x9867: 0x67A0,
    0x9868: 0x9DF2,
    0x9869: 0x4E99,
    0x986A: 0x4E98,
    0x986B: 0x9C10,
    0x986C: 0x8A6B,
    0x986D: 0x85C1,
    0x986E: 0x8568,
    0x986F: 0x6900,
    0x9870: 0x6E7E,
    0x9871: 0x7897,
    0x9872: 0x8155,
    0x989F: 0x5F0C,
    0x98A0: 0x4E10,
    0x98A1: 0x4E15,
    0x98A2: 0x4E2A,
    0x98A3: 0x4E31,
    0x98A4: 0x4E36,
    0x98A5: 0x4E3C,
    0x98A6: 0x4E3F,
    0x98A7: 0x4E42,
    0x98A8: 0x4E56,
    0x98A9: 0x4E58,
    0x98AA: 0x4E82,
    0x98AB: 0x4E85,
    0x98AC: 0x8C6B,
    0x98AD: 0x4E8A,
    0x98AE: 0x8212,
    0x98AF: 0x5F0D,
    0x98B0: 0x4E8E,
    0x98B1: 0x4E9E,
    0x98B2: 0x4E9F,
    0x98B3: 0x4EA0,
    0x98B4: 0x4EA2,
    0x98B5: 0x4EB0,
    0x98B6: 0x4EB3,
    0x98B7: 0x4EB6,
    0x98B8: 0x4ECE,
    0x98B9: 0x4ECD,
    0x98BA: 0x4EC4,
    0x98BB: 0x4EC6,
    0x98BC: 0x4EC2,
    0x98BD: 0x4ED7,
    0x98BE: 0x4EDE,
    0x98BF: 0x4EED,
    0x98C0: 0x4EDF,
    0x98C1: 0x4EF7,
    0x98C2: 0x4F09,
    0x98C3: 0x4F5A,
    0x98C4: 0x4F30,
    0x98C5: 0x4F5B,
    0x98C6: 0x4F5D,
    0x98C7: 0x4F57,
    0x98C8: 0x4F47,
    0x98C9: 0x4F76,
    0x98CA: 0x4F88,
    0x98CB: 0x4F8F,
    0x98CC: 0x4F98,
    0x98CD: 0x4F7B,
    0x98CE: 0x4F69,
    0x98CF: 0x4F70,
    0x98D0: 0x4F91,
    0x98D1: 0x4F6F,
    0x98D2: 0x4F86,
    0x98D3: 0x4F96,
    0x98D4: 0x5118,
    0x98D5: 0x4FD4,
    0x98D6: 0x4FDF,
    0x98D7: 0x4FCE,
    0x98D8: 0x4FD8,
    0x98D9: 0x4FDB,
    0x98DA: 0x4FD1,
    0x98DB: 0x4FDA,
    0x98DC: 0x4FD0,
    0x98DD: 0x4FE4,
    0x98DE: 0x4FE5,
    0x98DF: 0x501A,
    0x98E0: 0x5028,
    0x98E1: 0x5014,
    0x98E2: 0x502A,
    0x98E3: 0x5025,
    0x98E4: 0x5005,
    0x98E5: 0x4F1C,
    0x98E6: 0x4FF6,
    0x98E7: 0x5021,
    0x98E8: 0x5029,
    0x98E9: 0x502C,
    0x98EA: 0x4FFE,
    0x98EB: 0x4FEF,
    0x98EC: 0x5011,
    0x98ED: 0x5006,
    0x98EE: 0x5043,
    0x98EF: 0x5047,
    0x98F0: 0x6703,
    0x98F1: 0x5055,
    0x98F2: 0x5050,
    0x98F3: 0x5048,
    0x98F4: 0x505A,
    0x98F5: 0x5056,
    0x98F6: 0x506C,
    0x98F7: 0x5078,
    0x98F8: 0x5080,
    0x98F9: 0x509A,
    0x98FA: 0x5085,
    0x98FB: 0x50B4,
    0x98FC: 0x50B2,
    0x9940: 0x50C9,
    0x9941: 0x50CA,
    0x9942: 0x50B3,
    0x9943: 0x50C2,
    0x9944: 0x50D6,
    0x9945: 0x50DE,
    0x9946: 0x50E5,
    0x9947: 0x50ED,
    0x9948: 0x50E3,
    0x9949: 0x50EE,
    0x994A: 0x50F9,
    0x994B: 0x50F5,
    0x994C: 0x5109,
    0x994D: 0x5101,
    0x994E: 0x5102,
    0x994F: 0x5116,
    0x9950: 0x5115,
    0x9951: 0x5114,
    0x9952: 0x511A,
    0x9953: 0x5121,
    0x9954: 0x513A,
    0x9955: 0x5137,
    0x9956: 0x513C,
    0x9957: 0x513B,
    0x9958: 0x513F,
    0x9959: 0x5140,
    0x995A: 0x5152,
    0x995B: 0x514C,
    0x995C: 0x5154,
    0x995D: 0x5162,
    0x995E: 0x7AF8,
    0x995F: 0x5169,
    0x9960: 0x516A,
    0x9961: 0x516E,
    0x9962: 0x5180,
    0x9963: 0x5182,
    0x9964: 0x56D8,
    0x9965: 0x518C,
    0x9966: 0x5189,
    0x9967: 0x518F,
    0x9968: 0x5191,
    0x9969: 0x5193,
    0x996A: 0x5195,
    0x996B: 0x5196,
    0x996C: 0x51A4,
    0x996D: 0x51A6,
    0x996E: 0x51A2,
    0x996F: 0x51A9,
    0x9970: 0x51AA,
    0x9971: 0x51AB,
    0x9972: 0x51B3,
    0x9973: 0x51B1,
    0x9974: 0x51B2,
    0x9975: 0x51B0,
    0x9976: 0x51B5,
    0x9977: 0x51BD,
    0x9978: 0x51C5,
    0x9979: 0x51C9,
    0x997A: 0x51DB,
    0x997B: 0x51E0,
    0x997C: 0x8655,
    0x997D: 0x51E9,
    0x997E: 0x51ED,
    0x9980: 0x51F0,
    0x9981: 0x51F5,
    0x9982: 0x51FE,
    0x9983: 0x5204,
    0x9984: 0x520B,
    0x9985: 0x5214,
    0x9986: 0x520E,
    0x9987: 0x5227,
    0x9988: 0x522A,
    0x9989: 0x522E,
    0x998A: 0x5233,
    0x998B: 0x5239,
    0x998C: 0x524F,
    0x998D: 0x5244,
    0x998E: 0x524B,
    0x998F: 0x524C,
    0x9990: 0x525E,
    0x9991: 0x5254,
    0x9992: 0x526A,
    0x9993: 0x5274,
    0x9994: 0x5269,
    0x9995: 0x5273,
    0x9996: 0x527F,
    0x9997: 0x527D,
    0x9998: 0x528D,
    0x9999: 0x5294,
    0x999A: 0x5292,
    0x999B: 0x5271,
    0x999C: 0x5288,
    0x999D: 0x5291,
    0x999E: 0x8FA8,
    0x999F: 0x8FA7,
    0x99A0: 0x52AC,
    0x99A1: 0x52AD,
    0x99A2: 0x52BC,
    0x99A3: 0x52B5,
    0x99A4: 0x52C1,
    0x99A5: 0x52CD,
    0x99A6: 0x52D7,
    0x99A7: 0x52DE,
    0x99A8: 0x52E3,
    0x99A9: 0x52E6,
    0x99AA: 0x98ED,
    0x99AB: 0x52E0,
    0x99AC: 0x52F3,
    0x99AD: 0x52F5,
    0x99AE: 0x52F8,
    0x99AF: 0x52F9,
    0x99B0: 0x5306,
    0x99B1: 0x5308,
    0x99B2: 0x7538,
    0x99B3: 0x530D,
    0x99B4: 0x5310,
    0x99B5: 0x530F,
    0x99B6: 0x5315,
    0x99B7: 0x531A,
    0x99B8: 0x5323,
    0x99B9: 0x532F,
    0x99BA: 0x5331,
    0x99BB: 0x5333,
    0x99BC: 0x5338,
    0x99BD: 0x5340,
    0x99BE: 0x5346,
    0x99BF: 0x5345,
    0x99C0: 0x4E17,
    0x99C1: 0x5349,
    0x99C2: 0x534D,
    0x99C3: 0x51D6,
    0x99C4: 0x535E,
    0x99C5: 0x5369,
    0x99C6: 0x536E,
    0x99C7: 0x5918,
    0x99C8: 0x537B,
    0x99C9: 0x5377,
    0x99CA: 0x5382,
    0x99CB: 0x5396,
    0x99CC: 0x53A0,
    0x99CD: 0x53A6,
    0x99CE: 0x53A5,
    0x99CF: 0x53AE,
    0x99D0: 0x53B0,
    0x99D1: 0x53B6,
    0x99D2: 0x53C3,
    0x99D3: 0x7C12,
    0x99D4: 0x96D9,
    0x99D5: 0x53DF,
    0x99D6: 0x66FC,
    0x99D7: 0x71EE,
    0x99D8: 0x53EE,
    0x99D9: 0x53E8,
    0x99DA: 0x53ED,
    0x99DB: 0x53FA,
    0x99DC: 0x5401,
    0x99DD: 0x543D,
    0x99DE: 0x5440,
    0x99DF: 0x542C,
    0x99E0: 0x542D,
    0x99E1: 0x543C,
    0x99E2: 0x542E,
    0x99E3: 0x5436,
    0x99E4: 0x5429,
    0x99E5: 0x541D,
    0x99E6: 0x544E,
    0x99E7: 0x548F,
    0x99E8: 0x5475,
    0x99E9: 0x548E,
    0x99EA: 0x545F,
    0x99EB: 0x5471,
    0x99EC: 0x5477,
    0x99ED: 0x5470,
    0x99EE: 0x5492,
    0x99EF: 0x547B,
    0x99F0: 0x5480,
    0x99F1: 0x5476,
    0x99F2: 0x5484,
    0x99F3: 0x5490,
    0x99F4: 0x5486,
    0x99F5: 0x54C7,
    0x99F6: 0x54A2,
    0x99F7: 0x54B8,
    0x99F8: 0x54A5,
    0x99F9: 0x54AC,
    0x99FA: 0x54C4,
    0x99FB: 0x54C8,
    0x99FC: 0x54A8,
    0x9A40: 0x54AB,
    0x9A41: 0x54C2,
    0x9A42: 0x54A4,
    0x9A43: 0x54BE,
    0x9A44: 0x54BC,
    0x9A45: 0x54D8,
    0x9A46: 0x54E5,
    0x9A47: 0x54E6,
    0x9A48: 0x550F,
    0x9A49: 0x5514,
    0x9A4A: 0x54FD,
    0x9A4B: 0x54EE,
    0x9A4C: 0x54ED,
    0x9A4D: 0x54FA,
    0x9A4E: 0x54E2,
    0x9A4F: 0x5539,
    0x9A50: 0x5540,
    0x9A51: 0x5563,
    0x9A52: 0x554C,
    0x9A53: 0x552E,
    0x9A54: 0x555C,
    0x9A55: 0x5545,
    0x9A56: 0x5556,
    0x9A57: 0x5557,
    0x9A58: 0x5538,
    0x9A59: 0x5533,
    0x9A5A: 0x555D,
    0x9A5B: 0x5599,
    0x9A5C: 0x5580,
    0x9A5D: 0x54AF,
    0x9A5E: 0x558A,
    0x9A5F: 0x559F,
    0x9A60: 0x557B,
    0x9A61: 0x557E,
    0x9A62: 0x5598,
    0x9A63: 0x559E,
    0x9A64: 0x55AE,
    0x9A65: 0x557C,
    0x9A66: 0x5583,
    0x9A67: 0x55A9,
    0x9A68: 0x5587,
    0x9A69: 0x55A8,
    0x9A6A: 0x55DA,
    0x9A6B: 0x55C5,
    0x9A6C: 0x55DF,
    0x9A6D: 0x55C4,
    0x9A6E: 0x55DC,
    0x9A6F: 0x55E4,
    0x9A70: 0x55D4,
    0x9A71: 0x5614,
    0x9A72: 0x55F7,
    0x9A73: 0x5616,
    0x9A74: 0x55FE,
    0x9A75: 0x55FD,
    0x9A76: 0x561B,
    0x9A77: 0x55F9,
    0x9A78: 0x564E,
    0x9A79: 0x5650,
    0x9A7A: 0x71DF,
    0x9A7B: 0x5634,
    0x9A7C: 0x5636,
    0x9A7D: 0x5632,
    0x9A7E: 0x5638,
    0x9A80: 0x566B,
    0x9A81: 0x5664,
    0x9A82: 0x562F,
    0x9A83: 0x566C,
    0x9A84: 0x566A,
    0x9A85: 0x5686,
    0x9A86: 0x5680,
    0x9A87: 0x568A,
    0x9A88: 0x56A0,
    0x9A89: 0x5694,
    0x9A8A: 0x568F,
    0x9A8B: 0x56A5,
    0x9A8C: 0x56AE,
    0x9A8D: 0x56B6,
    0x9A8E: 0x56B4,
    0x9A8F: 0x56C2,
    0x9A90: 0x56BC,
    0x9A91: 0x56C1,
    0x9A92: 0x56C3,
    0x9A93: 0x56C0,
    0x9A94: 0x56C8,
    0x9A95: 0x56CE,
    0x9A96: 0x56D1,
    0x9A97: 0x56D3,
    0x9A98: 0x56D7,
    0x9A99: 0x56EE,
    0x9A9A: 0x56F9,
    0x9A9B: 0x5700,
    0x9A9C: 0x56FF,
    0x9A9D: 0x5704,
    0x9A9E: 0x5709,
    0x9A9F: 0x5708,
    0x9AA0: 0x570B,
    0x9AA1: 0x570D,
    0x9AA2: 0x5713,
    0x9AA3: 0x5718,
    0x9AA4: 0x5716,
    0x9AA5: 0x55C7,
    0x9AA6: 0x571C,
    0x9AA7: 0x5726,
    0x9AA8: 0x5737,
    0x9AA9: 0x5738,
    0x9AAA: 0x574E,
    0x9AAB: 0x573B,
    0x9AAC: 0x5740,
    0x9AAD: 0x574F,
    0x9AAE: 0x5769,
    0x9AAF: 0x57C0,
    0x9AB0: 0x5788,
    0x9AB1: 0x5761,
    0x9AB2: 0x577F,
    0x9AB3: 0x5789,
    0x9AB4: 0x5793,
    0x9AB5: 0x57A0,
    0x9AB6: 0x57B3,
    0x9AB7: 0x57A4,
    0x9AB8: 0x57AA,
    0x9AB9: 0x57B0,
    0x9ABA: 0x57C3,
    0x9ABB: 0x57C6,
    0x9ABC: 0x57D4,
    0x9ABD: 0x57D2,
    0x9ABE: 0x57D3,
    0x9ABF: 0x580A,
    0x9AC0: 0x57D6,
    0x9AC1: 0x57E3,
    0x9AC2: 0x580B,
    0x9AC3: 0x5819,
    0x9AC4: 0x581D,
    0x9AC5: 0x5872,
    0x9AC6: 0x5821,
    0x9AC7: 0x5862,
    0x9AC8: 0x584B,
    0x9AC9: 0x5870,
    0x9ACA: 0x6BC0,
    0x9ACB: 0x5852,
    0x9ACC: 0x583D,
    0x9ACD: 0x5879,
    0x9ACE: 0x5885,
    0x9ACF: 0x58B9,
    0x9AD0: 0x589F,
    0x9AD1: 0x58AB,
    0x9AD2: 0x58BA,
    0x9AD3: 0x58DE,
    0x9AD4: 0x58BB,
    0x9AD5: 0x58B8,
    0x9AD6: 0x58AE,
    0x9AD7: 0x58C5,
    0x9AD8: 0x58D3,
    0x9AD9: 0x58D1,
    0x9ADA: 0x58D7,
    0x9ADB: 0x58D9,
    0x9ADC: 0x58D8,
    0x9ADD: 0x58E5,
    0x9ADE: 0x58DC,
    0x9ADF: 0x58E4,
    0x9AE0: 0x58DF,
    0x9AE1: 0x58EF,
    0x9AE2: 0x58FA,
    0x9AE3: 0x58F9,
    0x9AE4: 0x58FB,
    0x9AE5: 0x58FC,
    0x9AE6: 0x58FD,
    0x9AE7: 0x5902,
    0x9AE8: 0x590A,
    0x9AE9: 0x5910,
    0x9AEA: 0x591B,
    0x9AEB: 0x68A6,
    0x9AEC: 0x5925,
    0x9AED: 0x592C,
    0x9AEE: 0x592D,
    0x9AEF: 0x5932,
    0x9AF0: 0x5938,
    0x9AF1: 0x593E,
    0x9AF2: 0x7AD2,
    0x9AF3: 0x5955,
    0x9AF4: 0x5950,
    0x9AF5: 0x594E,
    0x9AF6: 0x595A,
    0x9AF7: 0x5958,
    0x9AF8: 0x5962,
    0x9AF9: 0x5960,
    0x9AFA: 0x5967,
    0x9AFB: 0x596C,
    0x9AFC: 0x5969,
    0x9B40: 0x5978,
    0x9B41: 0x5981,
    0x9B42: 0x599D,
    0x9B43: 0x4F5E,
    0x9B44: 0x4FAB,
    0x9B45: 0x59A3,
    0x9B46: 0x59B2,
    0x9B47: 0x59C6,
    0x9B48: 0x59E8,
    0x9B49: 0x59DC,
    0x9B4A: 0x598D,
    0x9B4B: 0x59D9,
    0x9B4C: 0x59DA,
    0x9B4D: 0x5A25,
    0x9B4E: 0x5A1F,
    0x9B4F: 0x5A11,
    0x9B50: 0x5A1C,
    0x9B51: 0x5A09,
    0x9B52: 0x5A1A,
    0x9B53: 0x5A40,
    0x9B54: 0x5A6C,
    0x9B55: 0x5A49,
    0x9B56: 0x5A35,
    0x9B57: 0x5A36,
    0x9B58: 0x5A62,
    0x9B59: 0x5A6A,
    0x9B5A: 0x5A9A,
    0x9B5B: 0x5ABC,
    0x9B5C: 0x5ABE,
    0x9B5D: 0x5ACB,
    0x9B5E: 0x5AC2,
    0x9B5F: 0x5ABD,
    0x9B60: 0x5AE3,
    0x9B61: 0x5AD7,
    0x9B62: 0x5AE6,
    0x9B63: 0x5AE9,
    0x9B64: 0x5AD6,
    0x9B65: 0x5AFA,
    0x9B66: 0x5AFB,
    0x9B67: 0x5B0C,
    0x9B68: 0x5B0B,
    0x9B69: 0x5B16,
    0x9B6A: 0x5B32,
    0x9B6B: 0x5AD0,
    0x9B6C: 0x5B2A,
    0x9B6D: 0x5B36,
    0x9B6E: 0x5B3E,
    0x9B6F: 0x5B43,
    0x9B70: 0x5B45,
    0x9B71: 0x5B40,
    0x9B72: 0x5B51,
    0x9B73: 0x5B55,
    0x9B74: 0x5B5A,
    0x9B75: 0x5B5B,
    0x9B76: 0x5B65,
    0x9B77: 0x5B69,
    0x9B78: 0x5B70,
    0x9B79: 0x5B73,
    0x9B7A: 0x5B75,
    0x9B7B: 0x5B78,
    0x9B7C: 0x6588,
    0x9B7D: 0x5B7A,
    0x9B7E: 0x5B80,
    0x9B80: 0x5B83,
    0x9B81: 0x5BA6,
    0x9B82: 0x5BB8,
    0x9B83: 0x5BC3,
    0x9B84: 0x5BC7,
    0x9B85: 0x5BC9,
    0x9B86: 0x5BD4,
    0x9B87: 0x5BD0,
    0x9B88: 0x5BE4,
    0x9B89: 0x5BE6,
    0x9B8A: 0x5BE2,
    0x9B8B: 0x5BDE,
    0x9B8C: 0x5BE5,
    0x9B8D: 0x5BEB,
    0x9B8E: 0x5BF0,
    0x9B8F: 0x5BF6,
    0x9B90: 0x5BF3,
    0x9B91: 0x5C05,
    0x9B92: 0x5C07,
    0x9B93: 0x5C08,
    0x9B94: 0x5C0D,
    0x9B95: 0x5C13,
    0x9B96: 0x5C20,
    0x9B97: 0x5C22,
    0x9B98: 0x5C28,
    0x9B99: 0x5C38,
    0x9B9A: 0x5C39,
    0x9B9B: 0x5C41,
    0x9B9C: 0x5C46,
    0x9B9D: 0x5C4E,
    0x9B9E: 0x5C53,
    0x9B9F: 0x5C50,
    0x9BA0: 0x5C4F,
    0x9BA1: 0x5B71,
    0x9BA2: 0x5C6C,
    0x9BA3: 0x5C6E,
    0x9BA4: 0x4E62,
    0x9BA5: 0x5C76,
    0x9BA6: 0x5C79,
    0x9BA7: 0x5C8C,
    0x9BA8: 0x5C91,
    0x9BA9: 0x5C94,
    0x9BAA: 0x599B,
    0x9BAB: 0x5CAB,
    0x9BAC: 0x5CBB,
    0x9BAD: 0x5CB6,
    0x9BAE: 0x5CBC,
    0x9BAF: 0x5CB7,
    0x9BB0: 0x5CC5,
    0x9BB1: 0x5CBE,
    0x9BB2: 0x5CC7,
    0x9BB3: 0x5CD9,
    0x9BB4: 0x5CE9,
    0x9BB5: 0x5CFD,
    0x9BB6: 0x5CFA,
    0x9BB7: 0x5CED,
    0x9BB8: 0x5D8C,
    0x9BB9: 0x5CEA,
    0x9BBA: 0x5D0B,
    0x9BBB: 0x5D15,
    0x9BBC: 0x5D17,
    0x9BBD: 0x5D5C,
    0x9BBE: 0x5D1F,
    0x9BBF: 0x5D1B,
    0x9BC0: 0x5D11,
    0x9BC1: 0x5D14,
    0x9BC2: 0x5D22,
    0x9BC3: 0x5D1A,
    0x9BC4: 0x5D19,
    0x9BC5: 0x5D18,
    0x9BC6: 0x5D4C,
    0x9BC7: 0x5D52,
    0x9BC8: 0x5D4E,
    0x9BC9: 0x5D4B,
    0x9BCA: 0x5D6C,
    0x9BCB: 0x5D73,
    0x9BCC: 0x5D76,
    0x9BCD: 0x5D87,
    0x9BCE: 0x5D84,
    0x9BCF: 0x5D82,
    0x9BD0: 0x5DA2,
    0x9BD1: 0x5D9D,
    0x9BD2: 0x5DAC,
    0x9BD3: 0x5DAE,
    0x9BD4: 0x5DBD,
    0x9BD5: 0x5D90,
    0x9BD6: 0x5DB7,
    0x9BD7: 0x5DBC,
    0x9BD8: 0x5DC9,
    0x9BD9: 0x5DCD,
    0x9BDA: 0x5DD3,
    0x9BDB: 0x5DD2,
    0x9BDC: 0x5DD6,
    0x9BDD: 0x5DDB,
    0x9BDE: 0x5DEB,
    0x9BDF: 0x5DF2,
    0x9BE0: 0x5DF5,
    0x9BE1: 0x5E0B,
    0x9BE2: 0x5E1A,
    0x9BE3: 0x5E19,
    0x9BE4: 0x5E11,
    0x9BE5: 0x5E1B,
    0x9BE6: 0x5E36,
    0x9BE7: 0x5E37,
    0x9BE8: 0x5E44,
    0x9BE9: 0x5E43,
    0x9BEA: 0x5E40,
    0x9BEB: 0x5E4E,
    0x9BEC: 0x5E57,
    0x9BED: 0x5E54,
    0x9BEE: 0x5E5F,
    0x9BEF: 0x5E62,
    0x9BF0: 0x5E64,
    0x9BF1: 0x5E47,
    0x9BF2: 0x5E75,
    0x9BF3: 0x5E76,
    0x9BF4: 0x5E7A,
    0x9BF5: 0x9EBC,
    0x9BF6: 0x5E7F,
    0x9BF7: 0x5EA0,
    0x9BF8: 0x5EC1,
    0x9BF9: 0x5EC2,
    0x9BFA: 0x5EC8,
    0x9BFB: 0x5ED0,
    0x9BFC: 0x5ECF,
    0x9C40: 0x5ED6,
    0x9C41: 0x5EE3,
    0x9C42: 0x5EDD,
    0x9C43: 0x5EDA,
    0x9C44: 0x5EDB,
    0x9C45: 0x5EE2,
    0x9C46: 0x5EE1,
    0x9C47: 0x5EE8,
    0x9C48: 0x5EE9,
    0x9C49: 0x5EEC,
    0x9C4A: 0x5EF1,
    0x9C4B: 0x5EF3,
    0x9C4C: 0x5EF0,
    0x9C4D: 0x5EF4,
    0x9C4E: 0x5EF8,
    0x9C4F: 0x5EFE,
    0x9C50: 0x5F03,
    0x9C51: 0x5F09,
    0x9C52: 0x5F5D,
    0x9C53: 0x5F5C,
    0x9C54: 0x5F0B,
    0x9C55: 0x5F11,
    0x9C56: 0x5F16,
    0x9C57: 0x5F29,
    0x9C58: 0x5F2D,
    0x9C59: 0x5F38,
    0x9C5A: 0x5F41,
    0x9C5B: 0x5F48,
    0x9C5C: 0x5F4C,
    0x9C5D: 0x5F4E,
    0x9C5E: 0x5F2F,
    0x9C5F: 0x5F51,
    0x9C60: 0x5F56,
    0x9C61: 0x5F57,
    0x9C62: 0x5F59,
    0x9C63: 0x5F61,
    0x9C64: 0x5F6D,
    0x9C65: 0x5F73,
    0x9C66: 0x5F77,
    0x9C67: 0x5F83,
    0x9C68: 0x5F82,
    0x9C69: 0x5F7F,
    0x9C6A: 0x5F8A,
    0x9C6B: 0x5F88,
    0x9C6C: 0x5F91,
    0x9C6D: 0x5F87,
    0x9C6E: 0x5F9E,
    0x9C6F: 0x5F99,
    0x9C70: 0x5F98,
    0x9C71: 0x5FA0,
    0x9C72: 0x5FA8,
    0x9C73: 0x5FAD,
    0x9C74: 0x5FBC,
    0x9C75: 0x5FD6,
    0x9C76: 0x5FFB,
    0x9C77: 0x5FE4,
    0x9C78: 0x5FF8,
    0x9C79: 0x5FF1,
    0x9C7A: 0x5FDD,
    0x9C7B: 0x60B3,
    0x9C7C: 0x5FFF,
    0x9C7D: 0x6021,
    0x9C7E: 0x6060,
    0x9C80: 0x6019,
    0x9C81: 0x6010,
    0x9C82: 0x6029,
    0x9C83: 0x600E,
    0x9C84: 0x6031,
    0x9C85: 0x601B,
    0x9C86: 0x6015,
    0x9C87: 0x602B,
    0x9C88: 0x6026,
    0x9C89: 0x600F,
    0x9C8A: 0x603A,
    0x9C8B: 0x605A,
    0x9C8C: 0x6041,
    0x9C8D: 0x606A,
    0x9C8E: 0x6077,
    0x9C8F: 0x605F,
    0x9C90: 0x604A,
    0x9C91: 0x6046,
    0x9C92: 0x604D,
    0x9C93: 0x6063,
    0x9C94: 0x6043,
    0x9C95: 0x6064,
    0x9C96: 0x6042,
    0x9C97: 0x606C,
    0x9C98: 0x606B,
    0x9C99: 0x6059,
    0x9C9A: 0x6081,
    0x9C9B: 0x608D,
    0x9C9C: 0x60E7,
    0x9C9D: 0x6083,
    0x9C9E: 0x609A,
    0x9C9F: 0x6084,
    0x9CA0: 0x609B,
    0x9CA1: 0x6096,
    0x9CA2: 0x6097,
    0x9CA3: 0x6092,
    0x9CA4: 0x60A7,
    0x9CA5: 0x608B,
    0x9CA6: 0x60E1,
    0x9CA7: 0x60B8,
    0x9CA8: 0x60E0,
    0x9CA9: 0x60D3,
    0x9CAA: 0x60B4,
    0x9CAB: 0x5FF0,
    0x9CAC: 0x60BD,
    0x9CAD: 0x60C6,
    0x9CAE: 0x60B5,
    0x9CAF: 0x60D8,
    0x9CB0: 0x614D,
    0x9CB1: 0x6115,
    0x9CB2: 0x6106,
    0x9CB3: 0x60F6,
    0x9CB4: 0x60F7,
    0x9CB5: 0x6100,
    0x9CB6: 0x60F4,
    0x9CB7: 0x60FA,
    0x9CB8: 0x6103,
    0x9CB9: 0x6121,
    0x9CBA: 0x60FB,
    0x9CBB: 0x60F1,
    0x9CBC: 0x610D,
    0x9CBD: 0x610E,
    0x9CBE: 0x6147,
    0x9CBF: 0x613E,
    0x9CC0: 0x6128,
    0x9CC1: 0x6127,
    0x9CC2: 0x614A,
    0x9CC3: 0x613F,
    0x9CC4: 0x613C,
    0x9CC5: 0x612C,
    0x9CC6: 0x6134,
    0x9CC7: 0x613D,
    0x9CC8: 0x6142,
    0x9CC9: 0x6144,
    0x9CCA: 0x6173,
    0x9CCB: 0x6177,
    0x9CCC: 0x6158,
    0x9CCD: 0x6159,
    0x9CCE: 0x615A,
    0x9CCF: 0x616B,
    0x9CD0: 0x6174,
    0x9CD1: 0x616F,
    0x9CD2: 0x6165,
    0x9CD3: 0x6171,
    0x9CD4: 0x615F,
    0x9CD5: 0x615D,
    0x9CD6: 0x6153,
    0x9CD7: 0x6175,
    0x9CD8: 0x6199,
    0x9CD9: 0x6196,
    0x9CDA: 0x6187,
    0x9CDB: 0x61AC,
    0x9CDC: 0x6194,
    0x9CDD: 0x619A,
    0x9CDE: 0x618A,
    0x9CDF: 0x6191,
    0x9CE0: 0x61AB,
    0x9CE1: 0x61AE,
    0x9CE2: 0x61CC,
    0x9CE3: 0x61CA,
    0x9CE4: 0x61C9,
    0x9CE5: 0x61F7,
    0x9CE6: 0x61C8,
    0x9CE7: 0x61C3,
    0x9CE8: 0x61C6,
    0x9CE9: 0x61BA,
    0x9CEA: 0x61CB,
    0x9CEB: 0x7F79,
    0x9CEC: 0x61CD,
    0x9CED: 0x61E6,
    0x9CEE: 0x61E3,
    0x9CEF: 0x61F6,
    0x9CF0: 0x61FA,
    0x9CF1: 0x61F4,
    0x9CF2: 0x61FF,
    0x9CF3: 0x61FD,
    0x9CF4: 0x61FC,
    0x9CF5: 0x61FE,
    0x9CF6: 0x6200,
    0x9CF7: 0x6208,
    0x9CF8: 0x6209,
    0x9CF9: 0x620D,
    0x9CFA: 0x620C,
    0x9CFB: 0x6214,
    0x9CFC: 0x621B,
    0x9D40: 0x621E,
    0x9D41: 0x6221,
    0x9D42: 0x622A,
    0x9D43: 0x622E,
    0x9D44: 0x6230,
    0x9D45: 0x6232,
    0x9D46: 0x6233,
    0x9D47: 0x6241,
    0x9D48: 0x624E,
    0x9D49: 0x625E,
    0x9D4A: 0x6263,
    0x9D4B: 0x625B,
    0x9D4C: 0x6260,
    0x9D4D: 0x6268,
    0x9D4E: 0x627C,
    0x9D4F: 0x6282,
    0x9D50: 0x6289,
    0x9D51: 0x627E,
    0x9D52: 0x6292,
    0x9D53: 0x6293,
    0x9D54: 0x6296,
    0x9D55: 0x62D4,
    0x9D56: 0x6283,
    0x9D57: 0x6294,
    0x9D58: 0x62D7,
    0x9D59: 0x62D1,
    0x9D5A: 0x62BB,
    0x9D5B: 0x62CF,
    0x9D5C: 0x62FF,
    0x9D5D: 0x62C6,
    0x9D5E: 0x64D4,
    0x9D5F: 0x62C8,
    0x9D60: 0x62DC,
    0x9D61: 0x62CC,
    0x9D62: 0x62CA,
    0x9D63: 0x62C2,
    0x9D64: 0x62C7,
    0x9D65: 0x629B,
    0x9D66: 0x62C9,
    0x9D67: 0x630C,
    0x9D68: 0x62EE,
    0x9D69: 0x62F1,
    0x9D6A: 0x6327,
    0x9D6B: 0x6302,
    0x9D6C: 0x6308,
    0x9D6D: 0x62EF,
    0x9D6E: 0x62F5,
    0x9D6F: 0x6350,
    0x9D70: 0x633E,
    0x9D71: 0x634D,
    0x9D72: 0x641C,
    0x9D73: 0x634F,
    0x9D74: 0x6396,
    0x9D75: 0x638E,
    0x9D76: 0x6380,
    0x9D77: 0x63AB,
    0x9D78: 0x6376,
    0x9D79: 0x63A3,
    0x9D7A: 0x638F,
    0x9D7B: 0x6389,
    0x9D7C: 0x639F,
    0x9D7D: 0x63B5,
    0x9D7E: 0x636B,
    0x9D80: 0x6369,
    0x9D81: 0x63BE,
    0x9D82: 0x63E9,
    0x9D83: 0x63C0,
    0x9D84: 0x63C6,
    0x9D85: 0x63E3,
    0x9D86: 0x63C9,
    0x9D87: 0x63D2,
    0x9D88: 0x63F6,
    0x9D89: 0x63C4,
    0x9D8A: 0x6416,
    0x9D8B: 0x6434,
    0x9D8C: 0x6406,
    0x9D8D: 0x6413,
    0x9D8E: 0x6426,
    0x9D8F: 0x6436,
    0x9D90: 0x651D,
    0x9D91: 0x6417,
    0x9D92: 0x6428,
    0x9D93: 0x640F,
    0x9D94: 0x6467,
    0x9D95: 0x646F,
    0x9D96: 0x6476,
    0x9D97: 0x644E,
    0x9D98: 0x652A,
    0x9D99: 0x6495,
    0x9D9A: 0x6493,
    0x9D9B: 0x64A5,
    0x9D9C: 0x64A9,
    0x9D9D: 0x6488,
    0x9D9E: 0x64BC,
    0x9D9F: 0x64DA,
    0x9DA0: 0x64D2,
    0x9DA1: 0x64C5,
    0x9DA2: 0x64C7,
    0x9DA3: 0x64BB,
    0x9DA4: 0x64D8,
    0x9DA5: 0x64C2,
    0x9DA6: 0x64F1,
    0x9DA7: 0x64E7,
    0x9DA8: 0x8209,
    0x9DA9: 0x64E0,
    0x9DAA: 0x64E1,
    0x9DAB: 0x62AC,
    0x9DAC: 0x64E3,
    0x9DAD: 0x64EF,
    0x9DAE: 0x652C,
    0x9DAF: 0x64F6,
    0x9DB0: 0x64F4,
    0x9DB1: 0x64F2,
    0x9DB2: 0x64FA,
    0x9DB3: 0x6500,
    0x9DB4: 0x64FD,
    0x9DB5: 0x6518,
    0x9DB6: 0x651C,
    0x9DB7: 0x6505,
    0x9DB8: 0x6524,
    0x9DB9: 0x6523,
    0x9DBA: 0x652B,
    0x9DBB: 0x6534,
    0x9DBC: 0x6535,
    0x9DBD: 0x6537,
    0x9DBE: 0x6536,
    0x9DBF: 0x6538,
    0x9DC0: 0x754B,
    0x9DC1: 0x6548,
    0x9DC2: 0x6556,
    0x9DC3: 0x6555,
    0x9DC4: 0x654D,
    0x9DC5: 0x6558,
    0x9DC6: 0x655E,
    0x9DC7: 0x655D,
    0x9DC8: 0x6572,
    0x9DC9: 0x6578,
    0x9DCA: 0x6582,
    0x9DCB: 0x6583,
    0x9DCC: 0x8B8A,
    0x9DCD: 0x659B,
    0x9DCE: 0x659F,
    0x9DCF: 0x65AB,
    0x9DD0: 0x65B7,
    0x9DD1: 0x65C3,
    0x9DD2: 0x65C6,
    0x9DD3: 0x65C1,
    0x9DD4: 0x65C4,
    0x9DD5: 0x65CC,
    0x9DD6: 0x65D2,
    0x9DD7: 0x65DB,
    0x9DD8: 0x65D9,
    0x9DD9: 0x65E0,
    0x9DDA: 0x65E1,
    0x9DDB: 0x65F1,
    0x9DDC: 0x6772,
    0x9DDD: 0x660A,
    0x9DDE: 0x6603,
    0x9DDF: 0x65FB,
    0x9DE0: 0x6773,
    0x9DE1: 0x6635,
    0x9DE2: 0x6636,
    0x9DE3: 0x6634,
    0x9DE4: 0x661C,
    0x9DE5: 0x664F,
    0x9DE6: 0x6644,
    0x9DE7: 0x6649,
    0x9DE8: 0x6641,
    0x9DE9: 0x665E,
    0x9DEA: 0x665D,
    0x9DEB: 0x6664,
    0x9DEC: 0x6667,
    0x9DED: 0x6668,
    0x9DEE: 0x665F,
    0x9DEF: 0x6662,
    0x9DF0: 0x6670,
    0x9DF1: 0x6683,
    0x9DF2: 0x6688,
    0x9DF3: 0x668E,
    0x9DF4: 0x6689,
    0x9DF5: 0x6684,
    0x9DF6: 0x6698,
    0x9DF7: 0x669D,
    0x9DF8: 0x66C1,
    0x9DF9: 0x66B9,
    0x9DFA: 0x66C9,
    0x9DFB: 0x66BE,
    0x9DFC: 0x66BC,
    0x9E40: 0x66C4,
    0x9E41: 0x66B8,
    0x9E42: 0x66D6,
    0x9E43: 0x66DA,
    0x9E44: 0x66E0,
    0x9E45: 0x663F,
    0x9E46: 0x66E6,
    0x9E47: 0x66E9,
    0x9E48: 0x66F0,
    0x9E49: 0x66F5,
    0x9E4A: 0x66F7,
    0x9E4B: 0x670F,
    0x9E4C: 0x6716,
    0x9E4D: 0x671E,
    0x9E4E: 0x6726,
    0x9E4F: 0x6727,
    0x9E50: 0x9738,
    0x9E51: 0x672E,
    0x9E52: 0x673F,
    0x9E53: 0x6736,
    0x9E54: 0x6741,
    0x9E55: 0x6738,
    0x9E56: 0x6737,
    0x9E57: 0x6746,
    0x9E58: 0x675E,
    0x9E59: 0x6760,
    0x9E5A: 0x6759,
    0x9E5B: 0x6763,
    0x9E5C: 0x6764,
    0x9E5D: 0x6789,
    0x9E5E: 0x6770,
    0x9E5F: 0x67A9,
    0x9E60: 0x677C,
    0x9E61: 0x676A,
    0x9E62: 0x678C,
    0x9E63: 0x678B,
    0x9E64: 0x67A6,
    0x9E65: 0x67A1,
    0x9E66: 0x6785,
    0x9E67: 0x67B7,
    0x9E68: 0x67EF,
    0x9E69: 0x67B4,
    0x9E6A: 0x67EC,
    0x9E6B: 0x67B3,
    0x9E6C: 0x67E9,
    0x9E6D: 0x67B8,
    0x9E6E: 0x67E4,
    0x9E6F: 0x67DE,
    0x9E70: 0x67DD,
    0x9E71: 0x67E2,
    0x9E72: 0x67EE,
    0x9E73: 0x67B9,
    0x9E74: 0x67CE,
    0x9E75: 0x67C6,
    0x9E76: 0x67E7,
    0x9E77: 0x6A9C,
    0x9E78: 0x681E,
    0x9E79: 0x6846,
    0x9E7A: 0x6829,
    0x9E7B: 0x6840,
    0x9E7C: 0x684D,
    0x9E7D: 0x6832,
    0x9E7E: 0x684E,
    0x9E80: 0x68B3,
    0x9E81: 0x682B,
    0x9E82: 0x6859,
    0x9E83: 0x6863,
    0x9E84: 0x6877,
    0x9E85: 0x687F,
    0x9E86: 0x689F,
    0x9E87: 0x688F,
    0x9E88: 0x68AD,
    0x9E89: 0x6894,
    0x9E8A: 0x689D,
    0x9E8B: 0x689B,
    0x9E8C: 0x6883,
    0x9E8D: 0x6AAE,
    0x9E8E: 0x68B9,
    0x9E8F: 0x6874,
    0x9E90: 0x68B5,
    0x9E91: 0x68A0,
    0x9E92: 0x68BA,
    0x9E93: 0x690F,
    0x9E94: 0x688D,
    0x9E95: 0x687E,
    0x9E96: 0x6901,
    0x9E97: 0x68CA,
    0x9E98: 0x6908,
    0x9E99: 0x68D8,
    0x9E9A: 0x6922,
    0x9E9B: 0x6926,
    0x9E9C: 0x68E1,
    0x9E9D: 0x690C,
    0x9E9E: 0x68CD,
    0x9E9F: 0x68D4,
    0x9EA0: 0x68E7,
    0x9EA1: 0x68D5,
    0x9EA2: 0x6936,
    0x9EA3: 0x6912,
    0x9EA4: 0x6904,
    0x9EA5: 0x68D7,
    0x9EA6: 0x68E3,
    0x9EA7: 0x6925,
    0x9EA8: 0x68F9,
    0x9EA9: 0x68E0,
    0x9EAA: 0x68EF,
    0x9EAB: 0x6928,
    0x9EAC: 0x692A,
    0x9EAD: 0x691A,
    0x9EAE: 0x6923,
    0x9EAF: 0x6921,
    0x9EB0: 0x68C6,
    0x9EB1: 0x6979,
    0x9EB2: 0x6977,
    0x9EB3: 0x695C,
    0x9EB4: 0x6978,
    0x9EB5: 0x696B,
    0x9EB6: 0x6954,
    0x9EB7: 0x697E,
    0x9EB8: 0x696E,
    0x9EB9: 0x6939,
    0x9EBA: 0x6974,
    0x9EBB: 0x693D,
    0x9EBC: 0x6959,
    0x9EBD: 0x6930,
    0x9EBE: 0x6961,
    0x9EBF: 0x695E,
    0x9EC0: 0x695D,
    0x9EC1: 0x6981,
    0x9EC2: 0x696A,
    0x9EC3: 0x69B2,
    0x9EC4: 0x69AE,
    0x9EC5: 0x69D0,
    0x9EC6: 0x69BF,
    0x9EC7: 0x69C1,
    0x9EC8: 0x69D3,
    0x9EC9: 0x69BE,
    0x9ECA: 0x69CE,
    0x9ECB: 0x5BE8,
    0x9ECC: 0x69CA,
    0x9ECD: 0x69DD,
    0x9ECE: 0x69BB,
    0x9ECF: 0x69C3,
    0x9ED0: 0x69A7,
    0x9ED1: 0x6A2E,
    0x9ED2: 0x6991,
    0x9ED3: 0x69A0,
    0x9ED4: 0x699C,
    0x9ED5: 0x6995,
    0x9ED6: 0x69B4,
    0x9ED7: 0x69DE,
    0x9ED8: 0x69E8,
    0x9ED9: 0x6A02,
    0x9EDA: 0x6A1B,
    0x9EDB: 0x69FF,
    0x9EDC: 0x6B0A,
    0x9EDD: 0x69F9,
    0x9EDE: 0x69F2,
    0x9EDF: 0x69E7,
    0x9EE0: 0x6A05,
    0x9EE1: 0x69B1,
    0x9EE2: 0x6A1E,
    0x9EE3: 0x69ED,
    0x9EE4: 0x6A14,
    0x9EE5: 0x69EB,
    0x9EE6: 0x6A0A,
    0x9EE7: 0x6A12,
    0x9EE8: 0x6AC1,
    0x9EE9: 0x6A23,
    0x9EEA: 0x6A13,
    0x9EEB: 0x6A44,
    0x9EEC: 0x6A0C,
    0x9EED: 0x6A72,
    0x9EEE: 0x6A36,
    0x9EEF: 0x6A78,
    0x9EF0: 0x6A47,
    0x9EF1: 0x6A62,
    0x9EF2: 0x6A59,
    0x9EF3: 0x6A66,
    0x9EF4: 0x6A48,
    0x9EF5: 0x6A38,
    0x9EF6: 0x6A22,
    0x9EF7: 0x6A90,
    0x9EF8: 0x6A8D,
    0x9EF9: 0x6AA0,
    0x9EFA: 0x6A84,
    0x9EFB: 0x6AA2,
    0x9EFC: 0x6AA3,
    0x9F40: 0x6A97,
    0x9F41: 0x8617,
    0x9F42: 0x6ABB,
    0x9F43: 0x6AC3,
    0x9F44: 0x6AC2,
    0x9F45: 0x6AB8,
    0x9F46: 0x6AB3,
    0x9F47: 0x6AAC,
    0x9F48: 0x6ADE,
    0x9F49: 0x6AD1,
    0x9F4A: 0x6ADF,
    0x9F4B: 0x6AAA,
    0x9F4C: 0x6ADA,
    0x9F4D: 0x6AEA,
    0x9F4E: 0x6AFB,
    0x9F4F: 0x6B05,
    0x9F50: 0x8616,
    0x9F51: 0x6AFA,
    0x9F52: 0x6B12,
    0x9F53: 0x6B16,
    0x9F54: 0x9B31,
    0x9F55: 0x6B1F,
    0x9F56: 0x6B38,
    0x9F57: 0x6B37,
    0x9F58: 0x76DC,
    0x9F59: 0x6B39,
    0x9F5A: 0x98EE,
    0x9F5B: 0x6B47,
    0x9F5C: 0x6B43,
    0x9F5D: 0x6B49,
    0x9F5E: 0x6B50,
    0x9F5F: 0x6B59,
    0x9F60: 0x6B54,
    0x9F61: 0x6B5B,
    0x9F62: 0x6B5F,
    0x9F63: 0x6B61,
    0x9F64: 0x6B78,
    0x9F65: 0x6B79,
    0x9F66: 0x6B7F,
    0x9F67: 0x6B80,
    0x9F68: 0x6B84,
    0x9F69: 0x6B83,
    0x9F6A: 0x6B8D,
    0x9F6B: 0x6B98,
    0x9F6C: 0x6B95,
    0x9F6D: 0x6B9E,
    0x9F6E: 0x6BA4,
    0x9F6F: 0x6BAA,
    0x9F70: 0x6BAB,
    0x9F71: 0x6BAF,
    0x9F72: 0x6BB2,
    0x9F73: 0x6BB1,
    0x9F74: 0x6BB3,
    0x9F75: 0x6BB7,
    0x9F76: 0x6BBC,
    0x9F77: 0x6BC6,
    0x9F78: 0x6BCB,
    0x9F79: 0x6BD3,
    0x9F7A: 0x6BDF,
    0x9F7B: 0x6BEC,
    0x9F7C: 0x6BEB,
    0x9F7D: 0x6BF3,
    0x9F7E: 0x6BEF,
    0x9F80: 0x9EBE,
    0x9F81: 0x6C08,
    0x9F82: 0x6C13,
    0x9F83: 0x6C14,
    0x9F84: 0x6C1B,
    0x9F85: 0x6C24,
    0x9F86: 0x6C23,
    0x9F87: 0x6C5E,
    0x9F88: 0x6C55,
    0x9F89: 0x6C62,
    0x9F8A: 0x6C6A,
    0x9F8B: 0x6C82,
    0x9F8C: 0x6C8D,
    0x9F8D: 0x6C9A,
    0x9F8E: 0x6C81,
    0x9F8F: 0x6C9B,
    0x9F90: 0x6C7E,
    0x9F91: 0x6C68,
    0x9F92: 0x6C73,
    0x9F93: 0x6C92,
    0x9F94: 0x6C90,
    0x9F95: 0x6CC4,
    0x9F96: 0x6CF1,
    0x9F97: 0x6CD3,
    0x9F98: 0x6CBD,
    0x9F99: 0x6CD7,
    0x9F9A: 0x6CC5,
    0x9F9B: 0x6CDD,
    0x9F9C: 0x6CAE,
    0x9F9D: 0x6CB1,
    0x9F9E: 0x6CBE,
    0x9F9F: 0x6CBA,
    0x9FA0: 0x6CDB,
    0x9FA1: 0x6CEF,
    0x9FA2: 0x6CD9,
    0x9FA3: 0x6CEA,
    0x9FA4: 0x6D1F,
    0x9FA5: 0x884D,
    0x9FA6: 0x6D36,
    0x9FA7: 0x6D2B,
    0x9FA8: 0x6D3D,
    0x9FA9: 0x6D38,
    0x9FAA: 0x6D19,
    0x9FAB: 0x6D35,
    0x9FAC: 0x6D33,
    0x9FAD: 0x6D12,
    0x9FAE: 0x6D0C,
    0x9FAF: 0x6D63,
    0x9FB0: 0x6D93,
    0x9FB1: 0x6D64,
    0x9FB2: 0x6D5A,
    0x9FB3: 0x6D79,
    0x9FB4: 0x6D59,
    0x9FB5: 0x6D8E,
    0x9FB6: 0x6D95,
    0x9FB7: 0x6FE4,
    0x9FB8: 0x6D85,
    0x9FB9: 0x6DF9,
    0x9FBA: 0x6E15,
    0x9FBB: 0x6E0A,
    0x9FBC: 0x6DB5,
    0x9FBD: 0x6DC7,
    0x9FBE: 0x6DE6,
    0x9FBF: 0x6DB8,
    0x9FC0: 0x6DC6,
    0x9FC1: 0x6DEC,
    0x9FC2: 0x6DDE,
    0x9FC3: 0x6DCC,
    0x9FC4: 0x6DE8,
    0x9FC5: 0x6DD2,
    0x9FC6: 0x6DC5,
    0x9FC7: 0x6DFA,
    0x9FC8: 0x6DD9,
    0x9FC9: 0x6DE4,
    0x9FCA: 0x6DD5,
    0x9FCB: 0x6DEA,
    0x9FCC: 0x6DEE,
    0x9FCD: 0x6E2D,
    0x9FCE: 0x6E6E,
    0x9FCF: 0x6E2E,
    0x9FD0: 0x6E19,
    0x9FD1: 0x6E72,
    0x9FD2: 0x6E5F,
    0x9FD3: 0x6E3E,
    0x9FD4: 0x6E23,
    0x9FD5: 0x6E6B,
    0x9FD6: 0x6E2B,
    0x9FD7: 0x6E76,
    0x9FD8: 0x6E4D,
    0x9FD9: 0x6E1F,
    0x9FDA: 0x6E43,
    0x9FDB: 0x6E3A,
    0x9FDC: 0x6E4E,
    0x9FDD: 0x6E24,
    0x9FDE: 0x6EFF,
    0x9FDF: 0x6E1D,
    0x9FE0: 0x6E38,
    0x9FE1: 0x6E82,
    0x9FE2: 0x6EAA,
    0x9FE3: 0x6E98,
    0x9FE4: 0x6EC9,
    0x9FE5: 0x6EB7,
    0x9FE6: 0x6ED3,
    0x9FE7: 0x6EBD,
    0x9FE8: 0x6EAF,
    0x9FE9: 0x6EC4,
    0x9FEA: 0x6EB2,
    0x9FEB: 0x6ED4,
    0x9FEC: 0x6ED5,
    0x9FED: 0x6E8F,
    0x9FEE: 0x6EA5,
    0x9FEF: 0x6EC2,
    0x9FF0: 0x6E9F,
    0x9FF1: 0x6F41,
    0x9FF2: 0x6F11,
    0x9FF3: 0x704C,
    0x9FF4: 0x6EEC,
    0x9FF5: 0x6EF8,
    0x9FF6: 0x6EFE,
    0x9FF7: 0x6F3F,
    0x9FF8: 0x6EF2,
    0x9FF9: 0x6F31,
    0x9FFA: 0x6EEF,
    0x9FFB: 0x6F32,
    0x9FFC: 0x6ECC,
    0xA1: 0xFF61,
    0xA2: 0xFF62,
    0xA3: 0xFF63,
    0xA4: 0xFF64,
    0xA5: 0xFF65,
    0xA6: 0xFF66,
    0xA7: 0xFF67,
    0xA8: 0xFF68,
    0xA9: 0xFF69,
    0xAA: 0xFF6A,
    0xAB: 0xFF6B,
    0xAC: 0xFF6C,
    0xAD: 0xFF6D,
    0xAE: 0xFF6E,
    0xAF: 0xFF6F,
    0xB0: 0xFF70,
    0xB1: 0xFF71,
    0xB2: 0xFF72,
    0xB3: 0xFF73,
    0xB4: 0xFF74,
    0xB5: 0xFF75,
    0xB6: 0xFF76,
    0xB7: 0xFF77,
    0xB8: 0xFF78,
    0xB9: 0xFF79,
    0xBA: 0xFF7A,
    0xBB: 0xFF7B,
    0xBC: 0xFF7C,
    0xBD: 0xFF7D,
    0xBE: 0xFF7E,
    0xBF: 0xFF7F,
    0xC0: 0xFF80,
    0xC1: 0xFF81,
    0xC2: 0xFF82,
    0xC3: 0xFF83,
    0xC4: 0xFF84,
    0xC5: 0xFF85,
    0xC6: 0xFF86,
    0xC7: 0xFF87,
    0xC8: 0xFF88,
    0xC9: 0xFF89,
    0xCA: 0xFF8A,
    0xCB: 0xFF8B,
    0xCC: 0xFF8C,
    0xCD: 0xFF8D,
    0xCE: 0xFF8E,
    0xCF: 0xFF8F,
    0xD0: 0xFF90,
    0xD1: 0xFF91,
    0xD2: 0xFF92,
    0xD3: 0xFF93,
    0xD4: 0xFF94,
    0xD5: 0xFF95,
    0xD6: 0xFF96,
    0xD7: 0xFF97,
    0xD8: 0xFF98,
    0xD9: 0xFF99,
    0xDA: 0xFF9A,
    0xDB: 0xFF9B,
    0xDC: 0xFF9C,
    0xDD: 0xFF9D,
    0xDE: 0xFF9E,
    0xDF: 0xFF9F,
    0xE040: 0x6F3E,
    0xE041: 0x6F13,
    0xE042: 0x6EF7,
    0xE043: 0x6F86,
    0xE044: 0x6F7A,
    0xE045: 0x6F78,
    0xE046: 0x6F81,
    0xE047: 0x6F80,
    0xE048: 0x6F6F,
    0xE049: 0x6F5B,
    0xE04A: 0x6FF3,
    0xE04B: 0x6F6D,
    0xE04C: 0x6F82,
    0xE04D: 0x6F7C,
    0xE04E: 0x6F58,
    0xE04F: 0x6F8E,
    0xE050: 0x6F91,
    0xE051: 0x6FC2,
    0xE052: 0x6F66,
    0xE053: 0x6FB3,
    0xE054: 0x6FA3,
    0xE055: 0x6FA1,
    0xE056: 0x6FA4,
    0xE057: 0x6FB9,
    0xE058: 0x6FC6,
    0xE059: 0x6FAA,
    0xE05A: 0x6FDF,
    0xE05B: 0x6FD5,
    0xE05C: 0x6FEC,
    0xE05D: 0x6FD4,
    0xE05E: 0x6FD8,
    0xE05F: 0x6FF1,
    0xE060: 0x6FEE,
    0xE061: 0x6FDB,
    0xE062: 0x7009,
    0xE063: 0x700B,
    0xE064: 0x6FFA,
    0xE065: 0x7011,
    0xE066: 0x7001,
    0xE067: 0x700F,
    0xE068: 0x6FFE,
    0xE069: 0x701B,
    0xE06A: 0x701A,
    0xE06B: 0x6F74,
    0xE06C: 0x701D,
    0xE06D: 0x7018,
    0xE06E: 0x701F,
    0xE06F: 0x7030,
    0xE070: 0x703E,
    0xE071: 0x7032,
    0xE072: 0x7051,
    0xE073: 0x7063,
    0xE074: 0x7099,
    0xE075: 0x7092,
    0xE076: 0x70AF,
    0xE077: 0x70F1,
    0xE078: 0x70AC,
    0xE079: 0x70B8,
    0xE07A: 0x70B3,
    0xE07B: 0x70AE,
    0xE07C: 0x70DF,
    0xE07D: 0x70CB,
    0xE07E: 0x70DD,
    0xE080: 0x70D9,
    0xE081: 0x7109,
    0xE082: 0x70FD,
    0xE083: 0x711C,
    0xE084: 0x7119,
    0xE085: 0x7165,
    0xE086: 0x7155,
    0xE087: 0x7188,
    0xE088: 0x7166,
    0xE089: 0x7162,
    0xE08A: 0x714C,
    0xE08B: 0x7156,
    0xE08C: 0x716C,
    0xE08D: 0x718F,
    0xE08E: 0x71FB,
    0xE08F: 0x7184,
    0xE090: 0x7195,
    0xE091: 0x71A8,
    0xE092: 0x71AC,
    0xE093: 0x71D7,
    0xE094: 0x71B9,
    0xE095: 0x71BE,
    0xE096: 0x71D2,
    0xE097: 0x71C9,
    0xE098: 0x71D4,
    0xE099: 0x71CE,
    0xE09A: 0x71E0,
    0xE09B: 0x71EC,
    0xE09C: 0x71E7,
    0xE09D: 0x71F5,
    0xE09E: 0x71FC,
    0xE09F: 0x71F9,
    0xE0A0: 0x71FF,
    0xE0A1: 0x720D,
    0xE0A2: 0x7210,
    0xE0A3: 0x721B,
    0xE0A4: 0x7228,
    0xE0A5: 0x722D,
    0xE0A6: 0x722C,
    0xE0A7: 0x7230,
    0xE0A8: 0x7232,
    0xE0A9: 0x723B,
    0xE0AA: 0x723C,
    0xE0AB: 0x723F,
    0xE0AC: 0x7240,
    0xE0AD: 0x7246,
    0xE0AE: 0x724B,
    0xE0AF: 0x7258,
    0xE0B0: 0x7274,
    0xE0B1: 0x727E,
    0xE0B2: 0x7282,
    0xE0B3: 0x7281,
    0xE0B4: 0x7287,
    0xE0B5: 0x7292,
    0xE0B6: 0x7296,
    0xE0B7: 0x72A2,
    0xE0B8: 0x72A7,
    0xE0B9: 0x72B9,
    0xE0BA: 0x72B2,
    0xE0BB: 0x72C3,
    0xE0BC: 0x72C6,
    0xE0BD: 0x72C4,
    0xE0BE: 0x72CE,
    0xE0BF: 0x72D2,
    0xE0C0: 0x72E2,
    0xE0C1: 0x72E0,
    0xE0C2: 0x72E1,
    0xE0C3: 0x72F9,
    0xE0C4: 0x72F7,
    0xE0C5: 0x500F,
    0xE0C6: 0x7317,
    0xE0C7: 0x730A,
    0xE0C8: 0x731C,
    0xE0C9: 0x7316,
    0xE0CA: 0x731D,
    0xE0CB: 0x7334,
    0xE0CC: 0x732F,
    0xE0CD: 0x7329,
    0xE0CE: 0x7325,
    0xE0CF: 0x733E,
    0xE0D0: 0x734E,
    0xE0D1: 0x734F,
    0xE0D2: 0x9ED8,
    0xE0D3: 0x7357,
    0xE0D4: 0x736A,
    0xE0D5: 0x7368,
    0xE0D6: 0x7370,
    0xE0D7: 0x7378,
    0xE0D8: 0x7375,
    0xE0D9: 0x737B,
    0xE0DA: 0x737A,
    0xE0DB: 0x73C8,
    0xE0DC: 0x73B3,
    0xE0DD: 0x73CE,
    0xE0DE: 0x73BB,
    0xE0DF: 0x73C0,
    0xE0E0: 0x73E5,
    0xE0E1: 0x73EE,
    0xE0E2: 0x73DE,
    0xE0E3: 0x74A2,
    0xE0E4: 0x7405,
    0xE0E5: 0x746F,
    0xE0E6: 0x7425,
    0xE0E7: 0x73F8,
    0xE0E8: 0x7432,
    0xE0E9: 0x743A,
    0xE0EA: 0x7455,
    0xE0EB: 0x743F,
    0xE0EC: 0x745F,
    0xE0ED: 0x7459,
    0xE0EE: 0x7441,
    0xE0EF: 0x745C,
    0xE0F0: 0x7469,
    0xE0F1: 0x7470,
    0xE0F2: 0x7463,
    0xE0F3: 0x746A,
    0xE0F4: 0x7476,
    0xE0F5: 0x747E,
    0xE0F6: 0x748B,
    0xE0F7: 0x749E,
    0xE0F8: 0x74A7,
    0xE0F9: 0x74CA,
    0xE0FA: 0x74CF,
    0xE0FB: 0x74D4,
    0xE0FC: 0x73F1,
    0xE140: 0x74E0,
    0xE141: 0x74E3,
    0xE142: 0x74E7,
    0xE143: 0x74E9,
    0xE144: 0x74EE,
    0xE145: 0x74F2,
    0xE146: 0x74F0,
    0xE147: 0x74F1,
    0xE148: 0x74F8,
    0xE149: 0x74F7,
    0xE14A: 0x7504,
    0xE14B: 0x7503,
    0xE14C: 0x7505,
    0xE14D: 0x750C,
    0xE14E: 0x750E,
    0xE14F: 0x750D,
    0xE150: 0x7515,
    0xE151: 0x7513,
    0xE152: 0x751E,
    0xE153: 0x7526,
    0xE154: 0x752C,
    0xE155: 0x753C,
    0xE156: 0x7544,
    0xE157: 0x754D,
    0xE158: 0x754A,
    0xE159: 0x7549,
    0xE15A: 0x755B,
    0xE15B: 0x7546,
    0xE15C: 0x755A,
    0xE15D: 0x7569,
    0xE15E: 0x7564,
    0xE15F: 0x7567,
    0xE160: 0x756B,
    0xE161: 0x756D,
    0xE162: 0x7578,
    0xE163: 0x7576,
    0xE164: 0x7586,
    0xE165: 0x7587,
    0xE166: 0x7574,
    0xE167: 0x758A,
    0xE168: 0x7589,
    0xE169: 0x7582,
    0xE16A: 0x7594,
    0xE16B: 0x759A,
    0xE16C: 0x759D,
    0xE16D: 0x75A5,
    0xE16E: 0x75A3,
    0xE16F: 0x75C2,
    0xE170: 0x75B3,
    0xE171: 0x75C3,
    0xE172: 0x75B5,
    0xE173: 0x75BD,
    0xE174: 0x75B8,
    0xE175: 0x75BC,
    0xE176: 0x75B1,
    0xE177: 0x75CD,
    0xE178: 0x75CA,
    0xE179: 0x75D2,
    0xE17A: 0x75D9,
    0xE17B: 0x75E3,
    0xE17C: 0x75DE,
    0xE17D: 0x75FE,
    0xE17E: 0x75FF,
    0xE180: 0x75FC,
    0xE181: 0x7601,
    0xE182: 0x75F0,
    0xE183: 0x75FA,
    0xE184: 0x75F2,
    0xE185: 0x75F3,
    0xE186: 0x760B,
    0xE187: 0x760D,
    0xE188: 0x7609,
    0xE189: 0x761F,
    0xE18A: 0x7627,
    0xE18B: 0x7620,
    0xE18C: 0x7621,
    0xE18D: 0x7622,
    0xE18E: 0x7624,
    0xE18F: 0x7634,
    0xE190: 0x7630,
    0xE191: 0x763B,
    0xE192: 0x7647,
    0xE193: 0x7648,
    0xE194: 0x7646,
    0xE195: 0x765C,
    0xE196: 0x7658,
    0xE197: 0x7661,
    0xE198: 0x7662,
    0xE199: 0x7668,
    0xE19A: 0x7669,
    0xE19B: 0x766A,
    0xE19C: 0x7667,
    0xE19D: 0x766C,
    0xE19E: 0x7670,
    0xE19F: 0x7672,
    0xE1A0: 0x7676,
    0xE1A1: 0x7678,
    0xE1A2: 0x767C,
    0xE1A3: 0x7680,
    0xE1A4: 0x7683,
    0xE1A5: 0x7688,
    0xE1A6: 0x768B,
    0xE1A7: 0x768E,
    0xE1A8: 0x7696,
    0xE1A9: 0x7693,
    0xE1AA: 0x7699,
    0xE1AB: 0x769A,
    0xE1AC: 0x76B0,
    0xE1AD: 0x76B4,
    0xE1AE: 0x76B8,
    0xE1AF: 0x76B9,
    0xE1B0: 0x76BA,
    0xE1B1: 0x76C2,
    0xE1B2: 0x76CD,
    0xE1B3: 0x76D6,
    0xE1B4: 0x76D2,
    0xE1B5: 0x76DE,
    0xE1B6: 0x76E1,
    0xE1B7: 0x76E5,
    0xE1B8: 0x76E7,
    0xE1B9: 0x76EA,
    0xE1BA: 0x862F,
    0xE1BB: 0x76FB,
    0xE1BC: 0x7708,
    0xE1BD: 0x7707,
    0xE1BE: 0x7704,
    0xE1BF: 0x7729,
    0xE1C0: 0x7724,
    0xE1C1: 0x771E,
    0xE1C2: 0x7725,
    0xE1C3: 0x7726,
    0xE1C4: 0x771B,
    0xE1C5: 0x7737,
    0xE1C6: 0x7738,
    0xE1C7: 0x7747,
    0xE1C8: 0x775A,
    0xE1C9: 0x7768,
    0xE1CA: 0x776B,
    0xE1CB: 0x775B,
    0xE1CC: 0x7765,
    0xE1CD: 0x777F,
    0xE1CE: 0x777E,
    0xE1CF: 0x7779,
    0xE1D0: 0x778E,
    0xE1D1: 0x778B,
    0xE1D2: 0x7791,
    0xE1D3: 0x77A0,
    0xE1D4: 0x779E,
    0xE1D5: 0x77B0,
    0xE1D6: 0x77B6,
    0xE1D7: 0x77B9,
    0xE1D8: 0x77BF,
    0xE1D9: 0x77BC,
    0xE1DA: 0x77BD,
    0xE1DB: 0x77BB,
    0xE1DC: 0x77C7,
    0xE1DD: 0x77CD,
    0xE1DE: 0x77D7,
    0xE1DF: 0x77DA,
    0xE1E0: 0x77DC,
    0xE1E1: 0x77E3,
    0xE1E2: 0x77EE,
    0xE1E3: 0x77FC,
    0xE1E4: 0x780C,
    0xE1E5: 0x7812,
    0xE1E6: 0x7926,
    0xE1E7: 0x7820,
    0xE1E8: 0x792A,
    0xE1E9: 0x7845,
    0xE1EA: 0x788E,
    0xE1EB: 0x7874,
    0xE1EC: 0x7886,
    0xE1ED: 0x787C,
    0xE1EE: 0x789A,
    0xE1EF: 0x788C,
    0xE1F0: 0x78A3,
    0xE1F1: 0x78B5,
    0xE1F2: 0x78AA,
    0xE1F3: 0x78AF,
    0xE1F4: 0x78D1,
    0xE1F5: 0x78C6,
    0xE1F6: 0x78CB,
    0xE1F7: 0x78D4,
    0xE1F8: 0x78BE,
    0xE1F9: 0x78BC,
    0xE1FA: 0x78C5,
    0xE1FB: 0x78CA,
    0xE1FC: 0x78EC,
    0xE240: 0x78E7,
    0xE241: 0x78DA,
    0xE242: 0x78FD,
    0xE243: 0x78F4,
    0xE244: 0x7907,
    0xE245: 0x7912,
    0xE246: 0x7911,
    0xE247: 0x7919,
    0xE248: 0x792C,
    0xE249: 0x792B,
    0xE24A: 0x7940,
    0xE24B: 0x7960,
    0xE24C: 0x7957,
    0xE24D: 0x795F,
    0xE24E: 0x795A,
    0xE24F: 0x7955,
    0xE250: 0x7953,
    0xE251: 0x797A,
    0xE252: 0x797F,
    0xE253: 0x798A,
    0xE254: 0x799D,
    0xE255: 0x79A7,
    0xE256: 0x9F4B,
    0xE257: 0x79AA,
    0xE258: 0x79AE,
    0xE259: 0x79B3,
    0xE25A: 0x79B9,
    0xE25B: 0x79BA,
    0xE25C: 0x79C9,
    0xE25D: 0x79D5,
    0xE25E: 0x79E7,
    0xE25F: 0x79EC,
    0xE260: 0x79E1,
    0xE261: 0x79E3,
    0xE262: 0x7A08,
    0xE263: 0x7A0D,
    0xE264: 0x7A18,
    0xE265: 0x7A19,
    0xE266: 0x7A20,
    0xE267: 0x7A1F,
    0xE268: 0x7980,
    0xE269: 0x7A31,
    0xE26A: 0x7A3B,
    0xE26B: 0x7A3E,
    0xE26C: 0x7A37,
    0xE26D: 0x7A43,
    0xE26E: 0x7A57,
    0xE26F: 0x7A49,
    0xE270: 0x7A61,
    0xE271: 0x7A62,
    0xE272: 0x7A69,
    0xE273: 0x9F9D,
    0xE274: 0x7A70,
    0xE275: 0x7A79,
    0xE276: 0x7A7D,
    0xE277: 0x7A88,
    0xE278: 0x7A97,
    0xE279: 0x7A95,
    0xE27A: 0x7A98,
    0xE27B: 0x7A96,
    0xE27C: 0x7AA9,
    0xE27D: 0x7AC8,
    0xE27E: 0x7AB0,
    0xE280: 0x7AB6,
    0xE281: 0x7AC5,
    0xE282: 0x7AC4,
    0xE283: 0x7ABF,
    0xE284: 0x9083,
    0xE285: 0x7AC7,
    0xE286: 0x7ACA,
    0xE287: 0x7ACD,
    0xE288: 0x7ACF,
    0xE289: 0x7AD5,
    0xE28A: 0x7AD3,
    0xE28B: 0x7AD9,
    0xE28C: 0x7ADA,
    0xE28D: 0x7ADD,
    0xE28E: 0x7AE1,
    0xE28F: 0x7AE2,
    0xE290: 0x7AE6,
    0xE291: 0x7AED,
    0xE292: 0x7AF0,
    0xE293: 0x7B02,
    0xE294: 0x7B0F,
    0xE295: 0x7B0A,
    0xE296: 0x7B06,
    0xE297: 0x7B33,
    0xE298: 0x7B18,
    0xE299: 0x7B19,
    0xE29A: 0x7B1E,
    0xE29B: 0x7B35,
    0xE29C: 0x7B28,
    0xE29D: 0x7B36,
    0xE29E: 0x7B50,
    0xE29F: 0x7B7A,
    0xE2A0: 0x7B04,
    0xE2A1: 0x7B4D,
    0xE2A2: 0x7B0B,
    0xE2A3: 0x7B4C,
    0xE2A4: 0x7B45,
    0xE2A5: 0x7B75,
    0xE2A6: 0x7B65,
    0xE2A7: 0x7B74,
    0xE2A8: 0x7B67,
    0xE2A9: 0x7B70,
    0xE2AA: 0x7B71,
    0xE2AB: 0x7B6C,
    0xE2AC: 0x7B6E,
    0xE2AD: 0x7B9D,
    0xE2AE: 0x7B98,
    0xE2AF: 0x7B9F,
    0xE2B0: 0x7B8D,
    0xE2B1: 0x7B9C,
    0xE2B2: 0x7B9A,
    0xE2B3: 0x7B8B,
    0xE2B4: 0x7B92,
    0xE2B5: 0x7B8F,
    0xE2B6: 0x7B5D,
    0xE2B7: 0x7B99,
    0xE2B8: 0x7BCB,
    0xE2B9: 0x7BC1,
    0xE2BA: 0x7BCC,
    0xE2BB: 0x7BCF,
    0xE2BC: 0x7BB4,
    0xE2BD: 0x7BC6,
    0xE2BE: 0x7BDD,
    0xE2BF: 0x7BE9,
    0xE2C0: 0x7C11,
    0xE2C1: 0x7C14,
    0xE2C2: 0x7BE6,
    0xE2C3: 0x7BE5,
    0xE2C4: 0x7C60,
    0xE2C5: 0x7C00,
    0xE2C6: 0x7C07,
    0xE2C7: 0x7C13,
    0xE2C8: 0x7BF3,
    0xE2C9: 0x7BF7,
    0xE2CA: 0x7C17,
    0xE2CB: 0x7C0D,
    0xE2CC: 0x7BF6,
    0xE2CD: 0x7C23,
    0xE2CE: 0x7C27,
    0xE2CF: 0x7C2A,
    0xE2D0: 0x7C1F,
    0xE2D1: 0x7C37,
    0xE2D2: 0x7C2B,
    0xE2D3: 0x7C3D,
    0xE2D4: 0x7C4C,
    0xE2D5: 0x7C43,
    0xE2D6: 0x7C54,
    0xE2D7: 0x7C4F,
    0xE2D8: 0x7C40,
    0xE2D9: 0x7C50,
    0xE2DA: 0x7C58,
    0xE2DB: 0x7C5F,
    0xE2DC: 0x7C64,
    0xE2DD: 0x7C56,
    0xE2DE: 0x7C65,
    0xE2DF: 0x7C6C,
    0xE2E0: 0x7C75,
    0xE2E1: 0x7C83,
    0xE2E2: 0x7C90,
    0xE2E3: 0x7CA4,
    0xE2E4: 0x7CAD,
    0xE2E5: 0x7CA2,
    0xE2E6: 0x7CAB,
    0xE2E7: 0x7CA1,
    0xE2E8: 0x7CA8,
    0xE2E9: 0x7CB3,
    0xE2EA: 0x7CB2,
    0xE2EB: 0x7CB1,
    0xE2EC: 0x7CAE,
    0xE2ED: 0x7CB9,
    0xE2EE: 0x7CBD,
    0xE2EF: 0x7CC0,
    0xE2F0: 0x7CC5,
    0xE2F1: 0x7CC2,
    0xE2F2: 0x7CD8,
    0xE2F3: 0x7CD2,
    0xE2F4: 0x7CDC,
    0xE2F5: 0x7CE2,
    0xE2F6: 0x9B3B,
    0xE2F7: 0x7CEF,
    0xE2F8: 0x7CF2,
    0xE2F9: 0x7CF4,
    0xE2FA: 0x7CF6,
    0xE2FB: 0x7CFA,
    0xE2FC: 0x7D06,
    0xE340: 0x7D02,
    0xE341: 0x7D1C,
    0xE342: 0x7D15,
    0xE343: 0x7D0A,
    0xE344: 0x7D45,
    0xE345: 0x7D4B,
    0xE346: 0x7D2E,
    0xE347: 0x7D32,
    0xE348: 0x7D3F,
    0xE349: 0x7D35,
    0xE34A: 0x7D46,
    0xE34B: 0x7D73,
    0xE34C: 0x7D56,
    0xE34D: 0x7D4E,
    0xE34E: 0x7D72,
    0xE34F: 0x7D68,
    0xE350: 0x7D6E,
    0xE351: 0x7D4F,
    0xE352: 0x7D63,
    0xE353: 0x7D93,
    0xE354: 0x7D89,
    0xE355: 0x7D5B,
    0xE356: 0x7D8F,
    0xE357: 0x7D7D,
    0xE358: 0x7D9B,
    0xE359: 0x7DBA,
    0xE35A: 0x7DAE,
    0xE35B: 0x7DA3,
    0xE35C: 0x7DB5,
    0xE35D: 0x7DC7,
    0xE35E: 0x7DBD,
    0xE35F: 0x7DAB,
    0xE360: 0x7E3D,
    0xE361: 0x7DA2,
    0xE362: 0x7DAF,
    0xE363: 0x7DDC,
    0xE364: 0x7DB8,
    0xE365: 0x7D9F,
    0xE366: 0x7DB0,
    0xE367: 0x7DD8,
    0xE368: 0x7DDD,
    0xE369: 0x7DE4,
    0xE36A: 0x7DDE,
    0xE36B: 0x7DFB,
    0xE36C: 0x7DF2,
    0xE36D: 0x7DE1,
    0xE36E: 0x7E05,
    0xE36F: 0x7E0A,
    0xE370: 0x7E23,
    0xE371: 0x7E21,
    0xE372: 0x7E12,
    0xE373: 0x7E31,
    0xE374: 0x7E1F,
    0xE375: 0x7E09,
    0xE376: 0x7E0B,
    0xE377: 0x7E22,
    0xE378: 0x7E46,
    0xE379: 0x7E66,
    0xE37A: 0x7E3B,
    0xE37B: 0x7E35,
    0xE37C: 0x7E39,
    0xE37D: 0x7E43,
    0xE37E: 0x7E37,
    0xE380: 0x7E32,
    0xE381: 0x7E3A,
    0xE382: 0x7E67,
    0xE383: 0x7E5D,
    0xE384: 0x7E56,
    0xE385: 0x7E5E,
    0xE386: 0x7E59,
    0xE387: 0x7E5A,
    0xE388: 0x7E79,
    0xE389: 0x7E6A,
    0xE38A: 0x7E69,
    0xE38B: 0x7E7C,
    0xE38C: 0x7E7B,
    0xE38D: 0x7E83,
    0xE38E: 0x7DD5,
    0xE38F: 0x7E7D,
    0xE390: 0x8FAE,
    0xE391: 0x7E7F,
    0xE392: 0x7E88,
    0xE393: 0x7E89,
    0xE394: 0x7E8C,
    0xE395: 0x7E92,
    0xE396: 0x7E90,
    0xE397: 0x7E93,
    0xE398: 0x7E94,
    0xE399: 0x7E96,
    0xE39A: 0x7E8E,
    0xE39B: 0x7E9B,
    0xE39C: 0x7E9C,
    0xE39D: 0x7F38,
    0xE39E: 0x7F3A,
    0xE39F: 0x7F45,
    0xE3A0: 0x7F4C,
    0xE3A1: 0x7F4D,
    0xE3A2: 0x7F4E,
    0xE3A3: 0x7F50,
    0xE3A4: 0x7F51,
    0xE3A5: 0x7F55,
    0xE3A6: 0x7F54,
    0xE3A7: 0x7F58,
    0xE3A8: 0x7F5F,
    0xE3A9: 0x7F60,
    0xE3AA: 0x7F68,
    0xE3AB: 0x7F69,
    0xE3AC: 0x7F67,
    0xE3AD: 0x7F78,
    0xE3AE: 0x7F82,
    0xE3AF: 0x7F86,
    0xE3B0: 0x7F83,
    0xE3B1: 0x7F88,
    0xE3B2: 0x7F87,
    0xE3B3: 0x7F8C,
    0xE3B4: 0x7F94,
    0xE3B5: 0x7F9E,
    0xE3B6: 0x7F9D,
    0xE3B7: 0x7F9A,
    0xE3B8: 0x7FA3,
    0xE3B9: 0x7FAF,
    0xE3BA: 0x7FB2,
    0xE3BB: 0x7FB9,
    0xE3BC: 0x7FAE,
    0xE3BD: 0x7FB6,
    0xE3BE: 0x7FB8,
    0xE3BF: 0x8B71,
    0xE3C0: 0x7FC5,
    0xE3C1: 0x7FC6,
    0xE3C2: 0x7FCA,
    0xE3C3: 0x7FD5,
    0xE3C4: 0x7FD4,
    0xE3C5: 0x7FE1,
    0xE3C6: 0x7FE6,
    0xE3C7: 0x7FE9,
    0xE3C8: 0x7FF3,
    0xE3C9: 0x7FF9,
    0xE3CA: 0x98DC,
    0xE3CB: 0x8006,
    0xE3CC: 0x8004,
    0xE3CD: 0x800B,
    0xE3CE: 0x8012,
    0xE3CF: 0x8018,
    0xE3D0: 0x8019,
    0xE3D1: 0x801C,
    0xE3D2: 0x8021,
    0xE3D3: 0x8028,
    0xE3D4: 0x803F,
    0xE3D5: 0x803B,
    0xE3D6: 0x804A,
    0xE3D7: 0x8046,
    0xE3D8: 0x8052,
    0xE3D9: 0x8058,
    0xE3DA: 0x805A,
    0xE3DB: 0x805F,
    0xE3DC: 0x8062,
    0xE3DD: 0x8068,
    0xE3DE: 0x8073,
    0xE3DF: 0x8072,
    0xE3E0: 0x8070,
    0xE3E1: 0x8076,
    0xE3E2: 0x8079,
    0xE3E3: 0x807D,
    0xE3E4: 0x807F,
    0xE3E5: 0x8084,
    0xE3E6: 0x8086,
    0xE3E7: 0x8085,
    0xE3E8: 0x809B,
    0xE3E9: 0x8093,
    0xE3EA: 0x809A,
    0xE3EB: 0x80AD,
    0xE3EC: 0x5190,
    0xE3ED: 0x80AC,
    0xE3EE: 0x80DB,
    0xE3EF: 0x80E5,
    0xE3F0: 0x80D9,
    0xE3F1: 0x80DD,
    0xE3F2: 0x80C4,
    0xE3F3: 0x80DA,
    0xE3F4: 0x80D6,
    0xE3F5: 0x8109,
    0xE3F6: 0x80EF,
    0xE3F7: 0x80F1,
    0xE3F8: 0x811B,
    0xE3F9: 0x8129,
    0xE3FA: 0x8123,
    0xE3FB: 0x812F,
    0xE3FC: 0x814B,
    0xE440: 0x968B,
    0xE441: 0x8146,
    0xE442: 0x813E,
    0xE443: 0x8153,
    0xE444: 0x8151,
    0xE445: 0x80FC,
    0xE446: 0x8171,
    0xE447: 0x816E,
    0xE448: 0x8165,
    0xE449: 0x8166,
    0xE44A: 0x8174,
    0xE44B: 0x8183,
    0xE44C: 0x8188,
    0xE44D: 0x818A,
    0xE44E: 0x8180,
    0xE44F: 0x8182,
    0xE450: 0x81A0,
    0xE451: 0x8195,
    0xE452: 0x81A4,
    0xE453: 0x81A3,
    0xE454: 0x815F,
    0xE455: 0x8193,
    0xE456: 0x81A9,
    0xE457: 0x81B0,
    0xE458: 0x81B5,
    0xE459: 0x81BE,
    0xE45A: 0x81B8,
    0xE45B: 0x81BD,
    0xE45C: 0x81C0,
    0xE45D: 0x81C2,
    0xE45E: 0x81BA,
    0xE45F: 0x81C9,
    0xE460: 0x81CD,
    0xE461: 0x81D1,
    0xE462: 0x81D9,
    0xE463: 0x81D8,
    0xE464: 0x81C8,
    0xE465: 0x81DA,
    0xE466: 0x81DF,
    0xE467: 0x81E0,
    0xE468: 0x81E7,
    0xE469: 0x81FA,
    0xE46A: 0x81FB,
    0xE46B: 0x81FE,
    0xE46C: 0x8201,
    0xE46D: 0x8202,
    0xE46E: 0x8205,
    0xE46F: 0x8207,
    0xE470: 0x820A,
    0xE471: 0x820D,
    0xE472: 0x8210,
    0xE473: 0x8216,
    0xE474: 0x8229,
    0xE475: 0x822B,
    0xE476: 0x8238,
    0xE477: 0x8233,
    0xE478: 0x8240,
    0xE479: 0x8259,
    0xE47A: 0x8258,
    0xE47B: 0x825D,
    0xE47C: 0x825A,
    0xE47D: 0x825F,
    0xE47E: 0x8264,
    0xE480: 0x8262,
    0xE481: 0x8268,
    0xE482: 0x826A,
    0xE483: 0x826B,
    0xE484: 0x822E,
    0xE485: 0x8271,
    0xE486: 0x8277,
    0xE487: 0x8278,
    0xE488: 0x827E,
    0xE489: 0x828D,
    0xE48A: 0x8292,
    0xE48B: 0x82AB,
    0xE48C: 0x829F,
    0xE48D: 0x82BB,
    0xE48E: 0x82AC,
    0xE48F: 0x82E1,
    0xE490: 0x82E3,
    0xE491: 0x82DF,
    0xE492: 0x82D2,
    0xE493: 0x82F4,
    0xE494: 0x82F3,
    0xE495: 0x82FA,
    0xE496: 0x8393,
    0xE497: 0x8303,
    0xE498: 0x82FB,
    0xE499: 0x82F9,
    0xE49A: 0x82DE,
    0xE49B: 0x8306,
    0xE49C: 0x82DC,
    0xE49D: 0x8309,
    0xE49E: 0x82D9,
    0xE49F: 0x8335,
    0xE4A0: 0x8334,
    0xE4A1: 0x8316,
    0xE4A2: 0x8332,
    0xE4A3: 0x8331,
    0xE4A4: 0x8340,
    0xE4A5: 0x8339,
    0xE4A6: 0x8350,
    0xE4A7: 0x8345,
    0xE4A8: 0x832F,
    0xE4A9: 0x832B,
    0xE4AA: 0x8317,
    0xE4AB: 0x8318,
    0xE4AC: 0x8385,
    0xE4AD: 0x839A,
    0xE4AE: 0x83AA,
    0xE4AF: 0x839F,
    0xE4B0: 0x83A2,
    0xE4B1: 0x8396,
    0xE4B2: 0x8323,
    0xE4B3: 0x838E,
    0xE4B4: 0x8387,
    0xE4B5: 0x838A,
    0xE4B6: 0x837C,
    0xE4B7: 0x83B5,
    0xE4B8: 0x8373,
    0xE4B9: 0x8375,
    0xE4BA: 0x83A0,
    0xE4BB: 0x8389,
    0xE4BC: 0x83A8,
    0xE4BD: 0x83F4,
    0xE4BE: 0x8413,
    0xE4BF: 0x83EB,
    0xE4C0: 0x83CE,
    0xE4C1: 0x83FD,
    0xE4C2: 0x8403,
    0xE4C3: 0x83D8,
    0xE4C4: 0x840B,
    0xE4C5: 0x83C1,
    0xE4C6: 0x83F7,
    0xE4C7: 0x8407,
    0xE4C8: 0x83E0,
    0xE4C9: 0x83F2,
    0xE4CA: 0x840D,
    0xE4CB: 0x8422,
    0xE4CC: 0x8420,
    0xE4CD: 0x83BD,
    0xE4CE: 0x8438,
    0xE4CF: 0x8506,
    0xE4D0: 0x83FB,
    0xE4D1: 0x846D,
    0xE4D2: 0x842A,
    0xE4D3: 0x843C,
    0xE4D4: 0x855A,
    0xE4D5: 0x8484,
    0xE4D6: 0x8477,
    0xE4D7: 0x846B,
    0xE4D8: 0x84AD,
    0xE4D9: 0x846E,
    0xE4DA: 0x8482,
    0xE4DB: 0x8469,
    0xE4DC: 0x8446,
    0xE4DD: 0x842C,
    0xE4DE: 0x846F,
    0xE4DF: 0x8479,
    0xE4E0: 0x8435,
    0xE4E1: 0x84CA,
    0xE4E2: 0x8462,
    0xE4E3: 0x84B9,
    0xE4E4: 0x84BF,
    0xE4E5: 0x849F,
    0xE4E6: 0x84D9,
    0xE4E7: 0x84CD,
    0xE4E8: 0x84BB,
    0xE4E9: 0x84DA,
    0xE4EA: 0x84D0,
    0xE4EB: 0x84C1,
    0xE4EC: 0x84C6,
    0xE4ED: 0x84D6,
    0xE4EE: 0x84A1,
    0xE4EF: 0x8521,
    0xE4F0: 0x84FF,
    0xE4F1: 0x84F4,
    0xE4F2: 0x8517,
    0xE4F3: 0x8518,
    0xE4F4: 0x852C,
    0xE4F5: 0x851F,
    0xE4F6: 0x8515,
    0xE4F7: 0x8514,
    0xE4F8: 0x84FC,
    0xE4F9: 0x8540,
    0xE4FA: 0x8563,
    0xE4FB: 0x8558,
    0xE4FC: 0x8548,
    0xE540: 0x8541,
    0xE541: 0x8602,
    0xE542: 0x854B,
    0xE543: 0x8555,
    0xE544: 0x8580,
    0xE545: 0x85A4,
    0xE546: 0x8588,
    0xE547: 0x8591,
    0xE548: 0x858A,
    0xE549: 0x85A8,
    0xE54A: 0x856D,
    0xE54B: 0x8594,
    0xE54C: 0x859B,
    0xE54D: 0x85EA,
    0xE54E: 0x8587,
    0xE54F: 0x859C,
    0xE550: 0x8577,
    0xE551: 0x857E,
    0xE552: 0x8590,
    0xE553: 0x85C9,
    0xE554: 0x85BA,
    0xE555: 0x85CF,
    0xE556: 0x85B9,
    0xE557: 0x85D0,
    0xE558: 0x85D5,
    0xE559: 0x85DD,
    0xE55A: 0x85E5,
    0xE55B: 0x85DC,
    0xE55C: 0x85F9,
    0xE55D: 0x860A,
    0xE55E: 0x8613,
    0xE55F: 0x860B,
    0xE560: 0x85FE,
    0xE561: 0x85FA,
    0xE562: 0x8606,
    0xE563: 0x8622,
    0xE564: 0x861A,
    0xE565: 0x8630,
    0xE566: 0x863F,
    0xE567: 0x864D,
    0xE568: 0x4E55,
    0xE569: 0x8654,
    0xE56A: 0x865F,
    0xE56B: 0x8667,
    0xE56C: 0x8671,
    0xE56D: 0x8693,
    0xE56E: 0x86A3,
    0xE56F: 0x86A9,
    0xE570: 0x86AA,
    0xE571: 0x868B,
    0xE572: 0x868C,
    0xE573: 0x86B6,
    0xE574: 0x86AF,
    0xE575: 0x86C4,
    0xE576: 0x86C6,
    0xE577: 0x86B0,
    0xE578: 0x86C9,
    0xE579: 0x8823,
    0xE57A: 0x86AB,
    0xE57B: 0x86D4,
    0xE57C: 0x86DE,
    0xE57D: 0x86E9,
    0xE57E: 0x86EC,
    0xE580: 0x86DF,
    0xE581: 0x86DB,
    0xE582: 0x86EF,
    0xE583: 0x8712,
    0xE584: 0x8706,
    0xE585: 0x8708,
    0xE586: 0x8700,
    0xE587: 0x8703,
    0xE588: 0x86FB,
    0xE589: 0x8711,
    0xE58A: 0x8709,
    0xE58B: 0x870D,
    0xE58C: 0x86F9,
    0xE58D: 0x870A,
    0xE58E: 0x8734,
    0xE58F: 0x873F,
    0xE590: 0x8737,
    0xE591: 0x873B,
    0xE592: 0x8725,
    0xE593: 0x8729,
    0xE594: 0x871A,
    0xE595: 0x8760,
    0xE596: 0x875F,
    0xE597: 0x8778,
    0xE598: 0x874C,
    0xE599: 0x874E,
    0xE59A: 0x8774,
    0xE59B: 0x8757,
    0xE59C: 0x8768,
    0xE59D: 0x876E,
    0xE59E: 0x8759,
    0xE59F: 0x8753,
    0xE5A0: 0x8763,
    0xE5A1: 0x876A,
    0xE5A2: 0x8805,
    0xE5A3: 0x87A2,
    0xE5A4: 0x879F,
    0xE5A5: 0x8782,
    0xE5A6: 0x87AF,
    0xE5A7: 0x87CB,
    0xE5A8: 0x87BD,
    0xE5A9: 0x87C0,
    0xE5AA: 0x87D0,
    0xE5AB: 0x96D6,
    0xE5AC: 0x87AB,
    0xE5AD: 0x87C4,
    0xE5AE: 0x87B3,
    0xE5AF: 0x87C7,
    0xE5B0: 0x87C6,
    0xE5B1: 0x87BB,
    0xE5B2: 0x87EF,
    0xE5B3: 0x87F2,
    0xE5B4: 0x87E0,
    0xE5B5: 0x880F,
    0xE5B6: 0x880D,
    0xE5B7: 0x87FE,
    0xE5B8: 0x87F6,
    0xE5B9: 0x87F7,
    0xE5BA: 0x880E,
    0xE5BB: 0x87D2,
    0xE5BC: 0x8811,
    0xE5BD: 0x8816,
    0xE5BE: 0x8815,
    0xE5BF: 0x8822,
    0xE5C0: 0x8821,
    0xE5C1: 0x8831,
    0xE5C2: 0x8836,
    0xE5C3: 0x8839,
    0xE5C4: 0x8827,
    0xE5C5: 0x883B,
    0xE5C6: 0x8844,
    0xE5C7: 0x8842,
    0xE5C8: 0x8852,
    0xE5C9: 0x8859,
    0xE5CA: 0x885E,
    0xE5CB: 0x8862,
    0xE5CC: 0x886B,
    0xE5CD: 0x8881,
    0xE5CE: 0x887E,
    0xE5CF: 0x889E,
    0xE5D0: 0x8875,
    0xE5D1: 0x887D,
    0xE5D2: 0x88B5,
    0xE5D3: 0x8872,
    0xE5D4: 0x8882,
    0xE5D5: 0x8897,
    0xE5D6: 0x8892,
    0xE5D7: 0x88AE,
    0xE5D8: 0x8899,
    0xE5D9: 0x88A2,
    0xE5DA: 0x888D,
    0xE5DB: 0x88A4,
    0xE5DC: 0x88B0,
    0xE5DD: 0x88BF,
    0xE5DE: 0x88B1,
    0xE5DF: 0x88C3,
    0xE5E0: 0x88C4,
    0xE5E1: 0x88D4,
    0xE5E2: 0x88D8,
    0xE5E3: 0x88D9,
    0xE5E4: 0x88DD,
    0xE5E5: 0x88F9,
    0xE5E6: 0x8902,
    0xE5E7: 0x88FC,
    0xE5E8: 0x88F4,
    0xE5E9: 0x88E8,
    0xE5EA: 0x88F2,
    0xE5EB: 0x8904,
    0xE5EC: 0x890C,
    0xE5ED: 0x890A,
    0xE5EE: 0x8913,
    0xE5EF: 0x8943,
    0xE5F0: 0x891E,
    0xE5F1: 0x8925,
    0xE5F2: 0x892A,
    0xE5F3: 0x892B,
    0xE5F4: 0x8941,
    0xE5F5: 0x8944,
    0xE5F6: 0x893B,
    0xE5F7: 0x8936,
    0xE5F8: 0x8938,
    0xE5F9: 0x894C,
    0xE5FA: 0x891D,
    0xE5FB: 0x8960,
    0xE5FC: 0x895E,
    0xE640: 0x8966,
    0xE641: 0x8964,
    0xE642: 0x896D,
    0xE643: 0x896A,
    0xE644: 0x896F,
    0xE645: 0x8974,
    0xE646: 0x8977,
    0xE647: 0x897E,
    0xE648: 0x8983,
    0xE649: 0x8988,
    0xE64A: 0x898A,
    0xE64B: 0x8993,
    0xE64C: 0x8998,
    0xE64D: 0x89A1,
    0xE64E: 0x89A9,
    0xE64F: 0x89A6,
    0xE650: 0x89AC,
    0xE651: 0x89AF,
    0xE652: 0x89B2,
    0xE653: 0x89BA,
    0xE654: 0x89BD,
    0xE655: 0x89BF,
    0xE656: 0x89C0,
    0xE657: 0x89DA,
    0xE658: 0x89DC,
    0xE659: 0x89DD,
    0xE65A: 0x89E7,
    0xE65B: 0x89F4,
    0xE65C: 0x89F8,
    0xE65D: 0x8A03,
    0xE65E: 0x8A16,
    0xE65F: 0x8A10,
    0xE660: 0x8A0C,
    0xE661: 0x8A1B,
    0xE662: 0x8A1D,
    0xE663: 0x8A25,
    0xE664: 0x8A36,
    0xE665: 0x8A41,
    0xE666: 0x8A5B,
    0xE667: 0x8A52,
    0xE668: 0x8A46,
    0xE669: 0x8A48,
    0xE66A: 0x8A7C,
    0xE66B: 0x8A6D,
    0xE66C: 0x8A6C,
    0xE66D: 0x8A62,
    0xE66E: 0x8A85,
    0xE66F: 0x8A82,
    0xE670: 0x8A84,
    0xE671: 0x8AA8,
    0xE672: 0x8AA1,
    0xE673: 0x8A91,
    0xE674: 0x8AA5,
    0xE675: 0x8AA6,
    0xE676: 0x8A9A,
    0xE677: 0x8AA3,
    0xE678: 0x8AC4,
    0xE679: 0x8ACD,
    0xE67A: 0x8AC2,
    0xE67B: 0x8ADA,
    0xE67C: 0x8AEB,
    0xE67D: 0x8AF3,
    0xE67E: 0x8AE7,
    0xE680: 0x8AE4,
    0xE681: 0x8AF1,
    0xE682: 0x8B14,
    0xE683: 0x8AE0,
    0xE684: 0x8AE2,
    0xE685: 0x8AF7,
    0xE686: 0x8ADE,
    0xE687: 0x8ADB,
    0xE688: 0x8B0C,
    0xE689: 0x8B07,
    0xE68A: 0x8B1A,
    0xE68B: 0x8AE1,
    0xE68C: 0x8B16,
    0xE68D: 0x8B10,
    0xE68E: 0x8B17,
    0xE68F: 0x8B20,
    0xE690: 0x8B33,
    0xE691: 0x97AB,
    0xE692: 0x8B26,
    0xE693: 0x8B2B,
    0xE694: 0x8B3E,
    0xE695: 0x8B28,
    0xE696: 0x8B41,
    0xE697: 0x8B4C,
    0xE698: 0x8B4F,
    0xE699: 0x8B4E,
    0xE69A: 0x8B49,
    0xE69B: 0x8B56,
    0xE69C: 0x8B5B,
    0xE69D: 0x8B5A,
    0xE69E: 0x8B6B,
    0xE69F: 0x8B5F,
    0xE6A0: 0x8B6C,
    0xE6A1: 0x8B6F,
    0xE6A2: 0x8B74,
    0xE6A3: 0x8B7D,
    0xE6A4: 0x8B80,
    0xE6A5: 0x8B8C,
    0xE6A6: 0x8B8E,
    0xE6A7: 0x8B92,
    0xE6A8: 0x8B93,
    0xE6A9: 0x8B96,
    0xE6AA: 0x8B99,
    0xE6AB: 0x8B9A,
    0xE6AC: 0x8C3A,
    0xE6AD: 0x8C41,
    0xE6AE: 0x8C3F,
    0xE6AF: 0x8C48,
    0xE6B0: 0x8C4C,
    0xE6B1: 0x8C4E,
    0xE6B2: 0x8C50,
    0xE6B3: 0x8C55,
    0xE6B4: 0x8C62,
    0xE6B5: 0x8C6C,
    0xE6B6: 0x8C78,
    0xE6B7: 0x8C7A,
    0xE6B8: 0x8C82,
    0xE6B9: 0x8C89,
    0xE6BA: 0x8C85,
    0xE6BB: 0x8C8A,
    0xE6BC: 0x8C8D,
    0xE6BD: 0x8C8E,
    0xE6BE: 0x8C94,
    0xE6BF: 0x8C7C,
    0xE6C0: 0x8C98,
    0xE6C1: 0x621D,
    0xE6C2: 0x8CAD,
    0xE6C3: 0x8CAA,
    0xE6C4: 0x8CBD,
    0xE6C5: 0x8CB2,
    0xE6C6: 0x8CB3,
    0xE6C7: 0x8CAE,
    0xE6C8: 0x8CB6,
    0xE6C9: 0x8CC8,
    0xE6CA: 0x8CC1,
    0xE6CB: 0x8CE4,
    0xE6CC: 0x8CE3,
    0xE6CD: 0x8CDA,
    0xE6CE: 0x8CFD,
    0xE6CF: 0x8CFA,
    0xE6D0: 0x8CFB,
    0xE6D1: 0x8D04,
    0xE6D2: 0x8D05,
    0xE6D3: 0x8D0A,
    0xE6D4: 0x8D07,
    0xE6D5: 0x8D0F,
    0xE6D6: 0x8D0D,
    0xE6D7: 0x8D10,
    0xE6D8: 0x9F4E,
    0xE6D9: 0x8D13,
    0xE6DA: 0x8CCD,
    0xE6DB: 0x8D14,
    0xE6DC: 0x8D16,
    0xE6DD: 0x8D67,
    0xE6DE: 0x8D6D,
    0xE6DF: 0x8D71,
    0xE6E0: 0x8D73,
    0xE6E1: 0x8D81,
    0xE6E2: 0x8D99,
    0xE6E3: 0x8DC2,
    0xE6E4: 0x8DBE,
    0xE6E5: 0x8DBA,
    0xE6E6: 0x8DCF,
    0xE6E7: 0x8DDA,
    0xE6E8: 0x8DD6,
    0xE6E9: 0x8DCC,
    0xE6EA: 0x8DDB,
    0xE6EB: 0x8DCB,
    0xE6EC: 0x8DEA,
    0xE6ED: 0x8DEB,
    0xE6EE: 0x8DDF,
    0xE6EF: 0x8DE3,
    0xE6F0: 0x8DFC,
    0xE6F1: 0x8E08,
    0xE6F2: 0x8E09,
    0xE6F3: 0x8DFF,
    0xE6F4: 0x8E1D,
    0xE6F5: 0x8E1E,
    0xE6F6: 0x8E10,
    0xE6F7: 0x8E1F,
    0xE6F8: 0x8E42,
    0xE6F9: 0x8E35,
    0xE6FA: 0x8E30,
    0xE6FB: 0x8E34,
    0xE6FC: 0x8E4A,
    0xE740: 0x8E47,
    0xE741: 0x8E49,
    0xE742: 0x8E4C,
    0xE743: 0x8E50,
    0xE744: 0x8E48,
    0xE745: 0x8E59,
    0xE746: 0x8E64,
    0xE747: 0x8E60,
    0xE748: 0x8E2A,
    0xE749: 0x8E63,
    0xE74A: 0x8E55,
    0xE74B: 0x8E76,
    0xE74C: 0x8E72,
    0xE74D: 0x8E7C,
    0xE74E: 0x8E81,
    0xE74F: 0x8E87,
    0xE750: 0x8E85,
    0xE751: 0x8E84,
    0xE752: 0x8E8B,
    0xE753: 0x8E8A,
    0xE754: 0x8E93,
    0xE755: 0x8E91,
    0xE756: 0x8E94,
    0xE757: 0x8E99,
    0xE758: 0x8EAA,
    0xE759: 0x8EA1,
    0xE75A: 0x8EAC,
    0xE75B: 0x8EB0,
    0xE75C: 0x8EC6,
    0xE75D: 0x8EB1,
    0xE75E: 0x8EBE,
    0xE75F: 0x8EC5,
    0xE760: 0x8EC8,
    0xE761: 0x8ECB,
    0xE762: 0x8EDB,
    0xE763: 0x8EE3,
    0xE764: 0x8EFC,
    0xE765: 0x8EFB,
    0xE766: 0x8EEB,
    0xE767: 0x8EFE,
    0xE768: 0x8F0A,
    0xE769: 0x8F05,
    0xE76A: 0x8F15,
    0xE76B: 0x8F12,
    0xE76C: 0x8F19,
    0xE76D: 0x8F13,
    0xE76E: 0x8F1C,
    0xE76F: 0x8F1F,
    0xE770: 0x8F1B,
    0xE771: 0x8F0C,
    0xE772: 0x8F26,
    0xE773: 0x8F33,
    0xE774: 0x8F3B,
    0xE775: 0x8F39,
    0xE776: 0x8F45,
    0xE777: 0x8F42,
    0xE778: 0x8F3E,
    0xE779: 0x8F4C,
    0xE77A: 0x8F49,
    0xE77B: 0x8F46,
    0xE77C: 0x8F4E,
    0xE77D: 0x8F57,
    0xE77E: 0x8F5C,
    0xE780: 0x8F62,
    0xE781: 0x8F63,
    0xE782: 0x8F64,
    0xE783: 0x8F9C,
    0xE784: 0x8F9F,
    0xE785: 0x8FA3,
    0xE786: 0x8FAD,
    0xE787: 0x8FAF,
    0xE788: 0x8FB7,
    0xE789: 0x8FDA,
    0xE78A: 0x8FE5,
    0xE78B: 0x8FE2,
    0xE78C: 0x8FEA,
    0xE78D: 0x8FEF,
    0xE78E: 0x9087,
    0xE78F: 0x8FF4,
    0xE790: 0x9005,
    0xE791: 0x8FF9,
    0xE792: 0x8FFA,
    0xE793: 0x9011,
    0xE794: 0x9015,
    0xE795: 0x9021,
    0xE796: 0x900D,
    0xE797: 0x901E,
    0xE798: 0x9016,
    0xE799: 0x900B,
    0xE79A: 0x9027,
    0xE79B: 0x9036,
    0xE79C: 0x9035,
    0xE79D: 0x9039,
    0xE79E: 0x8FF8,
    0xE79F: 0x904F,
    0xE7A0: 0x9050,
    0xE7A1: 0x9051,
    0xE7A2: 0x9052,
    0xE7A3: 0x900E,
    0xE7A4: 0x9049,
    0xE7A5: 0x903E,
    0xE7A6: 0x9056,
    0xE7A7: 0x9058,
    0xE7A8: 0x905E,
    0xE7A9: 0x9068,
    0xE7AA: 0x906F,
    0xE7AB: 0x9076,
    0xE7AC: 0x96A8,
    0xE7AD: 0x9072,
    0xE7AE: 0x9082,
    0xE7AF: 0x907D,
    0xE7B0: 0x9081,
    0xE7B1: 0x9080,
    0xE7B2: 0x908A,
    0xE7B3: 0x9089,
    0xE7B4: 0x908F,
    0xE7B5: 0x90A8,
    0xE7B6: 0x90AF,
    0xE7B7: 0x90B1,
    0xE7B8: 0x90B5,
    0xE7B9: 0x90E2,
    0xE7BA: 0x90E4,
    0xE7BB: 0x6248,
    0xE7BC: 0x90DB,
    0xE7BD: 0x9102,
    0xE7BE: 0x9112,
    0xE7BF: 0x9119,
    0xE7C0: 0x9132,
    0xE7C1: 0x9130,
    0xE7C2: 0x914A,
    0xE7C3: 0x9156,
    0xE7C4: 0x9158,
    0xE7C5: 0x9163,
    0xE7C6: 0x9165,
    0xE7C7: 0x9169,
    0xE7C8: 0x9173,
    0xE7C9: 0x9172,
    0xE7CA: 0x918B,
    0xE7CB: 0x9189,
    0xE7CC: 0x9182,
    0xE7CD: 0x91A2,
    0xE7CE: 0x91AB,
    0xE7CF: 0x91AF,
    0xE7D0: 0x91AA,
    0xE7D1: 0x91B5,
    0xE7D2: 0x91B4,
    0xE7D3: 0x91BA,
    0xE7D4: 0x91C0,
    0xE7D5: 0x91C1,
    0xE7D6: 0x91C9,
    0xE7D7: 0x91CB,
    0xE7D8: 0x91D0,
    0xE7D9: 0x91D6,
    0xE7DA: 0x91DF,
    0xE7DB: 0x91E1,
    0xE7DC: 0x91DB,
    0xE7DD: 0x91FC,
    0xE7DE: 0x91F5,
    0xE7DF: 0x91F6,
    0xE7E0: 0x921E,
    0xE7E1: 0x91FF,
    0xE7E2: 0x9214,
    0xE7E3: 0x922C,
    0xE7E4: 0x9215,
    0xE7E5: 0x9211,
    0xE7E6: 0x925E,
    0xE7E7: 0x9257,
    0xE7E8: 0x9245,
    0xE7E9: 0x9249,
    0xE7EA: 0x9264,
    0xE7EB: 0x9248,
    0xE7EC: 0x9295,
    0xE7ED: 0x923F,
    0xE7EE: 0x924B,
    0xE7EF: 0x9250,
    0xE7F0: 0x929C,
    0xE7F1: 0x9296,
    0xE7F2: 0x9293,
    0xE7F3: 0x929B,
    0xE7F4: 0x925A,
    0xE7F5: 0x92CF,
    0xE7F6: 0x92B9,
    0xE7F7: 0x92B7,
    0xE7F8: 0x92E9,
    0xE7F9: 0x930F,
    0xE7FA: 0x92FA,
    0xE7FB: 0x9344,
    0xE7FC: 0x932E,
    0xE840: 0x9319,
    0xE841: 0x9322,
    0xE842: 0x931A,
    0xE843: 0x9323,
    0xE844: 0x933A,
    0xE845: 0x9335,
    0xE846: 0x933B,
    0xE847: 0x935C,
    0xE848: 0x9360,
    0xE849: 0x937C,
    0xE84A: 0x936E,
    0xE84B: 0x9356,
    0xE84C: 0x93B0,
    0xE84D: 0x93AC,
    0xE84E: 0x93AD,
    0xE84F: 0x9394,
    0xE850: 0x93B9,
    0xE851: 0x93D6,
    0xE852: 0x93D7,
    0xE853: 0x93E8,
    0xE854: 0x93E5,
    0xE855: 0x93D8,
    0xE856: 0x93C3,
    0xE857: 0x93DD,
    0xE858: 0x93D0,
    0xE859: 0x93C8,
    0xE85A: 0x93E4,
    0xE85B: 0x941A,
    0xE85C: 0x9414,
    0xE85D: 0x9413,
    0xE85E: 0x9403,
    0xE85F: 0x9407,
    0xE860: 0x9410,
    0xE861: 0x9436,
    0xE862: 0x942B,
    0xE863: 0x9435,
    0xE864: 0x9421,
    0xE865: 0x943A,
    0xE866: 0x9441,
    0xE867: 0x9452,
    0xE868: 0x9444,
    0xE869: 0x945B,
    0xE86A: 0x9460,
    0xE86B: 0x9462,
    0xE86C: 0x945E,
    0xE86D: 0x946A,
    0xE86E: 0x9229,
    0xE86F: 0x9470,
    0xE870: 0x9475,
    0xE871: 0x9477,
    0xE872: 0x947D,
    0xE873: 0x945A,
    0xE874: 0x947C,
    0xE875: 0x947E,
    0xE876: 0x9481,
    0xE877: 0x947F,
    0xE878: 0x9582,
    0xE879: 0x9587,
    0xE87A: 0x958A,
    0xE87B: 0x9594,
    0xE87C: 0x9596,
    0xE87D: 0x9598,
    0xE87E: 0x9599,
    0xE880: 0x95A0,
    0xE881: 0x95A8,
    0xE882: 0x95A7,
    0xE883: 0x95AD,
    0xE884: 0x95BC,
    0xE885: 0x95BB,
    0xE886: 0x95B9,
    0xE887: 0x95BE,
    0xE888: 0x95CA,
    0xE889: 0x6FF6,
    0xE88A: 0x95C3,
    0xE88B: 0x95CD,
    0xE88C: 0x95CC,
    0xE88D: 0x95D5,
    0xE88E: 0x95D4,
    0xE88F: 0x95D6,
    0xE890: 0x95DC,
    0xE891: 0x95E1,
    0xE892: 0x95E5,
    0xE893: 0x95E2,
    0xE894: 0x9621,
    0xE895: 0x9628,
    0xE896: 0x962E,
    0xE897: 0x962F,
    0xE898: 0x9642,
    0xE899: 0x964C,
    0xE89A: 0x964F,
    0xE89B: 0x964B,
    0xE89C: 0x9677,
    0xE89D: 0x965C,
    0xE89E: 0x965E,
    0xE89F: 0x965D,
    0xE8A0: 0x965F,
    0xE8A1: 0x9666,
    0xE8A2: 0x9672,
    0xE8A3: 0x966C,
    0xE8A4: 0x968D,
    0xE8A5: 0x9698,
    0xE8A6: 0x9695,
    0xE8A7: 0x9697,
    0xE8A8: 0x96AA,
    0xE8A9: 0x96A7,
    0xE8AA: 0x96B1,
    0xE8AB: 0x96B2,
    0xE8AC: 0x96B0,
    0xE8AD: 0x96B4,
    0xE8AE: 0x96B6,
    0xE8AF: 0x96B8,
    0xE8B0: 0x96B9,
    0xE8B1: 0x96CE,
    0xE8B2: 0x96CB,
    0xE8B3: 0x96C9,
    0xE8B4: 0x96CD,
    0xE8B5: 0x894D,
    0xE8B6: 0x96DC,
    0xE8B7: 0x970D,
    0xE8B8: 0x96D5,
    0xE8B9: 0x96F9,
    0xE8BA: 0x9704,
    0xE8BB: 0x9706,
    0xE8BC: 0x9708,
    0xE8BD: 0x9713,
    0xE8BE: 0x970E,
    0xE8BF: 0x9711,
    0xE8C0: 0x970F,
    0xE8C1: 0x9716,
    0xE8C2: 0x9719,
    0xE8C3: 0x9724,
    0xE8C4: 0x972A,
    0xE8C5: 0x9730,
    0xE8C6: 0x9739,
    0xE8C7: 0x973D,
    0xE8C8: 0x973E,
    0xE8C9: 0x9744,
    0xE8CA: 0x9746,
    0xE8CB: 0x9748,
    0xE8CC: 0x9742,
    0xE8CD: 0x9749,
    0xE8CE: 0x975C,
    0xE8CF: 0x9760,
    0xE8D0: 0x9764,
    0xE8D1: 0x9766,
    0xE8D2: 0x9768,
    0xE8D3: 0x52D2,
    0xE8D4: 0x976B,
    0xE8D5: 0x9771,
    0xE8D6: 0x9779,
    0xE8D7: 0x9785,
    0xE8D8: 0x977C,
    0xE8D9: 0x9781,
    0xE8DA: 0x977A,
    0xE8DB: 0x9786,
    0xE8DC: 0x978B,
    0xE8DD: 0x978F,
    0xE8DE: 0x9790,
    0xE8DF: 0x979C,
    0xE8E0: 0x97A8,
    0xE8E1: 0x97A6,
    0xE8E2: 0x97A3,
    0xE8E3: 0x97B3,
    0xE8E4: 0x97B4,
    0xE8E5: 0x97C3,
    0xE8E6: 0x97C6,
    0xE8E7: 0x97C8,
    0xE8E8: 0x97CB,
    0xE8E9: 0x97DC,
    0xE8EA: 0x97ED,
    0xE8EB: 0x9F4F,
    0xE8EC: 0x97F2,
    0xE8ED: 0x7ADF,
    0xE8EE: 0x97F6,
    0xE8EF: 0x97F5,
    0xE8F0: 0x980F,
    0xE8F1: 0x980C,
    0xE8F2: 0x9838,
    0xE8F3: 0x9824,
    0xE8F4: 0x9821,
    0xE8F5: 0x9837,
    0xE8F6: 0x983D,
    0xE8F7: 0x9846,
    0xE8F8: 0x984F,
    0xE8F9: 0x984B,
    0xE8FA: 0x986B,
    0xE8FB: 0x986F,
    0xE8FC: 0x9870,
    0xE940: 0x9871,
    0xE941: 0x9874,
    0xE942: 0x9873,
    0xE943: 0x98AA,
    0xE944: 0x98AF,
    0xE945: 0x98B1,
    0xE946: 0x98B6,
    0xE947: 0x98C4,
    0xE948: 0x98C3,
    0xE949: 0x98C6,
    0xE94A: 0x98E9,
    0xE94B: 0x98EB,
    0xE94C: 0x9903,
    0xE94D: 0x9909,
    0xE94E: 0x9912,
    0xE94F: 0x9914,
    0xE950: 0x9918,
    0xE951: 0x9921,
    0xE952: 0x991D,
    0xE953: 0x991E,
    0xE954: 0x9924,
    0xE955: 0x9920,
    0xE956: 0x992C,
    0xE957: 0x992E,
    0xE958: 0x993D,
    0xE959: 0x993E,
    0xE95A: 0x9942,
    0xE95B: 0x9949,
    0xE95C: 0x9945,
    0xE95D: 0x9950,
    0xE95E: 0x994B,
    0xE95F: 0x9951,
    0xE960: 0x9952,
    0xE961: 0x994C,
    0xE962: 0x9955,
    0xE963: 0x9997,
    0xE964: 0x9998,
    0xE965: 0x99A5,
    0xE966: 0x99AD,
    0xE967: 0x99AE,
    0xE968: 0x99BC,
    0xE969: 0x99DF,
    0xE96A: 0x99DB,
    0xE96B: 0x99DD,
    0xE96C: 0x99D8,
    0xE96D: 0x99D1,
    0xE96E: 0x99ED,
    0xE96F: 0x99EE,
    0xE970: 0x99F1,
    0xE971: 0x99F2,
    0xE972: 0x99FB,
    0xE973: 0x99F8,
    0xE974: 0x9A01,
    0xE975: 0x9A0F,
    0xE976: 0x9A05,
    0xE977: 0x99E2,
    0xE978: 0x9A19,
    0xE979: 0x9A2B,
    0xE97A: 0x9A37,
    0xE97B: 0x9A45,
    0xE97C: 0x9A42,
    0xE97D: 0x9A40,
    0xE97E: 0x9A43,
    0xE980: 0x9A3E,
    0xE981: 0x9A55,
    0xE982: 0x9A4D,
    0xE983: 0x9A5B,
    0xE984: 0x9A57,
    0xE985: 0x9A5F,
    0xE986: 0x9A62,
    0xE987: 0x9A65,
    0xE988: 0x9A64,
    0xE989: 0x9A69,
    0xE98A: 0x9A6B,
    0xE98B: 0x9A6A,
    0xE98C: 0x9AAD,
    0xE98D: 0x9AB0,
    0xE98E: 0x9ABC,
    0xE98F: 0x9AC0,
    0xE990: 0x9ACF,
    0xE991: 0x9AD1,
    0xE992: 0x9AD3,
    0xE993: 0x9AD4,
    0xE994: 0x9ADE,
    0xE995: 0x9ADF,
    0xE996: 0x9AE2,
    0xE997: 0x9AE3,
    0xE998: 0x9AE6,
    0xE999: 0x9AEF,
    0xE99A: 0x9AEB,
    0xE99B: 0x9AEE,
    0xE99C: 0x9AF4,
    0xE99D: 0x9AF1,
    0xE99E: 0x9AF7,
    0xE99F: 0x9AFB,
    0xE9A0: 0x9B06,
    0xE9A1: 0x9B18,
    0xE9A2: 0x9B1A,
    0xE9A3: 0x9B1F,
    0xE9A4: 0x9B22,
    0xE9A5: 0x9B23,
    0xE9A6: 0x9B25,
    0xE9A7: 0x9B27,
    0xE9A8: 0x9B28,
    0xE9A9: 0x9B29,
    0xE9AA: 0x9B2A,
    0xE9AB: 0x9B2E,
    0xE9AC: 0x9B2F,
    0xE9AD: 0x9B32,
    0xE9AE: 0x9B44,
    0xE9AF: 0x9B43,
    0xE9B0: 0x9B4F,
    0xE9B1: 0x9B4D,
    0xE9B2: 0x9B4E,
    0xE9B3: 0x9B51,
    0xE9B4: 0x9B58,
    0xE9B5: 0x9B74,
    0xE9B6: 0x9B93,
    0xE9B7: 0x9B83,
    0xE9B8: 0x9B91,
    0xE9B9: 0x9B96,
    0xE9BA: 0x9B97,
    0xE9BB: 0x9B9F,
    0xE9BC: 0x9BA0,
    0xE9BD: 0x9BA8,
    0xE9BE: 0x9BB4,
    0xE9BF: 0x9BC0,
    0xE9C0: 0x9BCA,
    0xE9C1: 0x9BB9,
    0xE9C2: 0x9BC6,
    0xE9C3: 0x9BCF,
    0xE9C4: 0x9BD1,
    0xE9C5: 0x9BD2,
    0xE9C6: 0x9BE3,
    0xE9C7: 0x9BE2,
    0xE9C8: 0x9BE4,
    0xE9C9: 0x9BD4,
    0xE9CA: 0x9BE1,
    0xE9CB: 0x9C3A,
    0xE9CC: 0x9BF2,
    0xE9CD: 0x9BF1,
    0xE9CE: 0x9BF0,
    0xE9CF: 0x9C15,
    0xE9D0: 0x9C14,
    0xE9D1: 0x9C09,
    0xE9D2: 0x9C13,
    0xE9D3: 0x9C0C,
    0xE9D4: 0x9C06,
    0xE9D5: 0x9C08,
    0xE9D6: 0x9C12,
    0xE9D7: 0x9C0A,
    0xE9D8: 0x9C04,
    0xE9D9: 0x9C2E,
    0xE9DA: 0x9C1B,
    0xE9DB: 0x9C25,
    0xE9DC: 0x9C24,
    0xE9DD: 0x9C21,
    0xE9DE: 0x9C30,
    0xE9DF: 0x9C47,
    0xE9E0: 0x9C32,
    0xE9E1: 0x9C46,
    0xE9E2: 0x9C3E,
    0xE9E3: 0x9C5A,
    0xE9E4: 0x9C60,
    0xE9E5: 0x9C67,
    0xE9E6: 0x9C76,
    0xE9E7: 0x9C78,
    0xE9E8: 0x9CE7,
    0xE9E9: 0x9CEC,
    0xE9EA: 0x9CF0,
    0xE9EB: 0x9D09,
    0xE9EC: 0x9D08,
    0xE9ED: 0x9CEB,
    0xE9EE: 0x9D03,
    0xE9EF: 0x9D06,
    0xE9F0: 0x9D2A,
    0xE9F1: 0x9D26,
    0xE9F2: 0x9DAF,
    0xE9F3: 0x9D23,
    0xE9F4: 0x9D1F,
    0xE9F5: 0x9D44,
    0xE9F6: 0x9D15,
    0xE9F7: 0x9D12,
    0xE9F8: 0x9D41,
    0xE9F9: 0x9D3F,
    0xE9FA: 0x9D3E,
    0xE9FB: 0x9D46,
    0xE9FC: 0x9D48,
    0xEA40: 0x9D5D,
    0xEA41: 0x9D5E,
    0xEA42: 0x9D64,
    0xEA43: 0x9D51,
    0xEA44: 0x9D50,
    0xEA45: 0x9D59,
    0xEA46: 0x9D72,
    0xEA47: 0x9D89,
    0xEA48: 0x9D87,
    0xEA49: 0x9DAB,
    0xEA4A: 0x9D6F,
    0xEA4B: 0x9D7A,
    0xEA4C: 0x9D9A,
    0xEA4D: 0x9DA4,
    0xEA4E: 0x9DA9,
    0xEA4F: 0x9DB2,
    0xEA50: 0x9DC4,
    0xEA51: 0x9DC1,
    0xEA52: 0x9DBB,
    0xEA53: 0x9DB8,
    0xEA54: 0x9DBA,
    0xEA55: 0x9DC6,
    0xEA56: 0x9DCF,
    0xEA57: 0x9DC2,
    0xEA58: 0x9DD9,
    0xEA59: 0x9DD3,
    0xEA5A: 0x9DF8,
    0xEA5B: 0x9DE6,
    0xEA5C: 0x9DED,
    0xEA5D: 0x9DEF,
    0xEA5E: 0x9DFD,
    0xEA5F: 0x9E1A,
    0xEA60: 0x9E1B,
    0xEA61: 0x9E1E,
    0xEA62: 0x9E75,
    0xEA63: 0x9E79,
    0xEA64: 0x9E7D,
    0xEA65: 0x9E81,
    0xEA66: 0x9E88,
    0xEA67: 0x9E8B,
    0xEA68: 0x9E8C,
    0xEA69: 0x9E92,
    0xEA6A: 0x9E95,
    0xEA6B: 0x9E91,
    0xEA6C: 0x9E9D,
    0xEA6D: 0x9EA5,
    0xEA6E: 0x9EA9,
    0xEA6F: 0x9EB8,
    0xEA70: 0x9EAA,
    0xEA71: 0x9EAD,
    0xEA72: 0x9761,
    0xEA73: 0x9ECC,
    0xEA74: 0x9ECE,
    0xEA75: 0x9ECF,
    0xEA76: 0x9ED0,
    0xEA77: 0x9ED4,
    0xEA78: 0x9EDC,
    0xEA79: 0x9EDE,
    0xEA7A: 0x9EDD,
    0xEA7B: 0x9EE0,
    0xEA7C: 0x9EE5,
    0xEA7D: 0x9EE8,
    0xEA7E: 0x9EEF,
    0xEA80: 0x9EF4,
    0xEA81: 0x9EF6,
    0xEA82: 0x9EF7,
    0xEA83: 0x9EF9,
    0xEA84: 0x9EFB,
    0xEA85: 0x9EFC,
    0xEA86: 0x9EFD,
    0xEA87: 0x9F07,
    0xEA88: 0x9F08,
    0xEA89: 0x76B7,
    0xEA8A: 0x9F15,
    0xEA8B: 0x9F21,
    0xEA8C: 0x9F2C,
    0xEA8D: 0x9F3E,
    0xEA8E: 0x9F4A,
    0xEA8F: 0x9F52,
    0xEA90: 0x9F54,
    0xEA91: 0x9F63,
    0xEA92: 0x9F5F,
    0xEA93: 0x9F60,
    0xEA94: 0x9F61,
    0xEA95: 0x9F66,
    0xEA96: 0x9F67,
    0xEA97: 0x9F6C,
    0xEA98: 0x9F6A,
    0xEA99: 0x9F77,
    0xEA9A: 0x9F72,
    0xEA9B: 0x9F76,
    0xEA9C: 0x9F95,
    0xEA9D: 0x9F9C,
    0xEA9E: 0x9FA0,
    0xEA9F: 0x582F,
    0xEAA0: 0x69C7,
    0xEAA1: 0x9059,
    0xEAA2: 0x7464,
    0xEAA3: 0x51DC,
    0xEAA4: 0x7199,
};


/***/ }),
/* 9 */
/***/ (function(module, exports, __webpack_require__) {

"use strict";

Object.defineProperty(exports, "__esModule", { value: true });
var GenericGF_1 = __webpack_require__(1);
var GenericGFPoly_1 = __webpack_require__(2);
function runEuclideanAlgorithm(field, a, b, R) {
    var _a;
    // Assume a's degree is >= b's
    if (a.degree() < b.degree()) {
        _a = [b, a], a = _a[0], b = _a[1];
    }
    var rLast = a;
    var r = b;
    var tLast = field.zero;
    var t = field.one;
    // Run Euclidean algorithm until r's degree is less than R/2
    while (r.degree() >= R / 2) {
        var rLastLast = rLast;
        var tLastLast = tLast;
        rLast = r;
        tLast = t;
        // Divide rLastLast by rLast, with quotient in q and remainder in r
        if (rLast.isZero()) {
            // Euclidean algorithm already terminated?
            return null;
        }
        r = rLastLast;
        var q = field.zero;
        var denominatorLeadingTerm = rLast.getCoefficient(rLast.degree());
        var dltInverse = field.inverse(denominatorLeadingTerm);
        while (r.degree() >= rLast.degree() && !r.isZero()) {
            var degreeDiff = r.degree() - rLast.degree();
            var scale = field.multiply(r.getCoefficient(r.degree()), dltInverse);
            q = q.addOrSubtract(field.buildMonomial(degreeDiff, scale));
            r = r.addOrSubtract(rLast.multiplyByMonomial(degreeDiff, scale));
        }
        t = q.multiplyPoly(tLast).addOrSubtract(tLastLast);
        if (r.degree() >= rLast.degree()) {
            return null;
        }
    }
    var sigmaTildeAtZero = t.getCoefficient(0);
    if (sigmaTildeAtZero === 0) {
        return null;
    }
    var inverse = field.inverse(sigmaTildeAtZero);
    return [t.multiply(inverse), r.multiply(inverse)];
}
function findErrorLocations(field, errorLocator) {
    // This is a direct application of Chien's search
    var numErrors = errorLocator.degree();
    if (numErrors === 1) {
        return [errorLocator.getCoefficient(1)];
    }
    var result = new Array(numErrors);
    var errorCount = 0;
    for (var i = 1; i < field.size && errorCount < numErrors; i++) {
        if (errorLocator.evaluateAt(i) === 0) {
            result[errorCount] = field.inverse(i);
            errorCount++;
        }
    }
    if (errorCount !== numErrors) {
        return null;
    }
    return result;
}
function findErrorMagnitudes(field, errorEvaluator, errorLocations) {
    // This is directly applying Forney's Formula
    var s = errorLocations.length;
    var result = new Array(s);
    for (var i = 0; i < s; i++) {
        var xiInverse = field.inverse(errorLocations[i]);
        var denominator = 1;
        for (var j = 0; j < s; j++) {
            if (i !== j) {
                denominator = field.multiply(denominator, GenericGF_1.addOrSubtractGF(1, field.multiply(errorLocations[j], xiInverse)));
            }
        }
        result[i] = field.multiply(errorEvaluator.evaluateAt(xiInverse), field.inverse(denominator));
        if (field.generatorBase !== 0) {
            result[i] = field.multiply(result[i], xiInverse);
        }
    }
    return result;
}
function decode(bytes, twoS) {
    var outputBytes = new Uint8ClampedArray(bytes.length);
    outputBytes.set(bytes);
    var field = new GenericGF_1.default(0x011D, 256, 0); // x^8 + x^4 + x^3 + x^2 + 1
    var poly = new GenericGFPoly_1.default(field, outputBytes);
    var syndromeCoefficients = new Uint8ClampedArray(twoS);
    var error = false;
    for (var s = 0; s < twoS; s++) {
        var evaluation = poly.evaluateAt(field.exp(s + field.generatorBase));
        syndromeCoefficients[syndromeCoefficients.length - 1 - s] = evaluation;
        if (evaluation !== 0) {
            error = true;
        }
    }
    if (!error) {
        return outputBytes;
    }
    var syndrome = new GenericGFPoly_1.default(field, syndromeCoefficients);
    var sigmaOmega = runEuclideanAlgorithm(field, field.buildMonomial(twoS, 1), syndrome, twoS);
    if (sigmaOmega === null) {
        return null;
    }
    var errorLocations = findErrorLocations(field, sigmaOmega[0]);
    if (errorLocations == null) {
        return null;
    }
    var errorMagnitudes = findErrorMagnitudes(field, sigmaOmega[1], errorLocations);
    for (var i = 0; i < errorLocations.length; i++) {
        var position = outputBytes.length - 1 - field.log(errorLocations[i]);
        if (position < 0) {
            return null;
        }
        outputBytes[position] = GenericGF_1.addOrSubtractGF(outputBytes[position], errorMagnitudes[i]);
    }
    return outputBytes;
}
exports.decode = decode;


/***/ }),
/* 10 */
/***/ (function(module, exports, __webpack_require__) {

"use strict";

Object.defineProperty(exports, "__esModule", { value: true });
exports.VERSIONS = [
    {
        infoBits: null,
        versionNumber: 1,
        alignmentPatternCenters: [],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 7,
                ecBlocks: [{ numBlocks: 1, dataCodewordsPerBlock: 19 }],
            },
            {
                ecCodewordsPerBlock: 10,
                ecBlocks: [{ numBlocks: 1, dataCodewordsPerBlock: 16 }],
            },
            {
                ecCodewordsPerBlock: 13,
                ecBlocks: [{ numBlocks: 1, dataCodewordsPerBlock: 13 }],
            },
            {
                ecCodewordsPerBlock: 17,
                ecBlocks: [{ numBlocks: 1, dataCodewordsPerBlock: 9 }],
            },
        ],
    },
    {
        infoBits: null,
        versionNumber: 2,
        alignmentPatternCenters: [6, 18],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 10,
                ecBlocks: [{ numBlocks: 1, dataCodewordsPerBlock: 34 }],
            },
            {
                ecCodewordsPerBlock: 16,
                ecBlocks: [{ numBlocks: 1, dataCodewordsPerBlock: 28 }],
            },
            {
                ecCodewordsPerBlock: 22,
                ecBlocks: [{ numBlocks: 1, dataCodewordsPerBlock: 22 }],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [{ numBlocks: 1, dataCodewordsPerBlock: 16 }],
            },
        ],
    },
    {
        infoBits: null,
        versionNumber: 3,
        alignmentPatternCenters: [6, 22],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 15,
                ecBlocks: [{ numBlocks: 1, dataCodewordsPerBlock: 55 }],
            },
            {
                ecCodewordsPerBlock: 26,
                ecBlocks: [{ numBlocks: 1, dataCodewordsPerBlock: 44 }],
            },
            {
                ecCodewordsPerBlock: 18,
                ecBlocks: [{ numBlocks: 2, dataCodewordsPerBlock: 17 }],
            },
            {
                ecCodewordsPerBlock: 22,
                ecBlocks: [{ numBlocks: 2, dataCodewordsPerBlock: 13 }],
            },
        ],
    },
    {
        infoBits: null,
        versionNumber: 4,
        alignmentPatternCenters: [6, 26],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 20,
                ecBlocks: [{ numBlocks: 1, dataCodewordsPerBlock: 80 }],
            },
            {
                ecCodewordsPerBlock: 18,
                ecBlocks: [{ numBlocks: 2, dataCodewordsPerBlock: 32 }],
            },
            {
                ecCodewordsPerBlock: 26,
                ecBlocks: [{ numBlocks: 2, dataCodewordsPerBlock: 24 }],
            },
            {
                ecCodewordsPerBlock: 16,
                ecBlocks: [{ numBlocks: 4, dataCodewordsPerBlock: 9 }],
            },
        ],
    },
    {
        infoBits: null,
        versionNumber: 5,
        alignmentPatternCenters: [6, 30],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 26,
                ecBlocks: [{ numBlocks: 1, dataCodewordsPerBlock: 108 }],
            },
            {
                ecCodewordsPerBlock: 24,
                ecBlocks: [{ numBlocks: 2, dataCodewordsPerBlock: 43 }],
            },
            {
                ecCodewordsPerBlock: 18,
                ecBlocks: [
                    { numBlocks: 2, dataCodewordsPerBlock: 15 },
                    { numBlocks: 2, dataCodewordsPerBlock: 16 },
                ],
            },
            {
                ecCodewordsPerBlock: 22,
                ecBlocks: [
                    { numBlocks: 2, dataCodewordsPerBlock: 11 },
                    { numBlocks: 2, dataCodewordsPerBlock: 12 },
                ],
            },
        ],
    },
    {
        infoBits: null,
        versionNumber: 6,
        alignmentPatternCenters: [6, 34],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 18,
                ecBlocks: [{ numBlocks: 2, dataCodewordsPerBlock: 68 }],
            },
            {
                ecCodewordsPerBlock: 16,
                ecBlocks: [{ numBlocks: 4, dataCodewordsPerBlock: 27 }],
            },
            {
                ecCodewordsPerBlock: 24,
                ecBlocks: [{ numBlocks: 4, dataCodewordsPerBlock: 19 }],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [{ numBlocks: 4, dataCodewordsPerBlock: 15 }],
            },
        ],
    },
    {
        infoBits: 0x07C94,
        versionNumber: 7,
        alignmentPatternCenters: [6, 22, 38],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 20,
                ecBlocks: [{ numBlocks: 2, dataCodewordsPerBlock: 78 }],
            },
            {
                ecCodewordsPerBlock: 18,
                ecBlocks: [{ numBlocks: 4, dataCodewordsPerBlock: 31 }],
            },
            {
                ecCodewordsPerBlock: 18,
                ecBlocks: [
                    { numBlocks: 2, dataCodewordsPerBlock: 14 },
                    { numBlocks: 4, dataCodewordsPerBlock: 15 },
                ],
            },
            {
                ecCodewordsPerBlock: 26,
                ecBlocks: [
                    { numBlocks: 4, dataCodewordsPerBlock: 13 },
                    { numBlocks: 1, dataCodewordsPerBlock: 14 },
                ],
            },
        ],
    },
    {
        infoBits: 0x085BC,
        versionNumber: 8,
        alignmentPatternCenters: [6, 24, 42],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 24,
                ecBlocks: [{ numBlocks: 2, dataCodewordsPerBlock: 97 }],
            },
            {
                ecCodewordsPerBlock: 22,
                ecBlocks: [
                    { numBlocks: 2, dataCodewordsPerBlock: 38 },
                    { numBlocks: 2, dataCodewordsPerBlock: 39 },
                ],
            },
            {
                ecCodewordsPerBlock: 22,
                ecBlocks: [
                    { numBlocks: 4, dataCodewordsPerBlock: 18 },
                    { numBlocks: 2, dataCodewordsPerBlock: 19 },
                ],
            },
            {
                ecCodewordsPerBlock: 26,
                ecBlocks: [
                    { numBlocks: 4, dataCodewordsPerBlock: 14 },
                    { numBlocks: 2, dataCodewordsPerBlock: 15 },
                ],
            },
        ],
    },
    {
        infoBits: 0x09A99,
        versionNumber: 9,
        alignmentPatternCenters: [6, 26, 46],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [{ numBlocks: 2, dataCodewordsPerBlock: 116 }],
            },
            {
                ecCodewordsPerBlock: 22,
                ecBlocks: [
                    { numBlocks: 3, dataCodewordsPerBlock: 36 },
                    { numBlocks: 2, dataCodewordsPerBlock: 37 },
                ],
            },
            {
                ecCodewordsPerBlock: 20,
                ecBlocks: [
                    { numBlocks: 4, dataCodewordsPerBlock: 16 },
                    { numBlocks: 4, dataCodewordsPerBlock: 17 },
                ],
            },
            {
                ecCodewordsPerBlock: 24,
                ecBlocks: [
                    { numBlocks: 4, dataCodewordsPerBlock: 12 },
                    { numBlocks: 4, dataCodewordsPerBlock: 13 },
                ],
            },
        ],
    },
    {
        infoBits: 0x0A4D3,
        versionNumber: 10,
        alignmentPatternCenters: [6, 28, 50],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 18,
                ecBlocks: [
                    { numBlocks: 2, dataCodewordsPerBlock: 68 },
                    { numBlocks: 2, dataCodewordsPerBlock: 69 },
                ],
            },
            {
                ecCodewordsPerBlock: 26,
                ecBlocks: [
                    { numBlocks: 4, dataCodewordsPerBlock: 43 },
                    { numBlocks: 1, dataCodewordsPerBlock: 44 },
                ],
            },
            {
                ecCodewordsPerBlock: 24,
                ecBlocks: [
                    { numBlocks: 6, dataCodewordsPerBlock: 19 },
                    { numBlocks: 2, dataCodewordsPerBlock: 20 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 6, dataCodewordsPerBlock: 15 },
                    { numBlocks: 2, dataCodewordsPerBlock: 16 },
                ],
            },
        ],
    },
    {
        infoBits: 0x0BBF6,
        versionNumber: 11,
        alignmentPatternCenters: [6, 30, 54],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 20,
                ecBlocks: [{ numBlocks: 4, dataCodewordsPerBlock: 81 }],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 1, dataCodewordsPerBlock: 50 },
                    { numBlocks: 4, dataCodewordsPerBlock: 51 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 4, dataCodewordsPerBlock: 22 },
                    { numBlocks: 4, dataCodewordsPerBlock: 23 },
                ],
            },
            {
                ecCodewordsPerBlock: 24,
                ecBlocks: [
                    { numBlocks: 3, dataCodewordsPerBlock: 12 },
                    { numBlocks: 8, dataCodewordsPerBlock: 13 },
                ],
            },
        ],
    },
    {
        infoBits: 0x0C762,
        versionNumber: 12,
        alignmentPatternCenters: [6, 32, 58],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 24,
                ecBlocks: [
                    { numBlocks: 2, dataCodewordsPerBlock: 92 },
                    { numBlocks: 2, dataCodewordsPerBlock: 93 },
                ],
            },
            {
                ecCodewordsPerBlock: 22,
                ecBlocks: [
                    { numBlocks: 6, dataCodewordsPerBlock: 36 },
                    { numBlocks: 2, dataCodewordsPerBlock: 37 },
                ],
            },
            {
                ecCodewordsPerBlock: 26,
                ecBlocks: [
                    { numBlocks: 4, dataCodewordsPerBlock: 20 },
                    { numBlocks: 6, dataCodewordsPerBlock: 21 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 7, dataCodewordsPerBlock: 14 },
                    { numBlocks: 4, dataCodewordsPerBlock: 15 },
                ],
            },
        ],
    },
    {
        infoBits: 0x0D847,
        versionNumber: 13,
        alignmentPatternCenters: [6, 34, 62],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 26,
                ecBlocks: [{ numBlocks: 4, dataCodewordsPerBlock: 107 }],
            },
            {
                ecCodewordsPerBlock: 22,
                ecBlocks: [
                    { numBlocks: 8, dataCodewordsPerBlock: 37 },
                    { numBlocks: 1, dataCodewordsPerBlock: 38 },
                ],
            },
            {
                ecCodewordsPerBlock: 24,
                ecBlocks: [
                    { numBlocks: 8, dataCodewordsPerBlock: 20 },
                    { numBlocks: 4, dataCodewordsPerBlock: 21 },
                ],
            },
            {
                ecCodewordsPerBlock: 22,
                ecBlocks: [
                    { numBlocks: 12, dataCodewordsPerBlock: 11 },
                    { numBlocks: 4, dataCodewordsPerBlock: 12 },
                ],
            },
        ],
    },
    {
        infoBits: 0x0E60D,
        versionNumber: 14,
        alignmentPatternCenters: [6, 26, 46, 66],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 3, dataCodewordsPerBlock: 115 },
                    { numBlocks: 1, dataCodewordsPerBlock: 116 },
                ],
            },
            {
                ecCodewordsPerBlock: 24,
                ecBlocks: [
                    { numBlocks: 4, dataCodewordsPerBlock: 40 },
                    { numBlocks: 5, dataCodewordsPerBlock: 41 },
                ],
            },
            {
                ecCodewordsPerBlock: 20,
                ecBlocks: [
                    { numBlocks: 11, dataCodewordsPerBlock: 16 },
                    { numBlocks: 5, dataCodewordsPerBlock: 17 },
                ],
            },
            {
                ecCodewordsPerBlock: 24,
                ecBlocks: [
                    { numBlocks: 11, dataCodewordsPerBlock: 12 },
                    { numBlocks: 5, dataCodewordsPerBlock: 13 },
                ],
            },
        ],
    },
    {
        infoBits: 0x0F928,
        versionNumber: 15,
        alignmentPatternCenters: [6, 26, 48, 70],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 22,
                ecBlocks: [
                    { numBlocks: 5, dataCodewordsPerBlock: 87 },
                    { numBlocks: 1, dataCodewordsPerBlock: 88 },
                ],
            },
            {
                ecCodewordsPerBlock: 24,
                ecBlocks: [
                    { numBlocks: 5, dataCodewordsPerBlock: 41 },
                    { numBlocks: 5, dataCodewordsPerBlock: 42 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 5, dataCodewordsPerBlock: 24 },
                    { numBlocks: 7, dataCodewordsPerBlock: 25 },
                ],
            },
            {
                ecCodewordsPerBlock: 24,
                ecBlocks: [
                    { numBlocks: 11, dataCodewordsPerBlock: 12 },
                    { numBlocks: 7, dataCodewordsPerBlock: 13 },
                ],
            },
        ],
    },
    {
        infoBits: 0x10B78,
        versionNumber: 16,
        alignmentPatternCenters: [6, 26, 50, 74],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 24,
                ecBlocks: [
                    { numBlocks: 5, dataCodewordsPerBlock: 98 },
                    { numBlocks: 1, dataCodewordsPerBlock: 99 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 7, dataCodewordsPerBlock: 45 },
                    { numBlocks: 3, dataCodewordsPerBlock: 46 },
                ],
            },
            {
                ecCodewordsPerBlock: 24,
                ecBlocks: [
                    { numBlocks: 15, dataCodewordsPerBlock: 19 },
                    { numBlocks: 2, dataCodewordsPerBlock: 20 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 3, dataCodewordsPerBlock: 15 },
                    { numBlocks: 13, dataCodewordsPerBlock: 16 },
                ],
            },
        ],
    },
    {
        infoBits: 0x1145D,
        versionNumber: 17,
        alignmentPatternCenters: [6, 30, 54, 78],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 1, dataCodewordsPerBlock: 107 },
                    { numBlocks: 5, dataCodewordsPerBlock: 108 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 10, dataCodewordsPerBlock: 46 },
                    { numBlocks: 1, dataCodewordsPerBlock: 47 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 1, dataCodewordsPerBlock: 22 },
                    { numBlocks: 15, dataCodewordsPerBlock: 23 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 2, dataCodewordsPerBlock: 14 },
                    { numBlocks: 17, dataCodewordsPerBlock: 15 },
                ],
            },
        ],
    },
    {
        infoBits: 0x12A17,
        versionNumber: 18,
        alignmentPatternCenters: [6, 30, 56, 82],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 5, dataCodewordsPerBlock: 120 },
                    { numBlocks: 1, dataCodewordsPerBlock: 121 },
                ],
            },
            {
                ecCodewordsPerBlock: 26,
                ecBlocks: [
                    { numBlocks: 9, dataCodewordsPerBlock: 43 },
                    { numBlocks: 4, dataCodewordsPerBlock: 44 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 17, dataCodewordsPerBlock: 22 },
                    { numBlocks: 1, dataCodewordsPerBlock: 23 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 2, dataCodewordsPerBlock: 14 },
                    { numBlocks: 19, dataCodewordsPerBlock: 15 },
                ],
            },
        ],
    },
    {
        infoBits: 0x13532,
        versionNumber: 19,
        alignmentPatternCenters: [6, 30, 58, 86],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 3, dataCodewordsPerBlock: 113 },
                    { numBlocks: 4, dataCodewordsPerBlock: 114 },
                ],
            },
            {
                ecCodewordsPerBlock: 26,
                ecBlocks: [
                    { numBlocks: 3, dataCodewordsPerBlock: 44 },
                    { numBlocks: 11, dataCodewordsPerBlock: 45 },
                ],
            },
            {
                ecCodewordsPerBlock: 26,
                ecBlocks: [
                    { numBlocks: 17, dataCodewordsPerBlock: 21 },
                    { numBlocks: 4, dataCodewordsPerBlock: 22 },
                ],
            },
            {
                ecCodewordsPerBlock: 26,
                ecBlocks: [
                    { numBlocks: 9, dataCodewordsPerBlock: 13 },
                    { numBlocks: 16, dataCodewordsPerBlock: 14 },
                ],
            },
        ],
    },
    {
        infoBits: 0x149A6,
        versionNumber: 20,
        alignmentPatternCenters: [6, 34, 62, 90],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 3, dataCodewordsPerBlock: 107 },
                    { numBlocks: 5, dataCodewordsPerBlock: 108 },
                ],
            },
            {
                ecCodewordsPerBlock: 26,
                ecBlocks: [
                    { numBlocks: 3, dataCodewordsPerBlock: 41 },
                    { numBlocks: 13, dataCodewordsPerBlock: 42 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 15, dataCodewordsPerBlock: 24 },
                    { numBlocks: 5, dataCodewordsPerBlock: 25 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 15, dataCodewordsPerBlock: 15 },
                    { numBlocks: 10, dataCodewordsPerBlock: 16 },
                ],
            },
        ],
    },
    {
        infoBits: 0x15683,
        versionNumber: 21,
        alignmentPatternCenters: [6, 28, 50, 72, 94],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 4, dataCodewordsPerBlock: 116 },
                    { numBlocks: 4, dataCodewordsPerBlock: 117 },
                ],
            },
            {
                ecCodewordsPerBlock: 26,
                ecBlocks: [{ numBlocks: 17, dataCodewordsPerBlock: 42 }],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 17, dataCodewordsPerBlock: 22 },
                    { numBlocks: 6, dataCodewordsPerBlock: 23 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 19, dataCodewordsPerBlock: 16 },
                    { numBlocks: 6, dataCodewordsPerBlock: 17 },
                ],
            },
        ],
    },
    {
        infoBits: 0x168C9,
        versionNumber: 22,
        alignmentPatternCenters: [6, 26, 50, 74, 98],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 2, dataCodewordsPerBlock: 111 },
                    { numBlocks: 7, dataCodewordsPerBlock: 112 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [{ numBlocks: 17, dataCodewordsPerBlock: 46 }],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 7, dataCodewordsPerBlock: 24 },
                    { numBlocks: 16, dataCodewordsPerBlock: 25 },
                ],
            },
            {
                ecCodewordsPerBlock: 24,
                ecBlocks: [{ numBlocks: 34, dataCodewordsPerBlock: 13 }],
            },
        ],
    },
    {
        infoBits: 0x177EC,
        versionNumber: 23,
        alignmentPatternCenters: [6, 30, 54, 74, 102],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 4, dataCodewordsPerBlock: 121 },
                    { numBlocks: 5, dataCodewordsPerBlock: 122 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 4, dataCodewordsPerBlock: 47 },
                    { numBlocks: 14, dataCodewordsPerBlock: 48 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 11, dataCodewordsPerBlock: 24 },
                    { numBlocks: 14, dataCodewordsPerBlock: 25 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 16, dataCodewordsPerBlock: 15 },
                    { numBlocks: 14, dataCodewordsPerBlock: 16 },
                ],
            },
        ],
    },
    {
        infoBits: 0x18EC4,
        versionNumber: 24,
        alignmentPatternCenters: [6, 28, 54, 80, 106],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 6, dataCodewordsPerBlock: 117 },
                    { numBlocks: 4, dataCodewordsPerBlock: 118 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 6, dataCodewordsPerBlock: 45 },
                    { numBlocks: 14, dataCodewordsPerBlock: 46 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 11, dataCodewordsPerBlock: 24 },
                    { numBlocks: 16, dataCodewordsPerBlock: 25 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 30, dataCodewordsPerBlock: 16 },
                    { numBlocks: 2, dataCodewordsPerBlock: 17 },
                ],
            },
        ],
    },
    {
        infoBits: 0x191E1,
        versionNumber: 25,
        alignmentPatternCenters: [6, 32, 58, 84, 110],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 26,
                ecBlocks: [
                    { numBlocks: 8, dataCodewordsPerBlock: 106 },
                    { numBlocks: 4, dataCodewordsPerBlock: 107 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 8, dataCodewordsPerBlock: 47 },
                    { numBlocks: 13, dataCodewordsPerBlock: 48 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 7, dataCodewordsPerBlock: 24 },
                    { numBlocks: 22, dataCodewordsPerBlock: 25 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 22, dataCodewordsPerBlock: 15 },
                    { numBlocks: 13, dataCodewordsPerBlock: 16 },
                ],
            },
        ],
    },
    {
        infoBits: 0x1AFAB,
        versionNumber: 26,
        alignmentPatternCenters: [6, 30, 58, 86, 114],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 10, dataCodewordsPerBlock: 114 },
                    { numBlocks: 2, dataCodewordsPerBlock: 115 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 19, dataCodewordsPerBlock: 46 },
                    { numBlocks: 4, dataCodewordsPerBlock: 47 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 28, dataCodewordsPerBlock: 22 },
                    { numBlocks: 6, dataCodewordsPerBlock: 23 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 33, dataCodewordsPerBlock: 16 },
                    { numBlocks: 4, dataCodewordsPerBlock: 17 },
                ],
            },
        ],
    },
    {
        infoBits: 0x1B08E,
        versionNumber: 27,
        alignmentPatternCenters: [6, 34, 62, 90, 118],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 8, dataCodewordsPerBlock: 122 },
                    { numBlocks: 4, dataCodewordsPerBlock: 123 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 22, dataCodewordsPerBlock: 45 },
                    { numBlocks: 3, dataCodewordsPerBlock: 46 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 8, dataCodewordsPerBlock: 23 },
                    { numBlocks: 26, dataCodewordsPerBlock: 24 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 12, dataCodewordsPerBlock: 15 },
                    { numBlocks: 28, dataCodewordsPerBlock: 16 },
                ],
            },
        ],
    },
    {
        infoBits: 0x1CC1A,
        versionNumber: 28,
        alignmentPatternCenters: [6, 26, 50, 74, 98, 122],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 3, dataCodewordsPerBlock: 117 },
                    { numBlocks: 10, dataCodewordsPerBlock: 118 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 3, dataCodewordsPerBlock: 45 },
                    { numBlocks: 23, dataCodewordsPerBlock: 46 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 4, dataCodewordsPerBlock: 24 },
                    { numBlocks: 31, dataCodewordsPerBlock: 25 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 11, dataCodewordsPerBlock: 15 },
                    { numBlocks: 31, dataCodewordsPerBlock: 16 },
                ],
            },
        ],
    },
    {
        infoBits: 0x1D33F,
        versionNumber: 29,
        alignmentPatternCenters: [6, 30, 54, 78, 102, 126],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 7, dataCodewordsPerBlock: 116 },
                    { numBlocks: 7, dataCodewordsPerBlock: 117 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 21, dataCodewordsPerBlock: 45 },
                    { numBlocks: 7, dataCodewordsPerBlock: 46 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 1, dataCodewordsPerBlock: 23 },
                    { numBlocks: 37, dataCodewordsPerBlock: 24 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 19, dataCodewordsPerBlock: 15 },
                    { numBlocks: 26, dataCodewordsPerBlock: 16 },
                ],
            },
        ],
    },
    {
        infoBits: 0x1ED75,
        versionNumber: 30,
        alignmentPatternCenters: [6, 26, 52, 78, 104, 130],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 5, dataCodewordsPerBlock: 115 },
                    { numBlocks: 10, dataCodewordsPerBlock: 116 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 19, dataCodewordsPerBlock: 47 },
                    { numBlocks: 10, dataCodewordsPerBlock: 48 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 15, dataCodewordsPerBlock: 24 },
                    { numBlocks: 25, dataCodewordsPerBlock: 25 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 23, dataCodewordsPerBlock: 15 },
                    { numBlocks: 25, dataCodewordsPerBlock: 16 },
                ],
            },
        ],
    },
    {
        infoBits: 0x1F250,
        versionNumber: 31,
        alignmentPatternCenters: [6, 30, 56, 82, 108, 134],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 13, dataCodewordsPerBlock: 115 },
                    { numBlocks: 3, dataCodewordsPerBlock: 116 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 2, dataCodewordsPerBlock: 46 },
                    { numBlocks: 29, dataCodewordsPerBlock: 47 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 42, dataCodewordsPerBlock: 24 },
                    { numBlocks: 1, dataCodewordsPerBlock: 25 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 23, dataCodewordsPerBlock: 15 },
                    { numBlocks: 28, dataCodewordsPerBlock: 16 },
                ],
            },
        ],
    },
    {
        infoBits: 0x209D5,
        versionNumber: 32,
        alignmentPatternCenters: [6, 34, 60, 86, 112, 138],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [{ numBlocks: 17, dataCodewordsPerBlock: 115 }],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 10, dataCodewordsPerBlock: 46 },
                    { numBlocks: 23, dataCodewordsPerBlock: 47 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 10, dataCodewordsPerBlock: 24 },
                    { numBlocks: 35, dataCodewordsPerBlock: 25 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 19, dataCodewordsPerBlock: 15 },
                    { numBlocks: 35, dataCodewordsPerBlock: 16 },
                ],
            },
        ],
    },
    {
        infoBits: 0x216F0,
        versionNumber: 33,
        alignmentPatternCenters: [6, 30, 58, 86, 114, 142],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 17, dataCodewordsPerBlock: 115 },
                    { numBlocks: 1, dataCodewordsPerBlock: 116 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 14, dataCodewordsPerBlock: 46 },
                    { numBlocks: 21, dataCodewordsPerBlock: 47 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 29, dataCodewordsPerBlock: 24 },
                    { numBlocks: 19, dataCodewordsPerBlock: 25 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 11, dataCodewordsPerBlock: 15 },
                    { numBlocks: 46, dataCodewordsPerBlock: 16 },
                ],
            },
        ],
    },
    {
        infoBits: 0x228BA,
        versionNumber: 34,
        alignmentPatternCenters: [6, 34, 62, 90, 118, 146],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 13, dataCodewordsPerBlock: 115 },
                    { numBlocks: 6, dataCodewordsPerBlock: 116 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 14, dataCodewordsPerBlock: 46 },
                    { numBlocks: 23, dataCodewordsPerBlock: 47 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 44, dataCodewordsPerBlock: 24 },
                    { numBlocks: 7, dataCodewordsPerBlock: 25 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 59, dataCodewordsPerBlock: 16 },
                    { numBlocks: 1, dataCodewordsPerBlock: 17 },
                ],
            },
        ],
    },
    {
        infoBits: 0x2379F,
        versionNumber: 35,
        alignmentPatternCenters: [6, 30, 54, 78, 102, 126, 150],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 12, dataCodewordsPerBlock: 121 },
                    { numBlocks: 7, dataCodewordsPerBlock: 122 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 12, dataCodewordsPerBlock: 47 },
                    { numBlocks: 26, dataCodewordsPerBlock: 48 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 39, dataCodewordsPerBlock: 24 },
                    { numBlocks: 14, dataCodewordsPerBlock: 25 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 22, dataCodewordsPerBlock: 15 },
                    { numBlocks: 41, dataCodewordsPerBlock: 16 },
                ],
            },
        ],
    },
    {
        infoBits: 0x24B0B,
        versionNumber: 36,
        alignmentPatternCenters: [6, 24, 50, 76, 102, 128, 154],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 6, dataCodewordsPerBlock: 121 },
                    { numBlocks: 14, dataCodewordsPerBlock: 122 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 6, dataCodewordsPerBlock: 47 },
                    { numBlocks: 34, dataCodewordsPerBlock: 48 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 46, dataCodewordsPerBlock: 24 },
                    { numBlocks: 10, dataCodewordsPerBlock: 25 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 2, dataCodewordsPerBlock: 15 },
                    { numBlocks: 64, dataCodewordsPerBlock: 16 },
                ],
            },
        ],
    },
    {
        infoBits: 0x2542E,
        versionNumber: 37,
        alignmentPatternCenters: [6, 28, 54, 80, 106, 132, 158],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 17, dataCodewordsPerBlock: 122 },
                    { numBlocks: 4, dataCodewordsPerBlock: 123 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 29, dataCodewordsPerBlock: 46 },
                    { numBlocks: 14, dataCodewordsPerBlock: 47 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 49, dataCodewordsPerBlock: 24 },
                    { numBlocks: 10, dataCodewordsPerBlock: 25 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 24, dataCodewordsPerBlock: 15 },
                    { numBlocks: 46, dataCodewordsPerBlock: 16 },
                ],
            },
        ],
    },
    {
        infoBits: 0x26A64,
        versionNumber: 38,
        alignmentPatternCenters: [6, 32, 58, 84, 110, 136, 162],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 4, dataCodewordsPerBlock: 122 },
                    { numBlocks: 18, dataCodewordsPerBlock: 123 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 13, dataCodewordsPerBlock: 46 },
                    { numBlocks: 32, dataCodewordsPerBlock: 47 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 48, dataCodewordsPerBlock: 24 },
                    { numBlocks: 14, dataCodewordsPerBlock: 25 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 42, dataCodewordsPerBlock: 15 },
                    { numBlocks: 32, dataCodewordsPerBlock: 16 },
                ],
            },
        ],
    },
    {
        infoBits: 0x27541,
        versionNumber: 39,
        alignmentPatternCenters: [6, 26, 54, 82, 110, 138, 166],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 20, dataCodewordsPerBlock: 117 },
                    { numBlocks: 4, dataCodewordsPerBlock: 118 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 40, dataCodewordsPerBlock: 47 },
                    { numBlocks: 7, dataCodewordsPerBlock: 48 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 43, dataCodewordsPerBlock: 24 },
                    { numBlocks: 22, dataCodewordsPerBlock: 25 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 10, dataCodewordsPerBlock: 15 },
                    { numBlocks: 67, dataCodewordsPerBlock: 16 },
                ],
            },
        ],
    },
    {
        infoBits: 0x28C69,
        versionNumber: 40,
        alignmentPatternCenters: [6, 30, 58, 86, 114, 142, 170],
        errorCorrectionLevels: [
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 19, dataCodewordsPerBlock: 118 },
                    { numBlocks: 6, dataCodewordsPerBlock: 119 },
                ],
            },
            {
                ecCodewordsPerBlock: 28,
                ecBlocks: [
                    { numBlocks: 18, dataCodewordsPerBlock: 47 },
                    { numBlocks: 31, dataCodewordsPerBlock: 48 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 34, dataCodewordsPerBlock: 24 },
                    { numBlocks: 34, dataCodewordsPerBlock: 25 },
                ],
            },
            {
                ecCodewordsPerBlock: 30,
                ecBlocks: [
                    { numBlocks: 20, dataCodewordsPerBlock: 15 },
                    { numBlocks: 61, dataCodewordsPerBlock: 16 },
                ],
            },
        ],
    },
];


/***/ }),
/* 11 */
/***/ (function(module, exports, __webpack_require__) {

"use strict";

Object.defineProperty(exports, "__esModule", { value: true });
var BitMatrix_1 = __webpack_require__(0);
function squareToQuadrilateral(p1, p2, p3, p4) {
    var dx3 = p1.x - p2.x + p3.x - p4.x;
    var dy3 = p1.y - p2.y + p3.y - p4.y;
    if (dx3 === 0 && dy3 === 0) { // Affine
        return {
            a11: p2.x - p1.x,
            a12: p2.y - p1.y,
            a13: 0,
            a21: p3.x - p2.x,
            a22: p3.y - p2.y,
            a23: 0,
            a31: p1.x,
            a32: p1.y,
            a33: 1,
        };
    }
    else {
        var dx1 = p2.x - p3.x;
        var dx2 = p4.x - p3.x;
        var dy1 = p2.y - p3.y;
        var dy2 = p4.y - p3.y;
        var denominator = dx1 * dy2 - dx2 * dy1;
        var a13 = (dx3 * dy2 - dx2 * dy3) / denominator;
        var a23 = (dx1 * dy3 - dx3 * dy1) / denominator;
        return {
            a11: p2.x - p1.x + a13 * p2.x,
            a12: p2.y - p1.y + a13 * p2.y,
            a13: a13,
            a21: p4.x - p1.x + a23 * p4.x,
            a22: p4.y - p1.y + a23 * p4.y,
            a23: a23,
            a31: p1.x,
            a32: p1.y,
            a33: 1,
        };
    }
}
function quadrilateralToSquare(p1, p2, p3, p4) {
    // Here, the adjoint serves as the inverse:
    var sToQ = squareToQuadrilateral(p1, p2, p3, p4);
    return {
        a11: sToQ.a22 * sToQ.a33 - sToQ.a23 * sToQ.a32,
        a12: sToQ.a13 * sToQ.a32 - sToQ.a12 * sToQ.a33,
        a13: sToQ.a12 * sToQ.a23 - sToQ.a13 * sToQ.a22,
        a21: sToQ.a23 * sToQ.a31 - sToQ.a21 * sToQ.a33,
        a22: sToQ.a11 * sToQ.a33 - sToQ.a13 * sToQ.a31,
        a23: sToQ.a13 * sToQ.a21 - sToQ.a11 * sToQ.a23,
        a31: sToQ.a21 * sToQ.a32 - sToQ.a22 * sToQ.a31,
        a32: sToQ.a12 * sToQ.a31 - sToQ.a11 * sToQ.a32,
        a33: sToQ.a11 * sToQ.a22 - sToQ.a12 * sToQ.a21,
    };
}
function times(a, b) {
    return {
        a11: a.a11 * b.a11 + a.a21 * b.a12 + a.a31 * b.a13,
        a12: a.a12 * b.a11 + a.a22 * b.a12 + a.a32 * b.a13,
        a13: a.a13 * b.a11 + a.a23 * b.a12 + a.a33 * b.a13,
        a21: a.a11 * b.a21 + a.a21 * b.a22 + a.a31 * b.a23,
        a22: a.a12 * b.a21 + a.a22 * b.a22 + a.a32 * b.a23,
        a23: a.a13 * b.a21 + a.a23 * b.a22 + a.a33 * b.a23,
        a31: a.a11 * b.a31 + a.a21 * b.a32 + a.a31 * b.a33,
        a32: a.a12 * b.a31 + a.a22 * b.a32 + a.a32 * b.a33,
        a33: a.a13 * b.a31 + a.a23 * b.a32 + a.a33 * b.a33,
    };
}
function extract(image, location) {
    var qToS = quadrilateralToSquare({ x: 3.5, y: 3.5 }, { x: location.dimension - 3.5, y: 3.5 }, { x: location.dimension - 6.5, y: location.dimension - 6.5 }, { x: 3.5, y: location.dimension - 3.5 });
    var sToQ = squareToQuadrilateral(location.topLeft, location.topRight, location.alignmentPattern, location.bottomLeft);
    var transform = times(sToQ, qToS);
    var matrix = BitMatrix_1.BitMatrix.createEmpty(location.dimension, location.dimension);
    var mappingFunction = function (x, y) {
        var denominator = transform.a13 * x + transform.a23 * y + transform.a33;
        return {
            x: (transform.a11 * x + transform.a21 * y + transform.a31) / denominator,
            y: (transform.a12 * x + transform.a22 * y + transform.a32) / denominator,
        };
    };
    for (var y = 0; y < location.dimension; y++) {
        for (var x = 0; x < location.dimension; x++) {
            var xValue = x + 0.5;
            var yValue = y + 0.5;
            var sourcePixel = mappingFunction(xValue, yValue);
            matrix.set(x, y, image.get(Math.floor(sourcePixel.x), Math.floor(sourcePixel.y)));
        }
    }
    return {
        matrix: matrix,
        mappingFunction: mappingFunction,
    };
}
exports.extract = extract;


/***/ }),
/* 12 */
/***/ (function(module, exports, __webpack_require__) {

"use strict";

Object.defineProperty(exports, "__esModule", { value: true });
var MAX_FINDERPATTERNS_TO_SEARCH = 4;
var MIN_QUAD_RATIO = 0.5;
var MAX_QUAD_RATIO = 1.5;
var distance = function (a, b) { return Math.sqrt(Math.pow((b.x - a.x), 2) + Math.pow((b.y - a.y), 2)); };
function sum(values) {
    return values.reduce(function (a, b) { return a + b; });
}
// Takes three finder patterns and organizes them into topLeft, topRight, etc
function reorderFinderPatterns(pattern1, pattern2, pattern3) {
    var _a, _b, _c, _d;
    // Find distances between pattern centers
    var oneTwoDistance = distance(pattern1, pattern2);
    var twoThreeDistance = distance(pattern2, pattern3);
    var oneThreeDistance = distance(pattern1, pattern3);
    var bottomLeft;
    var topLeft;
    var topRight;
    // Assume one closest to other two is B; A and C will just be guesses at first
    if (twoThreeDistance >= oneTwoDistance && twoThreeDistance >= oneThreeDistance) {
        _a = [pattern2, pattern1, pattern3], bottomLeft = _a[0], topLeft = _a[1], topRight = _a[2];
    }
    else if (oneThreeDistance >= twoThreeDistance && oneThreeDistance >= oneTwoDistance) {
        _b = [pattern1, pattern2, pattern3], bottomLeft = _b[0], topLeft = _b[1], topRight = _b[2];
    }
    else {
        _c = [pattern1, pattern3, pattern2], bottomLeft = _c[0], topLeft = _c[1], topRight = _c[2];
    }
    // Use cross product to figure out whether bottomLeft (A) and topRight (C) are correct or flipped in relation to topLeft (B)
    // This asks whether BC x BA has a positive z component, which is the arrangement we want. If it's negative, then
    // we've got it flipped around and should swap topRight and bottomLeft.
    if (((topRight.x - topLeft.x) * (bottomLeft.y - topLeft.y)) - ((topRight.y - topLeft.y) * (bottomLeft.x - topLeft.x)) < 0) {
        _d = [topRight, bottomLeft], bottomLeft = _d[0], topRight = _d[1];
    }
    return { bottomLeft: bottomLeft, topLeft: topLeft, topRight: topRight };
}
// Computes the dimension (number of modules on a side) of the QR Code based on the position of the finder patterns
function computeDimension(topLeft, topRight, bottomLeft, matrix) {
    var moduleSize = (sum(countBlackWhiteRun(topLeft, bottomLeft, matrix, 5)) / 7 + // Divide by 7 since the ratio is 1:1:3:1:1
        sum(countBlackWhiteRun(topLeft, topRight, matrix, 5)) / 7 +
        sum(countBlackWhiteRun(bottomLeft, topLeft, matrix, 5)) / 7 +
        sum(countBlackWhiteRun(topRight, topLeft, matrix, 5)) / 7) / 4;
    if (moduleSize < 1) {
        throw new Error("Invalid module size");
    }
    var topDimension = Math.round(distance(topLeft, topRight) / moduleSize);
    var sideDimension = Math.round(distance(topLeft, bottomLeft) / moduleSize);
    var dimension = Math.floor((topDimension + sideDimension) / 2) + 7;
    switch (dimension % 4) {
        case 0:
            dimension++;
            break;
        case 2:
            dimension--;
            break;
    }
    return { dimension: dimension, moduleSize: moduleSize };
}
// Takes an origin point and an end point and counts the sizes of the black white run from the origin towards the end point.
// Returns an array of elements, representing the pixel size of the black white run.
// Uses a variant of http://en.wikipedia.org/wiki/Bresenham's_line_algorithm
function countBlackWhiteRunTowardsPoint(origin, end, matrix, length) {
    var switchPoints = [{ x: Math.floor(origin.x), y: Math.floor(origin.y) }];
    var steep = Math.abs(end.y - origin.y) > Math.abs(end.x - origin.x);
    var fromX;
    var fromY;
    var toX;
    var toY;
    if (steep) {
        fromX = Math.floor(origin.y);
        fromY = Math.floor(origin.x);
        toX = Math.floor(end.y);
        toY = Math.floor(end.x);
    }
    else {
        fromX = Math.floor(origin.x);
        fromY = Math.floor(origin.y);
        toX = Math.floor(end.x);
        toY = Math.floor(end.y);
    }
    var dx = Math.abs(toX - fromX);
    var dy = Math.abs(toY - fromY);
    var error = Math.floor(-dx / 2);
    var xStep = fromX < toX ? 1 : -1;
    var yStep = fromY < toY ? 1 : -1;
    var currentPixel = true;
    // Loop up until x == toX, but not beyond
    for (var x = fromX, y = fromY; x !== toX + xStep; x += xStep) {
        // Does current pixel mean we have moved white to black or vice versa?
        // Scanning black in state 0,2 and white in state 1, so if we find the wrong
        // color, advance to next state or end if we are in state 2 already
        var realX = steep ? y : x;
        var realY = steep ? x : y;
        if (matrix.get(realX, realY) !== currentPixel) {
            currentPixel = !currentPixel;
            switchPoints.push({ x: realX, y: realY });
            if (switchPoints.length === length + 1) {
                break;
            }
        }
        error += dy;
        if (error > 0) {
            if (y === toY) {
                break;
            }
            y += yStep;
            error -= dx;
        }
    }
    var distances = [];
    for (var i = 0; i < length; i++) {
        if (switchPoints[i] && switchPoints[i + 1]) {
            distances.push(distance(switchPoints[i], switchPoints[i + 1]));
        }
        else {
            distances.push(0);
        }
    }
    return distances;
}
// Takes an origin point and an end point and counts the sizes of the black white run in the origin point
// along the line that intersects with the end point. Returns an array of elements, representing the pixel sizes
// of the black white run. Takes a length which represents the number of switches from black to white to look for.
function countBlackWhiteRun(origin, end, matrix, length) {
    var _a;
    var rise = end.y - origin.y;
    var run = end.x - origin.x;
    var towardsEnd = countBlackWhiteRunTowardsPoint(origin, end, matrix, Math.ceil(length / 2));
    var awayFromEnd = countBlackWhiteRunTowardsPoint(origin, { x: origin.x - run, y: origin.y - rise }, matrix, Math.ceil(length / 2));
    var middleValue = towardsEnd.shift() + awayFromEnd.shift() - 1; // Substract one so we don't double count a pixel
    return (_a = awayFromEnd.concat(middleValue)).concat.apply(_a, towardsEnd);
}
// Takes in a black white run and an array of expected ratios. Returns the average size of the run as well as the "error" -
// that is the amount the run diverges from the expected ratio
function scoreBlackWhiteRun(sequence, ratios) {
    var averageSize = sum(sequence) / sum(ratios);
    var error = 0;
    ratios.forEach(function (ratio, i) {
        error += Math.pow((sequence[i] - ratio * averageSize), 2);
    });
    return { averageSize: averageSize, error: error };
}
// Takes an X,Y point and an array of sizes and scores the point against those ratios.
// For example for a finder pattern takes the ratio list of 1:1:3:1:1 and checks horizontal, vertical and diagonal ratios
// against that.
function scorePattern(point, ratios, matrix) {
    try {
        var horizontalRun = countBlackWhiteRun(point, { x: -1, y: point.y }, matrix, ratios.length);
        var verticalRun = countBlackWhiteRun(point, { x: point.x, y: -1 }, matrix, ratios.length);
        var topLeftPoint = {
            x: Math.max(0, point.x - point.y) - 1,
            y: Math.max(0, point.y - point.x) - 1,
        };
        var topLeftBottomRightRun = countBlackWhiteRun(point, topLeftPoint, matrix, ratios.length);
        var bottomLeftPoint = {
            x: Math.min(matrix.width, point.x + point.y) + 1,
            y: Math.min(matrix.height, point.y + point.x) + 1,
        };
        var bottomLeftTopRightRun = countBlackWhiteRun(point, bottomLeftPoint, matrix, ratios.length);
        var horzError = scoreBlackWhiteRun(horizontalRun, ratios);
        var vertError = scoreBlackWhiteRun(verticalRun, ratios);
        var diagDownError = scoreBlackWhiteRun(topLeftBottomRightRun, ratios);
        var diagUpError = scoreBlackWhiteRun(bottomLeftTopRightRun, ratios);
        var ratioError = Math.sqrt(horzError.error * horzError.error +
            vertError.error * vertError.error +
            diagDownError.error * diagDownError.error +
            diagUpError.error * diagUpError.error);
        var avgSize = (horzError.averageSize + vertError.averageSize + diagDownError.averageSize + diagUpError.averageSize) / 4;
        var sizeError = (Math.pow((horzError.averageSize - avgSize), 2) +
            Math.pow((vertError.averageSize - avgSize), 2) +
            Math.pow((diagDownError.averageSize - avgSize), 2) +
            Math.pow((diagUpError.averageSize - avgSize), 2)) / avgSize;
        return ratioError + sizeError;
    }
    catch (_a) {
        return Infinity;
    }
}
function recenterLocation(matrix, p) {
    var leftX = Math.round(p.x);
    while (matrix.get(leftX, Math.round(p.y))) {
        leftX--;
    }
    var rightX = Math.round(p.x);
    while (matrix.get(rightX, Math.round(p.y))) {
        rightX++;
    }
    var x = (leftX + rightX) / 2;
    var topY = Math.round(p.y);
    while (matrix.get(Math.round(x), topY)) {
        topY--;
    }
    var bottomY = Math.round(p.y);
    while (matrix.get(Math.round(x), bottomY)) {
        bottomY++;
    }
    var y = (topY + bottomY) / 2;
    return { x: x, y: y };
}
function locate(matrix) {
    var finderPatternQuads = [];
    var activeFinderPatternQuads = [];
    var alignmentPatternQuads = [];
    var activeAlignmentPatternQuads = [];
    var _loop_1 = function (y) {
        var length_1 = 0;
        var lastBit = false;
        var scans = [0, 0, 0, 0, 0];
        var _loop_2 = function (x) {
            var v = matrix.get(x, y);
            if (v === lastBit) {
                length_1++;
            }
            else {
                scans = [scans[1], scans[2], scans[3], scans[4], length_1];
                length_1 = 1;
                lastBit = v;
                // Do the last 5 color changes ~ match the expected ratio for a finder pattern? 1:1:3:1:1 of b:w:b:w:b
                var averageFinderPatternBlocksize = sum(scans) / 7;
                var validFinderPattern = Math.abs(scans[0] - averageFinderPatternBlocksize) < averageFinderPatternBlocksize &&
                    Math.abs(scans[1] - averageFinderPatternBlocksize) < averageFinderPatternBlocksize &&
                    Math.abs(scans[2] - 3 * averageFinderPatternBlocksize) < 3 * averageFinderPatternBlocksize &&
                    Math.abs(scans[3] - averageFinderPatternBlocksize) < averageFinderPatternBlocksize &&
                    Math.abs(scans[4] - averageFinderPatternBlocksize) < averageFinderPatternBlocksize &&
                    !v; // And make sure the current pixel is white since finder patterns are bordered in white
                // Do the last 3 color changes ~ match the expected ratio for an alignment pattern? 1:1:1 of w:b:w
                var averageAlignmentPatternBlocksize = sum(scans.slice(-3)) / 3;
                var validAlignmentPattern = Math.abs(scans[2] - averageAlignmentPatternBlocksize) < averageAlignmentPatternBlocksize &&
                    Math.abs(scans[3] - averageAlignmentPatternBlocksize) < averageAlignmentPatternBlocksize &&
                    Math.abs(scans[4] - averageAlignmentPatternBlocksize) < averageAlignmentPatternBlocksize &&
                    v; // Is the current pixel black since alignment patterns are bordered in black
                if (validFinderPattern) {
                    // Compute the start and end x values of the large center black square
                    var endX_1 = x - scans[3] - scans[4];
                    var startX_1 = endX_1 - scans[2];
                    var line = { startX: startX_1, endX: endX_1, y: y };
                    // Is there a quad directly above the current spot? If so, extend it with the new line. Otherwise, create a new quad with
                    // that line as the starting point.
                    var matchingQuads = activeFinderPatternQuads.filter(function (q) {
                        return (startX_1 >= q.bottom.startX && startX_1 <= q.bottom.endX) ||
                            (endX_1 >= q.bottom.startX && startX_1 <= q.bottom.endX) ||
                            (startX_1 <= q.bottom.startX && endX_1 >= q.bottom.endX && ((scans[2] / (q.bottom.endX - q.bottom.startX)) < MAX_QUAD_RATIO &&
                                (scans[2] / (q.bottom.endX - q.bottom.startX)) > MIN_QUAD_RATIO));
                    });
                    if (matchingQuads.length > 0) {
                        matchingQuads[0].bottom = line;
                    }
                    else {
                        activeFinderPatternQuads.push({ top: line, bottom: line });
                    }
                }
                if (validAlignmentPattern) {
                    // Compute the start and end x values of the center black square
                    var endX_2 = x - scans[4];
                    var startX_2 = endX_2 - scans[3];
                    var line = { startX: startX_2, y: y, endX: endX_2 };
                    // Is there a quad directly above the current spot? If so, extend it with the new line. Otherwise, create a new quad with
                    // that line as the starting point.
                    var matchingQuads = activeAlignmentPatternQuads.filter(function (q) {
                        return (startX_2 >= q.bottom.startX && startX_2 <= q.bottom.endX) ||
                            (endX_2 >= q.bottom.startX && startX_2 <= q.bottom.endX) ||
                            (startX_2 <= q.bottom.startX && endX_2 >= q.bottom.endX && ((scans[2] / (q.bottom.endX - q.bottom.startX)) < MAX_QUAD_RATIO &&
                                (scans[2] / (q.bottom.endX - q.bottom.startX)) > MIN_QUAD_RATIO));
                    });
                    if (matchingQuads.length > 0) {
                        matchingQuads[0].bottom = line;
                    }
                    else {
                        activeAlignmentPatternQuads.push({ top: line, bottom: line });
                    }
                }
            }
        };
        for (var x = -1; x <= matrix.width; x++) {
            _loop_2(x);
        }
        finderPatternQuads.push.apply(finderPatternQuads, activeFinderPatternQuads.filter(function (q) { return q.bottom.y !== y && q.bottom.y - q.top.y >= 2; }));
        activeFinderPatternQuads = activeFinderPatternQuads.filter(function (q) { return q.bottom.y === y; });
        alignmentPatternQuads.push.apply(alignmentPatternQuads, activeAlignmentPatternQuads.filter(function (q) { return q.bottom.y !== y; }));
        activeAlignmentPatternQuads = activeAlignmentPatternQuads.filter(function (q) { return q.bottom.y === y; });
    };
    for (var y = 0; y <= matrix.height; y++) {
        _loop_1(y);
    }
    finderPatternQuads.push.apply(finderPatternQuads, activeFinderPatternQuads.filter(function (q) { return q.bottom.y - q.top.y >= 2; }));
    alignmentPatternQuads.push.apply(alignmentPatternQuads, activeAlignmentPatternQuads);
    var finderPatternGroups = finderPatternQuads
        .filter(function (q) { return q.bottom.y - q.top.y >= 2; }) // All quads must be at least 2px tall since the center square is larger than a block
        .map(function (q) {
        var x = (q.top.startX + q.top.endX + q.bottom.startX + q.bottom.endX) / 4;
        var y = (q.top.y + q.bottom.y + 1) / 2;
        if (!matrix.get(Math.round(x), Math.round(y))) {
            return;
        }
        var lengths = [q.top.endX - q.top.startX, q.bottom.endX - q.bottom.startX, q.bottom.y - q.top.y + 1];
        var size = sum(lengths) / lengths.length;
        var score = scorePattern({ x: Math.round(x), y: Math.round(y) }, [1, 1, 3, 1, 1], matrix);
        return { score: score, x: x, y: y, size: size };
    })
        .filter(function (q) { return !!q; }) // Filter out any rejected quads from above
        .sort(function (a, b) { return a.score - b.score; })
        // Now take the top finder pattern options and try to find 2 other options with a similar size.
        .map(function (point, i, finderPatterns) {
        if (i > MAX_FINDERPATTERNS_TO_SEARCH) {
            return null;
        }
        var otherPoints = finderPatterns
            .filter(function (p, ii) { return i !== ii; })
            .map(function (p) { return ({ x: p.x, y: p.y, score: p.score + (Math.pow((p.size - point.size), 2)) / point.size, size: p.size }); })
            .sort(function (a, b) { return a.score - b.score; });
        if (otherPoints.length < 2) {
            return null;
        }
        var score = point.score + otherPoints[0].score + otherPoints[1].score;
        return { points: [point].concat(otherPoints.slice(0, 2)), score: score };
    })
        .filter(function (q) { return !!q; }) // Filter out any rejected finder patterns from above
        .sort(function (a, b) { return a.score - b.score; });
    if (finderPatternGroups.length === 0) {
        return null;
    }
    var _a = reorderFinderPatterns(finderPatternGroups[0].points[0], finderPatternGroups[0].points[1], finderPatternGroups[0].points[2]), topRight = _a.topRight, topLeft = _a.topLeft, bottomLeft = _a.bottomLeft;
    var alignment = findAlignmentPattern(matrix, alignmentPatternQuads, topRight, topLeft, bottomLeft);
    var result = [];
    if (alignment) {
        result.push({
            alignmentPattern: { x: alignment.alignmentPattern.x, y: alignment.alignmentPattern.y },
            bottomLeft: { x: bottomLeft.x, y: bottomLeft.y },
            dimension: alignment.dimension,
            topLeft: { x: topLeft.x, y: topLeft.y },
            topRight: { x: topRight.x, y: topRight.y },
        });
    }
    // We normally use the center of the quads as the location of the tracking points, which is optimal for most cases and will account
    // for a skew in the image. However, In some cases, a slight skew might not be real and instead be caused by image compression
    // errors and/or low resolution. For those cases, we'd be better off centering the point exactly in the middle of the black area. We
    // compute and return the location data for the naively centered points as it is little additional work and allows for multiple
    // attempts at decoding harder images.
    var midTopRight = recenterLocation(matrix, topRight);
    var midTopLeft = recenterLocation(matrix, topLeft);
    var midBottomLeft = recenterLocation(matrix, bottomLeft);
    var centeredAlignment = findAlignmentPattern(matrix, alignmentPatternQuads, midTopRight, midTopLeft, midBottomLeft);
    if (centeredAlignment) {
        result.push({
            alignmentPattern: { x: centeredAlignment.alignmentPattern.x, y: centeredAlignment.alignmentPattern.y },
            bottomLeft: { x: midBottomLeft.x, y: midBottomLeft.y },
            topLeft: { x: midTopLeft.x, y: midTopLeft.y },
            topRight: { x: midTopRight.x, y: midTopRight.y },
            dimension: centeredAlignment.dimension,
        });
    }
    if (result.length === 0) {
        return null;
    }
    return result;
}
exports.locate = locate;
function findAlignmentPattern(matrix, alignmentPatternQuads, topRight, topLeft, bottomLeft) {
    var _a;
    // Now that we've found the three finder patterns we can determine the blockSize and the size of the QR code.
    // We'll use these to help find the alignment pattern but also later when we do the extraction.
    var dimension;
    var moduleSize;
    try {
        (_a = computeDimension(topLeft, topRight, bottomLeft, matrix), dimension = _a.dimension, moduleSize = _a.moduleSize);
    }
    catch (e) {
        return null;
    }
    // Now find the alignment pattern
    var bottomRightFinderPattern = {
        x: topRight.x - topLeft.x + bottomLeft.x,
        y: topRight.y - topLeft.y + bottomLeft.y,
    };
    var modulesBetweenFinderPatterns = ((distance(topLeft, bottomLeft) + distance(topLeft, topRight)) / 2 / moduleSize);
    var correctionToTopLeft = 1 - (3 / modulesBetweenFinderPatterns);
    var expectedAlignmentPattern = {
        x: topLeft.x + correctionToTopLeft * (bottomRightFinderPattern.x - topLeft.x),
        y: topLeft.y + correctionToTopLeft * (bottomRightFinderPattern.y - topLeft.y),
    };
    var alignmentPatterns = alignmentPatternQuads
        .map(function (q) {
        var x = (q.top.startX + q.top.endX + q.bottom.startX + q.bottom.endX) / 4;
        var y = (q.top.y + q.bottom.y + 1) / 2;
        if (!matrix.get(Math.floor(x), Math.floor(y))) {
            return;
        }
        var lengths = [q.top.endX - q.top.startX, q.bottom.endX - q.bottom.startX, (q.bottom.y - q.top.y + 1)];
        var size = sum(lengths) / lengths.length;
        var sizeScore = scorePattern({ x: Math.floor(x), y: Math.floor(y) }, [1, 1, 1], matrix);
        var score = sizeScore + distance({ x: x, y: y }, expectedAlignmentPattern);
        return { x: x, y: y, score: score };
    })
        .filter(function (v) { return !!v; })
        .sort(function (a, b) { return a.score - b.score; });
    // If there are less than 15 modules between finder patterns it's a version 1 QR code and as such has no alignmemnt pattern
    // so we can only use our best guess.
    var alignmentPattern = modulesBetweenFinderPatterns >= 15 && alignmentPatterns.length ? alignmentPatterns[0] : expectedAlignmentPattern;
    return { alignmentPattern: alignmentPattern, dimension: dimension };
}


/***/ })
/******/ ])["default"];
});

export const {jsQR} = root;
"###;

const LIT_ELEMENT_LIT_ELEMENT_673 : &'static str = r###"

/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */var l,o;const r=t;class s extends t{constructor(){super(...arguments),this.renderOptions={host:this},this._$Do=void 0}createRenderRoot(){var t,e;const i=super.createRenderRoot();return null!==(t=(e=this.renderOptions).renderBefore)&&void 0!==t||(e.renderBefore=i.firstChild),i}update(t){const i=this.render();this.hasUpdated||(this.renderOptions.isConnected=this.isConnected),super.update(t),this._$Do=e(i,this.renderRoot,this.renderOptions)}connectedCallback(){var t;super.connectedCallback(),null===(t=this._$Do)||void 0===t||t.setConnected(!0)}disconnectedCallback(){var t;super.disconnectedCallback(),null===(t=this._$Do)||void 0===t||t.setConnected(!1)}render(){return i}}s.finalized=!0,s._$litElement$=!0,null===(l=globalThis.litElementHydrateSupport)||void 0===l||l.call(globalThis,{LitElement:s});const n=globalThis.litElementPolyfillSupport;null==n||n({LitElement:s});const h={_$AK:(t,e,i)=>{t._$AK(e,i)},_$AL:t=>t._$AL};(null!==(o=globalThis.litElementVersions)&&void 0!==o?o:globalThis.litElementVersions=[]).push("3.2.2");export{s as LitElement,r as UpdatingElement,h as _$LE};
//# sourceMappingURL=lit-element.js.map

"###;

const LIT_HTML_ASYNC_DIRECTIVE_550 : &'static str = r###"

/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */const s=(i,t)=>{var e,o;const r=i._$AN;if(void 0===r)return!1;for(const i of r)null===(o=(e=i)._$AO)||void 0===o||o.call(e,t,!1),s(i,t);return!0},o=i=>{let t,e;do{if(void 0===(t=i._$AM))break;e=t._$AN,e.delete(i),i=t}while(0===(null==e?void 0:e.size))},r=i=>{for(let t;t=i._$AM;i=t){let e=t._$AN;if(void 0===e)t._$AN=e=new Set;else if(e.has(i))break;e.add(i),l(t)}};function n(i){void 0!==this._$AN?(o(this),this._$AM=i,r(this)):this._$AM=i}function h(i,t=!1,e=0){const r=this._$AH,n=this._$AN;if(void 0!==n&&0!==n.size)if(t)if(Array.isArray(r))for(let i=e;i<r.length;i++)s(r[i],!1),o(r[i]);else null!=r&&(s(r,!1),o(r));else s(this,i)}const l=i=>{var t,s,o,r;i.type==e.CHILD&&(null!==(t=(o=i)._$AP)&&void 0!==t||(o._$AP=h),null!==(s=(r=i)._$AQ)&&void 0!==s||(r._$AQ=n))};class c extends t{constructor(){super(...arguments),this._$AN=void 0}_$AT(i,t,e){super._$AT(i,t,e),r(this),this.isConnected=i._$AU}_$AO(i,t=!0){var e,r;i!==this.isConnected&&(this.isConnected=i,i?null===(e=this.reconnected)||void 0===e||e.call(this):null===(r=this.disconnected)||void 0===r||r.call(this)),t&&(s(this,i),o(this))}setValue(t){if(i(this._$Ct))this._$Ct._$AI(t,this);else{const i=[...this._$Ct._$AH];i[this._$Ci]=t,this._$Ct._$AI(i,this,0)}}disconnected(){}reconnected(){}}export{c as AsyncDirective};
//# sourceMappingURL=async-directive.js.map

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_HTML_389 : &'static str = r###"


export const flowHtml = (strings, tags, ...values)=>{
	strings = strings.slice(1).map(str=>{
		tags.forEach((value, key)=>{
			str = str.replace(new RegExp(`\{${key}\}`, 'g'), value);
		});
		return str;
	})
	//console.log("flowHtml:strings", strings)
	//console.log("flowHtml:values", values)
	return html(strings, ...values);
}

"###;

const ASPECTRON_FLOW_UX_SRC_PAGINATION_398 : &'static str = r###"




export const paginationStyle = css`
	.pagination-box{text-align:center;padding:10px 5px;}
	.pagination{display: inline-block;}
    .pagination-box[disabled]{display:none}
	.pagination a{
		color: var(--k-pagination-color);
		float: left;
		padding: 8px 16px;
		text-decoration: none;
		transition: background-color .3s;
		border: 1px solid #555;
		border-color:var(--k-pagination-border-color, var(--k-btn-border-color, #555));
		margin:var(--flow-pagination-margin, 2px 4px);
        cursor:pointer;
        user-select:none;
	}
	@media (max-width:768px){
		.pagination a{
			padding:8px 6px;
			margin: 0px 2px;
			font-size:0.8rem;
		}
	}
	.pagination a.disabled{
		opacity:0.5;
	}
	.pagination a.hidden{display:none}
	.pagination a.active{
		background:var(--k-pagination-active-bg, #2489da);
		color:var(--k-pagination-active-color, #FFF);
		border:1px solid #2489da;
		border-color:var(--k-pagination-active-border-color, #2489da);
	}
	.pagination a.active,
	.pagination a.disabled{
		cursor:default;
	}
	.pagination a:hover:not(.active):not(.disabled){
        border-color:var(--k-pagination-hover-border-color, var(--k-btn-hover-border-color, #777 ));
		background-color:var(--k-pagination-hover-bg-color, var(--k-btn-hover-bg-color, rgba(255,255,255, 0.2) ));
		color:var(--k-pagination-hover-color, var(--k-btn-hover-color, inherit));
	}
`;


export const buildPagination = (total, skip=0, limit=25)=>{
    skip = +skip;
    limit = +limit;
    total = +total;

    let totalPages = Math.ceil(total/limit);
    let activePage = Math.min(Math.ceil((skip+1)/limit), totalPages);
    let maxPages = Math.min(isSmallScreen?3:10, totalPages);
    let half = Math.floor(maxPages/2);
    let prev = Math.max(activePage-1, 1);
    let next = Math.min(activePage+1, totalPages)
    let p = 1;
    if(activePage-half > 1)
        p = activePage-maxPages+Math.min(totalPages-activePage, half)+1;

    let pages = [];
    for(let i=0; i<maxPages; i++){
        pages.push({
            p,
            skip:(p-1)*limit,
            active:activePage==p,
        })
        p++;
    }
    return {
        totalPages,
        activePage,
        isLast:activePage==totalPages,
        isFirst:activePage==1,
        prev,
        next,
        last:totalPages,
        lastSkip:(totalPages-1)*limit,
        prevSkip:(prev-1) * limit,
        nextSkip:(next-1) * limit,
        total,
        skip,
        limit,
        pages,
        maxPages,
        half
    }
}

export const renderPagination = (pagination, clickHandler, options={})=>{
    if(!pagination)
        pagination = {pages:[], isFirst:true, isLast:true, totalPages:0, type:''};
    let {pages, isFirst, isLast, prevSkip, nextSkip, totalPages, lastSkip, type} = pagination;

    let hideNextPrev = pages.length ==0?' hidden':'';
    let FIRST = options.first||'FIRST';
    let LAST = options.last||'LAST';
    let PREV = options.prev||'<';
    let NEXT = options.next||'>';
    clickHandler = clickHandler || (e=>{});
    return html`
    <div class="pagination-box" ?disabled="${!pages.length}" @click=${clickHandler}>
        <div class="pagination" data-pagination="${type}">
            <a class="first ${(isFirst?'disabled':'')+hideNextPrev}" data-skip="0">${FIRST}</a>
            <a class="prev ${(isFirst?'disabled':'')+hideNextPrev}" data-skip="${prevSkip}">${PREV}</a>
            ${repeat(pages, p=>p.p, p=>html`
                <a class="${p.active?'active':''}" data-skip="${p.skip}">${p.p}</a>
            `)}
            <a class="next ${(isLast?'disabled':'')+hideNextPrev}"  data-skip="${nextSkip}">${NEXT}</a>
            <a class="first ${(isLast?'disabled':'')+hideNextPrev}" data-skip="${lastSkip}">${LAST}</a>
        </div>
    </div>`;
}

export const loadingMaskStyle = css`
    .mask{
        position:absolute;
        left:0px;
        top:0px;
        right:0px;
        bottom:0px;
        width:100%;
        height:100%;
        opacity:1;
        z-index:1000;
        background-color:rgba(0, 0, 0, 0.7);
        background-color:var(--flow-loading-mask-bg-color, rgba(0, 0, 0, 0.7));
        animation: fade-out 1s ease forwards;
    }
    .mask .loading-logo{
        width: 100px;
        height: 100px;
        position: relative;
        left: 50%;
        top: 50%;
        margin: -50px 0 0 -50px;
    }

    :host(.loading) .mask,
    .loading .mask{
        animation-name: fade-in;
    }
    :host(:not(.loading)) .mask .loading-logo .animate{
        fill:#009688;
    }
    @keyframes fade-in{
        0% {z-index:-1; opacity:0}
        1% {z-index:1000; opacity:0}
        100% {z-index:1000; opacity:1}
    }
    @keyframes fade-out{
        0% {z-index:1000; opacity:1}
        99% {z-index:1000; opacity:0}
        100% {z-index:-1; opacity:0}
    }
`;
"###;

const LIT_REACTIVE_ELEMENT_REACTIVE_ELEMENT_89 : &'static str = r###"

/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */var s;const e=window,r=e.trustedTypes,h=r?r.emptyScript:"",o=e.reactiveElementPolyfillSupport,n={toAttribute(t,i){switch(i){case Boolean:t=t?h:null;break;case Object:case Array:t=null==t?t:JSON.stringify(t)}return t},fromAttribute(t,i){let s=t;switch(i){case Boolean:s=null!==t;break;case Number:s=null===t?null:Number(t);break;case Object:case Array:try{s=JSON.parse(t)}catch(t){s=null}}return s}},a=(t,i)=>i!==t&&(i==i||t==t),l={attribute:!0,type:String,converter:n,reflect:!1,hasChanged:a};class d extends HTMLElement{constructor(){super(),this._$Ei=new Map,this.isUpdatePending=!1,this.hasUpdated=!1,this._$El=null,this.u()}static addInitializer(t){var i;this.finalize(),(null!==(i=this.h)&&void 0!==i?i:this.h=[]).push(t)}static get observedAttributes(){this.finalize();const t=[];return this.elementProperties.forEach(((i,s)=>{const e=this._$Ep(s,i);void 0!==e&&(this._$Ev.set(e,s),t.push(e))})),t}static createProperty(t,i=l){if(i.state&&(i.attribute=!1),this.finalize(),this.elementProperties.set(t,i),!i.noAccessor&&!this.prototype.hasOwnProperty(t)){const s="symbol"==typeof t?Symbol():"__"+t,e=this.getPropertyDescriptor(t,s,i);void 0!==e&&Object.defineProperty(this.prototype,t,e)}}static getPropertyDescriptor(t,i,s){return{get(){return this[i]},set(e){const r=this[t];this[i]=e,this.requestUpdate(t,r,s)},configurable:!0,enumerable:!0}}static getPropertyOptions(t){return this.elementProperties.get(t)||l}static finalize(){if(this.hasOwnProperty("finalized"))return!1;this.finalized=!0;const t=Object.getPrototypeOf(this);if(t.finalize(),void 0!==t.h&&(this.h=[...t.h]),this.elementProperties=new Map(t.elementProperties),this._$Ev=new Map,this.hasOwnProperty("properties")){const t=this.properties,i=[...Object.getOwnPropertyNames(t),...Object.getOwnPropertySymbols(t)];for(const s of i)this.createProperty(s,t[s])}return this.elementStyles=this.finalizeStyles(this.styles),!0}static finalizeStyles(i){const s=[];if(Array.isArray(i)){const e=new Set(i.flat(1/0).reverse());for(const i of e)s.unshift(t(i))}else void 0!==i&&s.push(t(i));return s}static _$Ep(t,i){const s=i.attribute;return!1===s?void 0:"string"==typeof s?s:"string"==typeof t?t.toLowerCase():void 0}u(){var t;this._$E_=new Promise((t=>this.enableUpdating=t)),this._$AL=new Map,this._$Eg(),this.requestUpdate(),null===(t=this.constructor.h)||void 0===t||t.forEach((t=>t(this)))}addController(t){var i,s;(null!==(i=this._$ES)&&void 0!==i?i:this._$ES=[]).push(t),void 0!==this.renderRoot&&this.isConnected&&(null===(s=t.hostConnected)||void 0===s||s.call(t))}removeController(t){var i;null===(i=this._$ES)||void 0===i||i.splice(this._$ES.indexOf(t)>>>0,1)}_$Eg(){this.constructor.elementProperties.forEach(((t,i)=>{this.hasOwnProperty(i)&&(this._$Ei.set(i,this[i]),delete this[i])}))}createRenderRoot(){var t;const s=null!==(t=this.shadowRoot)&&void 0!==t?t:this.attachShadow(this.constructor.shadowRootOptions);return i(s,this.constructor.elementStyles),s}connectedCallback(){var t;void 0===this.renderRoot&&(this.renderRoot=this.createRenderRoot()),this.enableUpdating(!0),null===(t=this._$ES)||void 0===t||t.forEach((t=>{var i;return null===(i=t.hostConnected)||void 0===i?void 0:i.call(t)}))}enableUpdating(t){}disconnectedCallback(){var t;null===(t=this._$ES)||void 0===t||t.forEach((t=>{var i;return null===(i=t.hostDisconnected)||void 0===i?void 0:i.call(t)}))}attributeChangedCallback(t,i,s){this._$AK(t,s)}_$EO(t,i,s=l){var e;const r=this.constructor._$Ep(t,s);if(void 0!==r&&!0===s.reflect){const h=(void 0!==(null===(e=s.converter)||void 0===e?void 0:e.toAttribute)?s.converter:n).toAttribute(i,s.type);this._$El=t,null==h?this.removeAttribute(r):this.setAttribute(r,h),this._$El=null}}_$AK(t,i){var s;const e=this.constructor,r=e._$Ev.get(t);if(void 0!==r&&this._$El!==r){const t=e.getPropertyOptions(r),h="function"==typeof t.converter?{fromAttribute:t.converter}:void 0!==(null===(s=t.converter)||void 0===s?void 0:s.fromAttribute)?t.converter:n;this._$El=r,this[r]=h.fromAttribute(i,t.type),this._$El=null}}requestUpdate(t,i,s){let e=!0;void 0!==t&&(((s=s||this.constructor.getPropertyOptions(t)).hasChanged||a)(this[t],i)?(this._$AL.has(t)||this._$AL.set(t,i),!0===s.reflect&&this._$El!==t&&(void 0===this._$EC&&(this._$EC=new Map),this._$EC.set(t,s))):e=!1),!this.isUpdatePending&&e&&(this._$E_=this._$Ej())}async _$Ej(){this.isUpdatePending=!0;try{await this._$E_}catch(t){Promise.reject(t)}const t=this.scheduleUpdate();return null!=t&&await t,!this.isUpdatePending}scheduleUpdate(){return this.performUpdate()}performUpdate(){var t;if(!this.isUpdatePending)return;this.hasUpdated,this._$Ei&&(this._$Ei.forEach(((t,i)=>this[i]=t)),this._$Ei=void 0);let i=!1;const s=this._$AL;try{i=this.shouldUpdate(s),i?(this.willUpdate(s),null===(t=this._$ES)||void 0===t||t.forEach((t=>{var i;return null===(i=t.hostUpdate)||void 0===i?void 0:i.call(t)})),this.update(s)):this._$Ek()}catch(t){throw i=!1,this._$Ek(),t}i&&this._$AE(s)}willUpdate(t){}_$AE(t){var i;null===(i=this._$ES)||void 0===i||i.forEach((t=>{var i;return null===(i=t.hostUpdated)||void 0===i?void 0:i.call(t)})),this.hasUpdated||(this.hasUpdated=!0,this.firstUpdated(t)),this.updated(t)}_$Ek(){this._$AL=new Map,this.isUpdatePending=!1}get updateComplete(){return this.getUpdateComplete()}getUpdateComplete(){return this._$E_}shouldUpdate(t){return!0}update(t){void 0!==this._$EC&&(this._$EC.forEach(((t,i)=>this._$EO(i,this[i],t))),this._$EC=void 0),this._$Ek()}updated(t){}firstUpdated(t){}}d.finalized=!0,d.elementProperties=new Map,d.elementStyles=[],d.shadowRootOptions={mode:"open"},null==o||o({ReactiveElement:d}),(null!==(s=e.reactiveElementVersions)&&void 0!==s?s:e.reactiveElementVersions=[]).push("1.5.0");export{d as ReactiveElement,n as defaultConverter,a as notEqual};
//# sourceMappingURL=reactive-element.js.map

"###;

const LIT_REACTIVE_ELEMENT_CSS_TAG_31 : &'static str = r###"
/**
 * @license
 * Copyright 2019 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */
const t=window,e=t.ShadowRoot&&(void 0===t.ShadyCSS||t.ShadyCSS.nativeShadow)&&"adoptedStyleSheets"in Document.prototype&&"replace"in CSSStyleSheet.prototype,s=Symbol(),n=new WeakMap;class o{constructor(t,e,n){if(this._$cssResult$=!0,n!==s)throw Error("CSSResult is not constructable. Use `unsafeCSS` or `css` instead.");this.cssText=t,this.t=e}get styleSheet(){let t=this.o;const s=this.t;if(e&&void 0===t){const e=void 0!==s&&1===s.length;e&&(t=n.get(s)),void 0===t&&((this.o=t=new CSSStyleSheet).replaceSync(this.cssText),e&&n.set(s,t))}return t}toString(){return this.cssText}}const r=t=>new o("string"==typeof t?t:t+"",void 0,s),i=(t,...e)=>{const n=1===t.length?t[0]:e.reduce(((e,s,n)=>e+(t=>{if(!0===t._$cssResult$)return t.cssText;if("number"==typeof t)return t;throw Error("Value passed to 'css' function must be a 'css' function result: "+t+". Use 'unsafeCSS' to pass non-literal values, but take care to ensure page security.")})(s)+t[n+1]),t[0]);return new o(n,t,s)},S=(s,n)=>{e?s.adoptedStyleSheets=n.map((t=>t instanceof CSSStyleSheet?t:t.styleSheet)):n.forEach((e=>{const n=document.createElement("style"),o=t.litNonce;void 0!==o&&n.setAttribute("nonce",o),n.textContent=e.cssText,s.appendChild(n)}))},c=e?t=>t:t=>t instanceof CSSStyleSheet?(t=>{let e="";for(const s of t.cssRules)e+=s.cssText;return r(e)})(t):t;export{o as CSSResult,S as adoptStyles,i as css,c as getCompatibleStyle,e as supportsAdoptingStyleSheets,r as unsafeCSS};
//# sourceMappingURL=css-tag.js.map

"###;

const LIT_ELEMENT_INDEX_674 : &'static str = r###"

/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */console.warn("The main 'lit-element' module entrypoint is deprecated. Please update your imports to use the 'lit' package: 'lit' and 'lit/decorators.ts' or import from 'lit-element/lit-element.ts'. See https://lit.dev/msg/deprecated-import-path for more information.");
//# sourceMappingURL=index.js.map

"###;

const LIT_REACTIVE_ELEMENT_DECORATORS_BASE_38 : &'static str = r###"
/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */
const e=(e,t,o)=>{Object.defineProperty(t,o,e)},t=(e,t)=>({kind:"method",placement:"prototype",key:t.key,descriptor:e}),o=({finisher:e,descriptor:t})=>(o,n)=>{var r;if(void 0===n){const n=null!==(r=o.originalKey)&&void 0!==r?r:o.key,i=null!=t?{kind:"method",placement:"prototype",key:n,descriptor:t(o.key)}:{...o,key:n};return null!=e&&(i.finisher=function(t){e(t,n)}),i}{const r=o.constructor;void 0!==t&&Object.defineProperty(o,n,t(n)),null==e||e(r,n)}};export{o as decorateProperty,e as legacyPrototypeMethod,t as standardPrototypeMethod};
//# sourceMappingURL=base.js.map

"###;

const LIT_REACTIVE_ELEMENT_DECORATORS_CUSTOM_ELEMENT_40 : &'static str = r###"
/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */
const e=e=>n=>"function"==typeof n?((e,n)=>(customElements.define(e,n),n))(e,n):((e,n)=>{const{kind:t,elements:s}=n;return{kind:t,elements:s,finisher(n){customElements.define(e,n)}}})(e,n);export{e as customElement};
//# sourceMappingURL=custom-element.js.map

"###;

const LIT_REACTIVE_ELEMENT_DECORATORS_PROPERTY_33 : &'static str = r###"
/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */
const i=(i,e)=>"method"===e.kind&&e.descriptor&&!("value"in e.descriptor)?{...e,finisher(n){n.createProperty(e.key,i)}}:{kind:"field",key:Symbol(),placement:"own",descriptor:{},originalKey:e.key,initializer(){"function"==typeof e.initializer&&(this[e.key]=e.initializer.call(this))},finisher(n){n.createProperty(e.key,i)}};function e(e){return(n,t)=>void 0!==t?((i,e,n)=>{e.constructor.createProperty(n,i)})(e,n,t):i(e,n)}export{e as property};
//# sourceMappingURL=property.js.map

"###;

const LIT_REACTIVE_ELEMENT_DECORATORS_STATE_37 : &'static str = r###"

/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */function t(t){return r({...t,state:!0})}export{t as state};
//# sourceMappingURL=state.js.map

"###;

const LIT_REACTIVE_ELEMENT_DECORATORS_EVENT_OPTIONS_32 : &'static str = r###"

/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */function e(e){return r({finisher:(r,t)=>{Object.assign(r.prototype[t],e)}})}export{e as eventOptions};
//# sourceMappingURL=event-options.js.map

"###;

const LIT_REACTIVE_ELEMENT_DECORATORS_QUERY_34 : &'static str = r###"

/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */function i(i,n){return o({descriptor:o=>{const t={get(){var o,n;return null!==(n=null===(o=this.renderRoot)||void 0===o?void 0:o.querySelector(i))&&void 0!==n?n:null},enumerable:!0,configurable:!0};if(n){const n="symbol"==typeof o?Symbol():"__"+o;t.get=function(){var o,t;return void 0===this[n]&&(this[n]=null!==(t=null===(o=this.renderRoot)||void 0===o?void 0:o.querySelector(i))&&void 0!==t?t:null),this[n]}}return t}})}export{i as query};
//# sourceMappingURL=query.js.map

"###;

const LIT_REACTIVE_ELEMENT_DECORATORS_QUERY_ALL_36 : &'static str = r###"

/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */function e(e){return r({descriptor:r=>({get(){var r,o;return null!==(o=null===(r=this.renderRoot)||void 0===r?void 0:r.querySelectorAll(e))&&void 0!==o?o:[]},enumerable:!0,configurable:!0})})}export{e as queryAll};
//# sourceMappingURL=query-all.js.map

"###;

const LIT_REACTIVE_ELEMENT_DECORATORS_QUERY_ASYNC_35 : &'static str = r###"

/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */
function e(e){return r({descriptor:r=>({async get(){var r;return await this.updateComplete,null===(r=this.renderRoot)||void 0===r?void 0:r.querySelector(e)},enumerable:!0,configurable:!0})})}export{e as queryAsync};
//# sourceMappingURL=query-async.js.map

"###;

const LIT_REACTIVE_ELEMENT_DECORATORS_QUERY_ASSIGNED_ELEMENTS_39 : &'static str = r###"

/**
 * @license
 * Copyright 2021 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */var n;const e=null!=(null===(n=window.HTMLSlotElement)||void 0===n?void 0:n.prototype.assignedElements)?(o,n)=>o.assignedElements(n):(o,n)=>o.assignedNodes(n).filter((o=>o.nodeType===Node.ELEMENT_NODE));function l(n){const{slot:l,selector:t}=null!=n?n:{};return o({descriptor:o=>({get(){var o;const r="slot"+(l?`[name=${l}]`:":not([name])"),i=null===(o=this.renderRoot)||void 0===o?void 0:o.querySelector(r),s=null!=i?e(i,n):[];return t?s.filter((o=>o.matches(t))):s},enumerable:!0,configurable:!0})})}export{l as queryAssignedElements};
//# sourceMappingURL=query-assigned-elements.js.map

"###;

const LIT_REACTIVE_ELEMENT_DECORATORS_QUERY_ASSIGNED_NODES_41 : &'static str = r###"

/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */function o(o,n,r){let l,s=o;return"object"==typeof o?(s=o.slot,l=o):l={flatten:n},r?t({slot:s,flatten:n,selector:r}):e({descriptor:e=>({get(){var e,t;const o="slot"+(s?`[name=${s}]`:":not([name])"),n=null===(e=this.renderRoot)||void 0===e?void 0:e.querySelector(o);return null!==(t=null==n?void 0:n.assignedNodes(l))&&void 0!==t?t:[]},enumerable:!0,configurable:!0})})}export{o as queryAssignedNodes};
//# sourceMappingURL=query-assigned-nodes.js.map

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_EVENTS_367 : &'static str = r###"


export class FlowEvents{

	constructor(){

		this.events = new Map();
		this.refs = new Map();
		this.listeners = [];
		this.mevents = [];

		this.on('destroy', ()=>{
			this.mevents.forEach(uid=>{
			   this.off(uid);
			});
		})
	}

	on(op, fn){
		if(!fn)
			throw new Error("events::on() - callback is required");
		let uid = UID();
		if(!this.events.has(op))
			this.events.set(op, new Map());
		this.events.get(op).set(uid, fn)
		this.refs.set(uid, op);
		return uid;
	}

	mon(op, fn){
		let uid = this.on(op, fn);
		this.mevents.push(uid);
		return uid;
	}

	off(uid, op) {
		if (uid) {
			let op = this.refs.get(uid);
			this.refs.delete(uid);
			let list = this.events.get(op)
			if(list)
				list.delete(uid);
		}else if(op){
			(this.events.get(op)||[]).forEach((fn, uid)=>{
				this.refs.delete(uid);
			});

			this.events.delete(op);
		};
	}

	emit(op, args) {

		let list = this.events.get(op);
		list && list.forEach(fn=>{
			fn(args);
		})

		this.listeners.forEach(listener=>{
			listener.emit.call(listener, op, args);
		})
	}

	emitAsync(op, ...args) {
		dpc(()=>{
			this.emit(op, ...args);
		})
	}

	addListener(listener) {
		if(this.listeners.indexOf(listener)<0)
			this.listeners.push(listener);
	}

	removeListener(listener) {
		let index = this.listeners.indexOf(listener);
		if(index>-1)
			this.listeners.splice(index, 1);
	}

	getListeners() {
		return this.listeners;
	}
}
"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_TOOLBAR_ITEM_406 : &'static str = r###"


/**
* @class FlowToolbarItem
* @extends BaseElement
* @example
*   <flow-toolbar-item></flow-toolbar-item>
*
*/

export class FlowToolbarItem extends BaseElement {
	static get properties() {
		return {
			text:{type:String, value:""},
			subText:{type:String, value:""},
			icon:{type:String, icon:""},
			pressedText:{type:String, value:""},
			pressedSubText:{type:String, value:""},
			pressedIcon:{type:String, icon:""},
			togglable:{type:Boolean, reflect:true},
			pressed:{type:Boolean, reflect:true}
		}
	}

	static get styles() {
		return css`
			:host{
				position:relative;display:block;
				text-align:var(--flow-toolbar-item-text-align, center);
				padding:var(--flow-toolbar-item-padding, 5px);
				max-width:var(--flow-toolbar-item-max-width, 80px);
				min-width:var(--flow-toolbar-item-min-width, 60px);
			}
			:host:before{
				position: absolute;
			    left: 0px;
			    top: -2px;
			    bottom: -2px;
			    right: 0px;
			    background:var(--flow-toolbar-item-shadow-bg, rgba(100,100,100, 0.2));
			    border-radius: 100px;
			    transform-origin: center center;
			    transform: scale(0,0);
			    transition: all 0.2s ease;
			    content:"";z-index:-1;
			}
			:host(:not(.disabled)){
				cursor:pointer;
			}
			:host(:not(.disabled):hover):before{
			    border-radius: 3px;
			    transform: scale(1,1);
			}
			.icon{
				display:block;
				width:var(--flow-toolbar-item-icon-width, 28px);
			    height:var(--flow-toolbar-item-icon-height, 28px);
			    margin:var(--flow-toolbar-item-icon-margin, 0px auto);
			    --fa-icon-size:var(--flow-toolbar-item-icon-width, 28px);
			}
			.text{
				font-size:var(--flow-toolbar-item-text-font-size, 0.6rem);
				user-select:none;
			}
			.sub-text{
				font-size:var(--flow-toolbar-item-sub-text-font-size, 0.5rem);
				user-select:none;
			}
			:host([togglable][pressed]){
				background:var(--flow-toolbar-item-pressed-bg, initial);
				color:var(--flow-toolbar-item-pressed-color, var(--flow-primary-color));
				--fa-icon-color:var(--flow-toolbar-item-pressed-icon-color, var(--flow-primary-color));
			}
		`;
	}

	constructor(){
		super();
		this.initPropertiesDefaultValues();
		this.addEventListener("click", this.onClick.bind(this));
	}

	render() {
		let {text, subText, icon, togglable, pressed} = this;
		if(togglable && pressed){
			let {pressedText, pressedSubText, pressedIcon} = this;
			if(pressedText)
				text = pressedText;
			if(pressedSubText)
				subText = pressedSubText;
			if(pressedIcon)
				icon = pressedIcon;
		}
		return html`${icon? html`<fa-icon class="icon" icon="${icon}" size="28"></fa-icon>`:''}
				${text? html`<div class="text">${text}</div>`:''}
				${subText? html`<div class="sub-text">${subText}</div>`:''}
				`
	}
	onClick(){
		if(!this.togglable)
			return
		this.pressed = !this.pressed;
		this.fire("flow-toolbar-item-state", {
			pressed:this.pressed, el:this,
			code:this.dataset.code
		}, {bubbles:true});
	}
}

FlowToolbarItem.define('flow-toolbar-item');

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_GRIDSTACK_PANEL_361 : &'static str = r###"



/**
* @class FlowGridStackPanel
* @extends BaseElement
* @prop {String} heading
* @prop {Boolean} opened
*
* @cssvar {background-color} [--flow-gridstack-panel-bg=#FFF]
* @cssvar {color} [--flow-gridstack-panel-color=var(--flow-color)]
* @cssvar {border-radius} [--flow-gridstack-panel-border-radius=4px]
* @cssvar {border} [--flow-gridstack-panel-border=1px solid var(--flow-primary-color)]
* @cssvar {padding} [--flow-gridstack-panel-heading-padding=5px]
* @cssvar {background-color} [--flow-gridstack-panel-heading-bg=var(--flow-primary-color)]
* @cssvar {color} [--flow-gridstack-panel-head-color=var(--flow-primary-invert-color)]
* @cssvar {align-items} [--flow-gridstack-panel-heading-align-items=center]
* @cssvar {overflow} [-flow-gridstack-panel-heading-overflow=hidden]
* @cssvar {text-overflow} [--flow-gridstack-panel-heading-text-overflow=ellipses]
* @cssvar {flex} [--flow-gridstack-panel-head-flex=1]
* @cssvar {align-items} [--flow-gridstack-panel-head-align-items=center]
* @example
*   <flow-gridstack-panel></flow-gridstack-panel>
*
*/

/*
the following causes jsdoc to fail:
... @ cssvar {--fa-icon-color} [--flow-gridstack-panel-head-color=var(--flow-primary-invert-color)]

*/

export const FlowGridStackPanelMixin = (base)=>{
class FlowGridStackPanelKlass extends base {
	static get properties() {
		return {
			heading:{type:String, value:"Hello"},
			//opened:{type:Boolean, value:false, reflect:true}
		}
	}

	static get styles() {
		return css`
			:host {
				display:flex;flex-direction:column;
				align-items:stretch;
				justify-content:start;
				background-color:var(--flow-gridstack-panel-bg, #FFF);
				color:var(--flow-gridstack-panel-color, var(--flow-color));
				border-radius:var(--flow-gridstack-panel-border-radius, 4px);
				border:var(--flow-gridstack-panel-border, 1px solid var(--flow-primary-color));
				box-sizing:border-box;
				--flow-dropdown-content-right:2px;
				--flow-dropdown-border:var(--flow-gridstack-dd-border, 1px solid var(--flow-primary-color));
			}
			.heading{
				text-overflow:elipsis;overflow:hidden;
				padding:var(--flow-gridstack-panel-heading-padding, 5px);
				background-color:var(--flow-gridstack-panel-heading-bg, var(--flow-primary-color));
				color:var(--flow-gridstack-panel-head-color, var(--flow-primary-invert-color));
				display:flex;flex-direction:row;
				align-items:var(--flow-gridstack-panel-heading-align-items, center);
				overflow:var(--flow-gridstack-panel-heading-overflow, hidden);
				text-overflow:var(--flow-gridstack-panel-heading-text-overflow, ellipses);
				--fa-icon-color:var(--flow-gridstack-panel-head-color, var(--flow-primary-invert-color));
			}
			.head{
				display:flex;flex-direction:row;height:100%;
				flex:var(--flow-gridstack-panel-head-flex, 1);
				align-items:var(--flow-gridstack-panel-head-align-items, center);
			}
			.drag-region{cursor:move;}
			.body{overflow:auto;flex:1}
			/*:host(:not([opened])) .body{
				display:none;
			}*/
			.heading fa-icon:not(.disabled){cursor:pointer}
		`;
	}
	constructor(){
		super();
		this.initPropertiesDefaultValues();
	}

	render() {
		return html` 
			<div class="heading" @click="${this.onHeadingClick}">
				${this.renderHeadPrefix()}
				<div class="head drag-region" 
					@click="${this.onHeadClick}">${this.renderHead()}</div>
				${this.renderHeadSuffix()}
			</div>
			<flow-dropdown id="settingDD" no-trigger right-align absolute>
				${this.renderSettings()}
			</flow-dropdown>
			${this.renderExtraBody()}
			<div class="body">${this.renderBody()}${this.renderBodySuffix()}</div>`;
	}
	renderExtraBody(){
		return '';
	}
	renderBodySuffix(){
		return '';
	}

	update(changes){
		super.update(changes);
		this.bindDDTriggers();
	}

	renderHeadPrefix(){
		return html `<fa-icon icon="window-maximize"></fa-icon>`
	}
	renderHeadSuffix(){
		return html `
		<fa-icon class="setting-trigger" data-trigger-for="settingDD" icon="cog"></fa-icon>
		<fa-icon icon="times" @click="${this.onClosePanelClick}"></fa-icon>`
	}
	renderHead(){
		return this.heading || ''
	}
	renderBody(){
		return html`
		<div>
			PANEL : ${Math.random()*10000}
			<div>contexts:${JSON.stringify(this.ctxworkspaces||[])}</div>
		</div>
		`
	}

	renderSettings(){
		return html`
		<div class="head">
		<flow-btn @click="${this.onClosePanelClick}">
			<fa-icon icon="times"></fa-icon>
		</flow-btn>
		</div>`
	}

	onHeadingClick(){
		
	}

	acceptContext(ctx){
		return super.acceptContext(ctx)
	}

	onContextsUpdate(){

	}

	onHeadClick(){
		this.opened = !this.opened;
	}

	serialize(){
		let {opened} = this;
		let data = Object.assign({}, super.serialize(), {
			opened
		});
		return data;
	}
	deserialize(data){
		super.deserialize(data);
		let {opened} = data||{};
		this.opened = !!opened;
	}

	getGridstackDragHandle(){
		return this.renderRoot.querySelector('.heading .head')
	}

	toggleSettingDD(){
		this.settingDD.toggle();
	}

	onClosePanelClick(){
		return this.fire(
			"remove-gridstack-panel-request", 
			{panel:this},
			{bubbles:true, cancelable:true},
			null,
			true
		);
	}
}

return FlowGridStackPanelKlass
}

export const FlowGridStackPanelImpl = FlowGridStackPanelMixin(BaseElement);
export class FlowGridStackPanel extends FlowContextListenerMixin(FlowGridStackPanelImpl){};

FlowGridStackPanel.define('flow-gridstack-panel');

"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_CONTEXT_TEST_348 : &'static str = r###"




export class FlowContextA extends FlowContextElement{
	static get properties(){
		return {
			name:{type:String, value:"Context A"},
			type:{type:String, value:"example"},
			code:{type:String, value:"ctxa"}
		}
	}
	static get styles(){
		return [FlowContextElement.styles, css`
			:host{display:block;padding:5px}
		`]
	}
}
FlowContextA.init();

export class FlowContextB extends FlowContextElement{
	static get properties(){
		return {
			name:{type:String, value:"Context B"},
			type:{type:String, value:"example"},
			code:{type:String, value:"ctxb"}
		}
	}
}
FlowContextB.init();

export class FlowContextC extends FlowContextElement{
	static get properties(){
		return {
			name:{type:String, value:"Context C"},
			type:{type:String, value:"example"},
			code:{type:String, value:"ctxc"}
		}
	}
}
FlowContextC.init();

export class FlowContextD extends FlowContextElement{
	static get properties(){
		return {
			name:{type:String, value:"Context D"},
			type:{type:String, value:"example"},
			code:{type:String, value:"ctxd"}
		}
	}
}
FlowContextD.init();

export class FlowContextE extends FlowContextElement{
	static get properties(){
		return {
			name:{type:String, value:"Context E"},
			type:{type:String, value:"example"},
			code:{type:String, value:"ctxe"}
		}
	}
}
FlowContextE.init();

FlowContextWorkspace({
	name:"Workspace A",
	code:"group1",
	contexts:[{
		code:"ctxa"
	},{
		code:"ctxb"
	}]
})


"###;

const ASPECTRON_FLOW_UX_SRC_FLOW_CONTEXT_422 : &'static str = r###"


export const FlowContextWorkspaces = new Map();
export const FlowContexts = new Map();

const boxStyle = css`
	:host{
		display:block;padding:5px;border-radius:3px;margin-top:5px;
		border:1px solid var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1)));
	}
	.head{
		display:flex;align-items:center;
		padding:5px;
	}
	.head .name{
		flex:1;overflow:hidden;text-overflow:ellipsis;
		margin-right:10px;
	}
	.head fa-icon{margin:5px;}
	fa-icon[disabled]{
		pointer-events:none;opacity:0.5;
	}
	fa-icon:not([disabled]){cursor:pointer}
`

let workspaceCount = 0;

export class FlowContextWorkspaceItem extends BaseElement{
	static get properties(){
		return {
			data:{type:Object},
			multi:{type:Boolean, reflect:true}
		}
	}
	static get styles(){
		return css`
			:host{
				display:flex;align-items:center;
			}
			:host([multi].item){
				padding:5px;margin:0px 2px 2px 0px;
				border-radius:3px;
				border:1px solid var(--flow-border-color, var(--flow-primary-color, rgba(0,151,115,1)));
			}
		`
	}
	render(){
		let {data={name:""}} = this;
		//console.log("WorkspaceItem:data", this, this.parentNode, data.name, JSON.stringify(data))
		return html`${data.name||""}`
	}
}

FlowContextWorkspaceItem.define("flow-context-workspace-item");

export const objectToProperties = config=>{
	let props = {};
	for (const [key, value] of Object.entries(config)) {
		if(value.value !== undefined && value.type){
			props[key] = value;
			continue;
		}
		props[key] = {
			type:utils.valueToDataType(value)||String,
			value
		};
	}

	return props;
}

export class ContextElement extends BaseElement{
	static get props(){
		if(!this._finalized){
			this.finalize();
			this._finalized = true;
		}
		let keys = [...this._classProperties.keys()];
		let properties = this.properties;
		let props = {}, ctor=this;
		while(properties && keys.length){
			[...keys].forEach(key=>{
				if(properties.hasOwnProperty(key)){
					let p = properties[key]
					if(p.hasOwnProperty('value')){
						props[key] = p.value;
						let index = keys.indexOf(key);
						//console.log("key, index:::", key, index, keys)
						keys.splice(index, 1);
						//console.log("key, index", key, index, keys)
					}
				}
			});
			ctor = ctor.__proto__;
			properties = ctor.properties;
		}


		/*
		let props2 = this._classProperties;
		let _props = {};
		props2.forEach((v, key)=>{
			_props[key] = v.value;
		})
		*/

		//console.log("propspropsprops", keys, props)

		return props;
	}
}

export class FlowContextElement extends ContextElement{
	static get properties(){
		return {
			name:{type:String, value:""},
			code:{type:String, value:""},
			type:{type:String, value:""},
			config:{type:Object, value:{}},
			advance:{type:Boolean, reflect:true}
		}
	}
	static get styles(){
		return [boxStyle, css`
			:host{display:inline-block;padding:5px;margin:0px 5px 5px;}
			:host(:not([advance])) .remove-ctx-icon{display:none}
			:host(:not([advance])){border-color:transparent;}
		`]
	}
	static init(){
		this.config = this.props;
		let {code} = this.config;
		//console.log("FlowContextElement:init", code, this.config)
		if(!FlowContexts.has(code)){
			FlowContexts.set(code, this);
			this.define(`flow-ctx-${code}`)
		}
	}

	render(){
		return html`
		<div class="head">
			<span class="name">${this.name||''}</span>
			<fa-icon class="remove-ctx-icon" icon="trash-alt"
				@click="${this.onRemoveClick}"></fa-icon>
		</div>
		${this.renderBody()}`
	}
	constructor(){
		super();
		this.initPropertiesDefaultValues();
	}
	fireNotification(){
		let args = this.buildConfig();
		this.fire("context-updated", args, {bubbles:true})
	}
	onRemoveClick(){
		this.fire("remove-ctx-request", {code:this.code}, {bubbles:true})
	}
	renderBody(){
		return '';
	}
	buildConfig(){
		let {code, type, name} = this;
		let result = {name, type, code};
		result.config = this.getConfig();
		return result;
	}
	setConfig(config){
		this.config = config;
	}
	getConfig(){
		return this.config || {};
	}
}

//window.FlowContextWorkspaces = FlowContextWorkspaces;
export class FlowContextWorkspaceElement extends ContextElement{
	static get properties(){
		return {
			name:{type:String, value:""},
			code:{type:String, value:""},
			contexts:{type:Array, value:[]},
			readonly:{type:Boolean, value:false},
			advance:{type:Boolean, reflect:true}
		}
	}

	static init(){
		this.config = this.props;
		let {code} = this.config;
		//console.log("FlowContextWorkspaceElement:init", code, this.config, this.properties)
		if(code){
			if(!FlowContextWorkspaces.has(code)){
				FlowContextWorkspaces.set(code, this);
				this.define(`flow-ctxworkspace-${code}`)
			}
		}
	}
	static createContextNode(code, attr, props){
		/*
		let el = this.createElement(`flow-ctx-${code}`, attr, props);
		el.classList.add("flow-context");
		return el;
		*/
		let testValue = 123;
		let tags = new Map();
		tags.set("tag", `flow-ctx-${code}`);
		//tags.set("attr1", code)
		return flowHtml`${tags}<{tag} ?advance=${attr.advance}
			config=${JSON.stringify(props.config||{})} class="flow-context"></{tag}>`;
	}
	static createContextWorkspaceNode(code, attr, props){
		/*
		let el = this.createElement(`flow-ctxworkspace-${code}`, attr, props);
		el.classList.add("flow-workspace");
		return el;
		*/
		let tags = new Map();
		tags.set("tag", `flow-ctxworkspace-${code}`);
		const {manager} = props;

		return flowHtml`${tags}<{tag} ?advance=${manager.advance}
			.manager=${manager} class="flow-workspace"></{tag}>`;
	}
	static get styles() {
		return [boxStyle, css`
			:host(:not([advance])) .head {
				display:none
			}
		`]
	}

	static updateConfig(config){
		//console.log("updateConfig\n"+JSON.stringify(this.config)+"\n"+
		//JSON.stringify(config)+"\n")
		this.config = config;
		let props = deepClone(this.config);
		Manager.getContextManager().saveWorkspaceAndNotify(props)
	}
	

	render(){
		let hasAddable = this.getAddableContexts().length
		return html`
		<div class="head">
			${this.renderHeadPrefix()}
			<span class="name">
				<flow-input ?readonly=${this.readonly} label="Workspace Name" @changed="${this.onNameChange}"
					class="name-input" .value="${this.name}" pattern="^.{3,20}$">
				</flow-input>
			</span>
			${this.renderHeadTools({hasAddable})}
		</div>
		${this.renderContexts()}`
	}
	renderHeadPrefix(){
		return '';
	}
	renderHeadTools({hasAddable}){
		let isDirty = this.isDirty();
		return html`<fa-icon icon="plus-circle" ?disabled=${!hasAddable} 
			@click="${this.onAddContextClick}"></fa-icon>
		<fa-icon icon="times" class="reset-workspace-btn" title="Reset"
			?disabled=${!isDirty} @click="${this.onResetClick}"></fa-icon>
		${this.manager.multiWorkspace?html`<fa-icon icon="trash-alt" 
			title="Remove Workspace" @click="${this.onRemoveWorkspaceClick}"></fa-icon>`:''}`;
	}

	renderContexts(){
		let contexts = this.manager.filterContexts(this.getContexts());
		this.log("render:contexts", contexts, JSON.stringify(this.constructor.config))
		return html`
		<div class="contexts" @remove-ctx-request=${this.onRemoveContext}
			@context-updated=${this.onContextUpdate}>
			${contexts.map(ctx=>{
				return this.constructor.createContextNode(ctx.code, {
					advance:this.advance
				}, {
					config: Object.assign({}, ctx.config||{})
				})
				/*let el = this.constructor.createContextNode(ctx.code);
				if(!el.setConfig){
					console.log("el.setConfig missing", el)
				}
				el.setConfig(Object.assign({}, ctx.config||{}));
				return el;*/
			})}
		</div>`
	}

	getContexts(){
		let klass;
		return (this.contexts||[]).map(ctx=>{
			klass = FlowContexts.get(ctx.code);
			if(!klass)
				return
			//console.log("ctx", klass.config, JSON.stringify(ctx))
			return Object.assign({}, klass.config, ctx);
		}).filter(ctx=>!!ctx);
	}

	isDirty(){
		let {config, defaultConfig} = this.constructor;
		let hash1 = this.makeHash(config);
		let hash2 = this.makeHash(defaultConfig);
		//console.log("hash1 != hash2\n"+hash1+"\n----\n"+hash2)
		return hash1 != hash2;
	}
	makeHash(obj){
		return JSON.stringify(obj);
	}

	buildConfig(){
		let {name, code} = this; 
		let config = {name, code};
		this.getPropsKeys().forEach(key=>{
			if(key != 'contexts')
				config[key] = this[key];
		})
		config.contexts = this.buildContextConfig();
		return config;
	}
	buildContextConfig(){
		let contexts = this.renderRoot.querySelectorAll(".flow-context");
		contexts = [...contexts].map(ctx=>ctx.buildConfig());
		this.log("buildContextConfig:contexts", contexts)
		return contexts;
	}

	onNameChange(e){
		if(e.detail.value.length < 3)
			return
		this.name = e.detail.value;
	}

	onRemoveContext(e){
		let index = this.contexts.findIndex(c=>c.code == e.detail.code);
		if(index >-1){
			this.contexts.splice(index, 1);
			this.requestUpdate("contexts")
		}
	}

	getAddableContexts(){
		//console.log("getAddableContexts:this.contexts", this.contexts)
		return this.manager.filterContexts([...FlowContexts.values()].map(ctx=>ctx.config).filter(ctx=>{
			return !this.contexts.find(c=>c.code==ctx.code)
		}));
	}

	onResetClick(e){
		let config = deepClone(this.constructor.defaultConfig, true)
		this.initConfig(config)
	}

	onAddContextClick(e){
		//this.fire('add-context', {code:this.code}, {bubbles:true})
		let items = this.getAddableContexts();
		//console.log("items", items)
		let body = html`<flow-menu data-name="contexts" multiple>
			${items.map(item=>{
				return html`<flow-menu-item value="${item.code}">${item.name}</flow-menu-item>`
			})}
			</flow-menu>`;

		FlowDialog.show({
			title:'Select Context',
			body,
			hideCloseBtn:true,
			compact:true,
			btns:[{
				text:"Close",
				handler:(resolve, result)=>{
					resolve();
				}
			},{
				text:"Done",
				cls:"primary",
				handler:(resolve, result)=>{
					resolve();
					let {contexts} = result.values;
					contexts = contexts.map(code=>{
						return {code};
					})
					//console.log("result", contexts)
					if(contexts.length)
						this.contexts = [...contexts, ...this.contexts];
				}
			}]
		});
	}

	constructor(){
		super();
		//this.initPropertiesDefaultValues();
		this.initConfig(this.constructor.config);
		this.saveState()
	}

	initConfig(config){
		Object.entries(config).forEach(([key, value])=>{
			//console.log("initConfig", key+"=>", value)
			this[key] = value;
		});
	}

	updated(changes){
		super.updated(changes)
		this.updateStaticValues();
		let ctor = this.constructor;
		if(!ctor.defaultConfig){
			ctor.defaultConfig = deepClone(ctor.config)
			this.recheckDirty();
		}
	}
	recheckDirty(){
		let btn = this.renderRoot.querySelector('.reset-workspace-btn');
		if(!btn)
			return
		//console.log("this.isDirty()", this.isDirty())
		if(this.isDirty())
			btn.removeAttribute('disabled')
		else
			btn.setAttribute('disabled', true)
	}
	onContextUpdate(){
		this.updateStaticValues();
		this.recheckDirty()
	}
	getPropsKeys(extraKeys=[]){
		return ["name", "contexts", "readonly"].concat(extraKeys);
	}
	updateStaticValues(){
		this.updatePropValues(this.getPropsKeys());
	}
	updatePropValues(keys){
		let props = this.constructor.config;
		keys.forEach(key=>{
			if(key == "contexts"){
				props[key] = this.buildContextConfig();
				return
			}
			props[key] = this[key];
		});
		this.fireUpdateNotification();
	}
	buildNotificationArgs(){
		let {code} = this;
		let args = {code};
		let props = this.constructor.config;
		this.getPropsKeys().forEach(key=>{
			args[key] = props[key];
		})
		return args;
	}
	fireUpdateNotification(){
		let props = this.buildNotificationArgs();
		if(!this.validateNotificationHash(props))
			return
		this.fire("workspace-updated", {props, el:this}, {bubbles:true})
	}
	validateNotificationHash(props){
		let hash = JSON.stringify(props);
		if(this._hash == hash){
			//this.log("this._hash == hash\n"+this._hash+"\n--------\n"+hash)
			return false;
		}
		this._hash = hash;
		return true;
	}
	saveState(){
		this._hash = JSON.stringify(this.buildNotificationArgs());
	}
}

export const FlowContext = (config, base)=>{
	let props = objectToProperties(config);
	class klass extends (base||FlowContextElement){
		static get properties(){
			return props;
		}
	}

	klass.init();
}

export const CreateFlowContextWorkspace = (config, base)=>{
	let props = objectToProperties(config);
	//console.log("CreateFlowContextWorkspace", config, props)
	class klass extends (base||FlowContextWorkspaceElement){
		static get properties(){
			return props;
		}
		_initLog(forceLog){
			super._initLog(forceLog, `FlowContextWorkspace:${config.code}`);
		}
	}

	klass.init();
}

export const FlowContextListenerMixin = base=>{
	class FlowContextListener extends base{
		static get properties(){
			return {
				ctxworkspaces:{type:Array, value:[]},
				multiWorkspace:{type:Boolean}
			}
		}

		constructor(){
			super();
			this.registerListener("flow-ctxworkspace-updated", this.onContextUpdate.bind(this));
			//console.log("flow-ctxworkspace-loaded: event bind")
			this.registerListener("flow-ctxworkspace-loaded", this.onContextLoaded.bind(this));
		}

		acceptContext(context){
			return !!context.type;
		}
		onContextUpdate(e){
			let {props} = e.detail;
			//console.log("onContextUpdate:props", props, e)
			let workspaces = this.ctxworkspaces||[];
			if(workspaces.includes(props.code))
				this.contextsUpdate();
		}
		onContextLoaded(e){
			//console.log("flow-ctxworkspace-loaded: event got ",this, this.ctxworkspaces)
			this.contextsUpdate();
		}
		contextsUpdate(){
			this.requestUpdate("ctxworkspaces", null)
		}
		getContextManagerConfig(){
			return {
				workspaces:this.ctxworkspaces||[],
				multiWorkspace:!!this.multiWorkspace
			}
		}
		setContextManagerConfig(config){
			let {workspaces} = config;
			//console.log("workspaces", workspaces)
			this.ctxworkspaces = workspaces;
			this.contextsUpdate();
		}

		openContextManager(){
			Manager.open(this);
		}

		getContextWorkspaces(){
			let workspaces = this.ctxworkspaces||[];
			return workspaces.map(code=>{
				let workspace = (FlowContextWorkspaces.get(code)||{}).config;
				if(!workspace)
					return false;
				workspace = Object.assign({}, workspace)
				workspace.contexts = (workspace.contexts||[]).map(ctx=>{
					let klass = FlowContexts.get(ctx.code);
					if(!klass)
						return false;
					return Object.assign({}, klass.config, ctx);
				}).filter(ctx=>this.acceptContext(ctx))
				return workspace;
			}).filter(w=>w);
		}

		serialize(){
			let {ctxworkspaces} = this;
			return Object.assign({}, super.serialize(), {
				ctxworkspaces:this.getContextWorkspaces()
			});
		}
		deserialize(data){
			super.deserialize(data);
			let {ctxworkspaces=[]} = data||{};
			//console.log("got contexts", data)
			this.ctxworkspaces = ctxworkspaces;
			this.contextsUpdate();
		}
	}

	return FlowContextListener;
}

export class FlowContextManager extends BaseElement{
	static get properties(){
		return {
			selected:{type:Array, value:[]},
			isLoading:{type:Boolean},
			advance:{type:Boolean, reflect:true, value:true}
		}
	}

	static get styles(){
		return [ScrollbarStyle, css `
			:host{--fa-icon-size:20px;}
			dialog{
				display:flex;padding:0px;height:700px; top:2vh;
				width:800px;max-width:95vw;
				max-height:95vh;flex-direction:column;
			    border:var(--flow-context-manager-dialog-border, 2px solid var(--flow-primary-color, #025763));
			    border-radius:var(--flow-context-manager-dialog-border-radius, 4px);
			    min-width:var(--flow-context-manager-dialog-min-width, 300px);
			    min-height:var(--flow-context-manager-dialog-min-height, 200px);
			    background-color:var(--flow-context-manager-dialog-bg, var(--flow-dialog-background-color, var(--flow-background-color)));
			    color:var(--flow-context-manager-dialog-color, var(--flow-dialog-color, var(--flow-color)));
			    box-shadow:var(--flow-box-shadow);
			}
			dialog:not([open]){display:none}
			.head,.header{
				display:flex;align-items:center;justify-content:center;
				padding:var(--flow-context-manager-head-padding, 10px);
				line-height:1.3;
			}
			.header{
				justify-content:flex-end;min-height:72px
			}
			.head-text{
				flex:1;overflow:hidden;text-overflow:ellipsis;
				font-size:var(--flow-context-manager-head-text-font-size, 1.2rem);
				margin-right:15px;
			}
			.header{
				padding:var(--flow-context-manager-header-padding, 0px 5px);
			}
			.workspace-selector{
				flex:1;--flow-selector-dropdown-width:100%;
				--flow-select-selected-max-width:100%;
			}
			.close-icon{cursor:pointer;--fa-icon-size:20px;}
			.body{
				padding:var(--flow-context-manager-body-padding, 10px);
				flex:1;overflow:auto;
			}
			.buttons{
			    margin:10px;display:flex;flex-wrap:wrap;justify-content:flex-end;
			}
			.buttons flow-btn{margin:0px 5px;align-items:center;display:flex;}
			.buttons flow-btn:last-child{margin-right:0px;}
			.buttons flow-btn:first-child{margin-left:0px;}
			dialog[loading]:after{
				content:"";z-index:10000;
				position:absolute;left:0px;top:0px;width:100%;height:100%;
				background-color:var(--flow-context-manager-loading-mask-bg, rgba(0,0,0, 0.5))
			}
			:host(:not([advance])) .advance-tools,
			[hidden]{
				display:none;
			}
			:host([advance]) flow-btn.toggle-advance-mode-btn{
				background-color:var(--flow-primary-color);
				color:var(--flow-primary-invert-color);
				--fa-icon-color:var(--flow-primary-invert-color);
			}
		`];
	}

	static get _tagName(){
		return 'flow-context-manager';
	}

	static getContextManager(){
		let ctxManger = document.querySelector(this._tagName);
		if(ctxManger)
			return ctxManger;
		ctxManger = document.createElement(this._tagName);
		document.body.appendChild(ctxManger);
		return ctxManger;
	}

	static createContextManager(){
		return this.getContextManager();
	}

	static open(cmp){
		let ctxManger = this.getContextManager();
		ctxManger.showModal(cmp)
	}

	render(){
		return html`
		<link rel="stylesheet" type="text/css" href="${baseUrl}/resources/extern/dialog/dialog-polyfill.css" />
		<dialog @close=${this.onDialogClose} ?loading=${this.isLoading} ?advance=${this.advance}>
			<div class="head">
				<span class="head-text">${this.heading||'Context Manager'}</span>
				<fa-icon class="close-icon" @click="${this.onCloseClick}" icon="times"></fa-icon>
			</div>
			<div class="header">
				${this.renderWorkspaceSelector()}
				${this.renderHeaderTools()}
			</div>
			<div class="body" @workspace-updated="${this.onWorkspaceUpdate}">
				${this.renderWorkspaces()}
			</div>
			<div class="buttons">
				<flow-btn @click="${this.onCloseClick}">Close</flow-btn>
				<flow-btn @click="${this.onDoneClick}" class="primary">Done</flow-btn>
			</div>
		</dialog>`;
	}

	constructor(){
		super();
		this.restoreWorkspaces();
		this.initPropertiesDefaultValues();
	}

	renderWorkspaceSelector(){
		let workspaces = [...FlowContextWorkspaces.values()].map(w=>w.config);
		//console.log("this.selected", this.selected)
		return html`<flow-selector class="workspace-selector advance-tools" label="Select Workspace"
			?multiple=${this.multiWorkspace} .selected="${this.selected.slice(0)}"
			@select="${this.onWorkspacesSelectionChange}">
			${workspaces.map(w=>{
				return this.renderWorkspaceItem(w);
			})}
		</flow-selector>`
	}
	renderWorkspaceItem(w){
		return html`<flow-context-workspace-item 
					value="${w.code}" ?multi=${this.multiWorkspace}
					class="menu-item" data-text="${w.name}" .data="${w}"></flow-context-workspace-item>`
	}

	renderHeaderTools(){
		return html`
		<flow-btn class="advance-tools add-workspace-btn" title="Add workspace"  
			@click="${this.onCreateWorkspaceClick}" >
			<fa-icon icon="plus"></fa-icon>
		</flow-btn>
		<!--flow-btn class="close-panel-btn" title="Remove Panel" 
			@click="${this.onClosePanelClick}" >
			<fa-icon icon="times"></fa-icon>
		</flow-btn>
		<flow-btn class="toggle-advance-mode-btn" title="Toggle advance mode"
			@click="${this.onToggleModeClick}" >
			<fa-icon icon="cogs"></fa-icon>
		</flow-btn-->`
	}

	renderWorkspaces(){
		let workspaces = this.getWorkspacesConfig();
		let nodes = workspaces.map(w=>{
			return FlowContextWorkspaceElement.createContextWorkspaceNode(w.code, {}, {
				manager:this
			})
		});
		return nodes;
	}

	buildNewWorkspaceConfig(config){
		return Object.assign({
			name:`Workspace ${++workspaceCount}`,
			code:`workspace${(Math.random()*10000).toFixed(0)}`,
			contexts:[]
		}, config||{});
	}

	createNewWorkspace(baseClass){
		let config = this.buildNewWorkspaceConfig();
		CreateWorkspace(config, baseClass);
		return config;
	}

	filterContexts(contexts){
		let cmp = this.getHostComponent();
		if(!cmp)
			return [];
		return (contexts||[]).filter(ctx=>cmp.acceptContext(ctx))
	}

	saveWorkspace(props){
		let workspaces = [];
		FlowContextWorkspaces.forEach(klass=>{
			let config = Object.assign({}, klass.config);
			workspaces.push(config);
		});
		this.saveWorkspacesConfig(workspaces);
	}

	saveWorkspacesConfig(workspaces){
		this.constructor.setLocalSetting("ctxworkspaces", JSON.stringify(workspaces))
	}

	fetchWorkspacesConfig(){
		return new Promise((resolve, reject)=>{
			let data = JSON.parse(this.constructor.getLocalSetting("ctxworkspaces", "[]"))
			//just to test delay in loading
			//setTimeout(()=>{
				resolve(data)
			//}, 1000);
		})
	}
	restoreWorkspaces(){
		this.isLoading = true;
		let p = this.fetchWorkspacesConfig();
		p.then(workspaces=>{
			this.loadWorkspacesConfig(workspaces)
		},err=>{

		}).catch((e)=>{

		}).finally(()=>{
			this.isLoading = false;
		})
		
	}
	loadWorkspacesConfig(workspaces){
		FlowLoadWorkspaces(workspaces);
		this.requestUpdate("_workspace", null)
		//console.log("low-ctxworkspace-loaded : event fire")
		
	}

	onWorkspaceUpdate(e){
		let {props} = e.detail;
		this.saveWorkspaceAndNotify(props);
	}
	saveWorkspaceAndNotify(props){
		this.saveWorkspace(props);
		this.fire("flow-ctxworkspace-updated", {props}, {}, window);
	}

	onToggleModeClick(){
		this.advance = !this.advance;
	}

	onClosePanelClick(){
		let cmp = this.getHostComponent();
		if(!cmp || !cmp.onClosePanelClick)
			return

		let e = cmp.onClosePanelClick();
		dpc(100, ()=>{
			//console.log("e:::", e, e.defaultPrevented)
			if(e.defaultPrevented)
				return
			this.close();
		})
		
	}

	onCreateWorkspaceClick(){
		let config = this.createNewWorkspace();
		if(this.multiWorkspace){
			this.selected.push(config.code);
			this.requestUpdate("workspaces", null)
		}else{
			this.selected = [config.code]
		}
		//console.log("onCreateWorkspaceClick", FlowContextWorkspaces, this.selected)
	}

	onWorkspacesSelectionChange(e){
		let {selected} = e.detail;
		if(!Array.isArray(selected)){
			if(selected)
				selected = [selected]
			else
				selected = [];
		}
		this.selected = selected;
		//console.log("Workspaces:selected", selected)
	}
	getWorkspacesConfig(){
		let cmp = this.getHostComponent();
		if(!cmp)
			return [];
		let config = [];
		//let {workspacesMap} = this;
		this.selected.map(wCode=>{
			let workspace = (FlowContextWorkspaces.get(wCode)||{}).config;
			if(!workspace)
				return
			/*
			let workspaceConfig = workspacesMap.get(wCode);
			if(workspaceConfig){
				console.log("workspaceConfig", workspaceConfig)
				config.push(workspaceConfig);
				return
			}
			*/
			workspace = Object.assign({}, workspace);
			workspace.contexts = workspace.contexts.filter(ctx=>{
				return FlowContexts.get(ctx.code||ctx)
			}).filter(cmp.acceptContext)
			config.push(workspace);
		});

		return config;
	}

	firstUpdated(){
		this.dialog = this.renderRoot.querySelector('dialog');
		dialogPolyfill.registerDialog(this.dialog);
		if(this._show)
			this[this._show]();
	}

	onDialogClose(e){
		if(this._show){
			this[this._show]();
			return
		}
		let detail = {e};
		this.dispatchEvent(new CustomEvent('closed', {detail}))
	}

	showModal(cmp){
		let fromCloseEvent = !cmp;
		cmp = cmp || this.getHostComponent();
		if(!cmp)
			return
		let missingFn = [
			'getContextManagerConfig',
			'setContextManagerConfig',
			'acceptContext'
		].find(fn=>{
			return (typeof cmp[fn] != 'function');
		});

		if(missingFn)
			return this.log("showModal: missing function", missingFn);

		if(!fromCloseEvent)
			this.setHostComponent(cmp);
		this._show = 'showModal';
		if(this.dialog){
			//console.log(this.dialog)
			return this.dialog.showModal();
		}
	}

	setHostComponent(cmp){
		this._cmp = cmp;
		if(cmp){
			let {workspaces, multiWorkspace} = cmp.getContextManagerConfig();
			this.selected = workspaces.map(w=>w.code||w);
			this.workspaces = workspaces;
			this.multiWorkspace = !!multiWorkspace;
		}else{
			this.selected = [];
			this.workspaces = [];
			this.multiWorkspace = false;
		}

		/*
		let {workspaces=[]} = this;
		this.workspacesMap = new Map();
		workspaces.forEach(workspace=>{
			this.workspacesMap.set(workspace.code, workspace);
			/*
			let {contexts=[]} = workspace;
			workspace.contexts = new Map();
			contexts.map(ctx=>{
				workspace.contexts.set(ctx.code, ctx);
			})
			* /
		})*/
	}
	getHostComponent(){
		return this._cmp;
	}

	onCloseClick(){
		this.close();
	}

	buildConfig(){
		let workspaces = this.renderRoot.querySelectorAll(".flow-workspace");
		//return [...workspaces].map(w=>w.buildConfig());
		return [...workspaces].map(w=>w.code);
	}

	onDoneClick(){
		let cmp = this.getHostComponent();
		if(!cmp){
			this.close();
			return
		}
		let workspaces = this.buildConfig();
		//console.log("workspaces-config", workspaces);

		cmp.setContextManagerConfig({workspaces});
		this.close();
	}

	close(){
		this._show = false;
		this.setHostComponent(null);
		if(this.dialog)
			this.dialog.close();
	}

	destroy(){
		this.close();
		this.remove();
	}
}

FlowContextManager.define(FlowContextManager._tagName, {
	'window.dialogPolyfill': baseUrl+'/resources/extern/dialog/dialog-polyfill.js'
});


export const FlowLoadWorkspaces = (workspaces)=>{
	console.log("xxxxxxxxxxxxxxx FlowLoadWorkspaces", workspaces)
	workspaces.forEach(workspace=>{
		if(!workspace?.code)
			return
		let wKlass = FlowContextWorkspaces.get(workspace.code)
		if(wKlass){
			wKlass.updateConfig(workspace)
		}else{
			CreateWorkspace(workspace);
		}
	});
	trigger("flow-ctxworkspace-loaded");
}

let Manager = FlowContextManager;
let CreateWorkspace = CreateFlowContextWorkspace;
export const SetFlowContextManger = manager=>{
	Manager = manager;
}
export const SetFlowContextWorkspaceCreator = creator=>{
	CreateWorkspace = creator;
}


"###;

const ASPECTRON_FLOW_UX_RESOURCES_EXTERN_PROGRESSBAR_PROGRESSBAR_341 : &'static str = r###"
let root = {};
(function(f){
	if(typeof exports==="object"&&typeof module!=="undefined"){
		module.exports=f()
	}else if(typeof define==="function"&&define.amd){
		define([],f)
	}else{
		var g;
		if(typeof window!=="undefined"){
			g=window
		}else if(typeof global!=="undefined"){
			g=global
		}else if(typeof self!=="undefined"){
			g=self
		}else{
			g=this
		}
		g.ProgressBar = f();
		root['ProgressBar'] = g.ProgressBar;
	}
})(function(){var define,module,exports;return (function(){function r(e,n,t){function o(i,f){if(!n[i]){if(!e[i]){var c="function"==typeof require&&require;if(!f&&c)return c(i,!0);if(u)return u(i,!0);var a=new Error("Cannot find module '"+i+"'");throw a.code="MODULE_NOT_FOUND",a}var p=n[i]={exports:{}};e[i][0].call(p.exports,function(r){var n=e[i][1][r];return o(n||r)},p,p.exports,r,e,n,t)}return n[i].exports}for(var u="function"==typeof require&&require,i=0;i<t.length;i++)o(t[i]);return o}return r})()({1:[function(require,module,exports){
/*! Shifty 2.8.3 - https://github.com/jeremyckahn/shifty */
!function(t,n){"object"==typeof exports&&"object"==typeof module?module.exports=n():"function"==typeof define&&define.amd?define("shifty",[],n):"object"==typeof exports?exports.shifty=n():t.shifty=n()}(window,(function(){return function(t){var n={};function e(r){if(n[r])return n[r].exports;var i=n[r]={i:r,l:!1,exports:{}};return t[r].call(i.exports,i,i.exports,e),i.l=!0,i.exports}return e.m=t,e.c=n,e.d=function(t,n,r){e.o(t,n)||Object.defineProperty(t,n,{enumerable:!0,get:r})},e.r=function(t){"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(t,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(t,"__esModule",{value:!0})},e.t=function(t,n){if(1&n&&(t=e(t)),8&n)return t;if(4&n&&"object"==typeof t&&t&&t.__esModule)return t;var r=Object.create(null);if(e.r(r),Object.defineProperty(r,"default",{enumerable:!0,value:t}),2&n&&"string"!=typeof t)for(var i in t)e.d(r,i,function(n){return t[n]}.bind(null,i));return r},e.n=function(t){var n=t&&t.__esModule?function(){return t.default}:function(){return t};return e.d(n,"a",n),n},e.o=function(t,n){return Object.prototype.hasOwnProperty.call(t,n)},e.p="",e(e.s=3)}([function(t,n,e){"use strict";(function(t){e.d(n,"e",(function(){return v})),e.d(n,"c",(function(){return _})),e.d(n,"b",(function(){return m})),e.d(n,"a",(function(){return b})),e.d(n,"d",(function(){return w}));var r=e(1);function i(t,n){for(var e=0;e<n.length;e++){var r=n[e];r.enumerable=r.enumerable||!1,r.configurable=!0,"value"in r&&(r.writable=!0),Object.defineProperty(t,r.key,r)}}function u(t){return(u="function"==typeof Symbol&&"symbol"==typeof Symbol.iterator?function(t){return typeof t}:function(t){return t&&"function"==typeof Symbol&&t.constructor===Symbol&&t!==Symbol.prototype?"symbol":typeof t})(t)}function o(t,n){var e=Object.keys(t);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(t);n&&(r=r.filter((function(n){return Object.getOwnPropertyDescriptor(t,n).enumerable}))),e.push.apply(e,r)}return e}function a(t){for(var n=1;n<arguments.length;n++){var e=null!=arguments[n]?arguments[n]:{};n%2?o(Object(e),!0).forEach((function(n){c(t,n,e[n])})):Object.getOwnPropertyDescriptors?Object.defineProperties(t,Object.getOwnPropertyDescriptors(e)):o(Object(e)).forEach((function(n){Object.defineProperty(t,n,Object.getOwnPropertyDescriptor(e,n))}))}return t}function c(t,n,e){return n in t?Object.defineProperty(t,n,{value:e,enumerable:!0,configurable:!0,writable:!0}):t[n]=e,t}var f="undefined"!=typeof window?window:t,s=f.requestAnimationFrame||f.webkitRequestAnimationFrame||f.oRequestAnimationFrame||f.msRequestAnimationFrame||f.mozCancelRequestAnimationFrame&&f.mozRequestAnimationFrame||setTimeout,l=function(){},h=null,p=null,d=a({},r),v=function(t,n,e,r,i,u,o){var a=t<u?0:(t-u)/i;for(var c in n){var f=o[c],s=f.call?f:d[f],l=e[c];n[c]=l+(r[c]-l)*s(a)}return n},y=function(t,n){var e=t._attachment,r=t._currentState,i=t._delay,u=t._easing,o=t._originalState,a=t._duration,c=t._step,f=t._targetState,s=t._timestamp,l=s+i+a,h=n>l?l:n,p=a-(l-h);h>=l?(c(f,e,p),t.stop(!0)):(t._applyFilter("beforeTween"),h<s+i?(h=1,a=1,s=1):s+=i,v(h,r,o,f,a,s,u),t._applyFilter("afterTween"),c(r,e,p))},_=function(){for(var t=b.now(),n=h;n;){var e=n._next;y(n,t),n=e}},m=function(t){var n=arguments.length>1&&void 0!==arguments[1]?arguments[1]:"linear",e={},r=u(n);if("string"===r||"function"===r)for(var i in t)e[i]=n;else for(var o in t)e[o]=n[o]||"linear";return e},g=function(t){if(t===h)(h=t._next)?h._previous=null:p=null;else if(t===p)(p=t._previous)?p._next=null:h=null;else{var n=t._previous,e=t._next;n._next=e,e._previous=n}t._previous=t._next=null},b=function(){function t(){var n=arguments.length>0&&void 0!==arguments[0]?arguments[0]:{},e=arguments.length>1&&void 0!==arguments[1]?arguments[1]:void 0;!function(t,n){if(!(t instanceof n))throw new TypeError("Cannot call a class as a function")}(this,t),this._currentState=n,this._configured=!1,this._filters=[],this._timestamp=null,this._next=null,this._previous=null,e&&this.setConfig(e)}var n,e,r;return n=t,(e=[{key:"_applyFilter",value:function(t){var n=!0,e=!1,r=void 0;try{for(var i,u=this._filters[Symbol.iterator]();!(n=(i=u.next()).done);n=!0){var o=i.value[t];o&&o(this)}}catch(t){e=!0,r=t}finally{try{n||null==u.return||u.return()}finally{if(e)throw r}}}},{key:"tween",value:function(){var n=arguments.length>0&&void 0!==arguments[0]?arguments[0]:void 0,e=this._attachment,r=this._configured;return!n&&r||this.setConfig(n),this._pausedAtTime=null,this._timestamp=t.now(),this._start(this.get(),e),this.resume()}},{key:"setConfig",value:function(){var n=this,e=arguments.length>0&&void 0!==arguments[0]?arguments[0]:{},r=e.attachment,i=e.delay,u=void 0===i?0:i,o=e.duration,c=void 0===o?500:o,f=e.easing,s=e.from,h=e.promise,p=void 0===h?Promise:h,d=e.start,v=void 0===d?l:d,y=e.step,_=void 0===y?l:y,g=e.to;this._configured=!0,this._attachment=r,this._isPlaying=!1,this._pausedAtTime=null,this._scheduleId=null,this._delay=u,this._start=v,this._step=_,this._duration=c,this._currentState=a({},s||this.get()),this._originalState=this.get(),this._targetState=a({},g||this.get());var b=this._currentState;this._targetState=a({},b,{},this._targetState),this._easing=m(b,f);var w=t.filters;for(var O in this._filters.length=0,w)w[O].doesApply(this)&&this._filters.push(w[O]);return this._applyFilter("tweenCreated"),this._promise=new p((function(t,e){n._resolve=t,n._reject=e})),this._promise.catch(l),this}},{key:"get",value:function(){return a({},this._currentState)}},{key:"set",value:function(t){this._currentState=t}},{key:"pause",value:function(){if(this._isPlaying)return this._pausedAtTime=t.now(),this._isPlaying=!1,g(this),this}},{key:"resume",value:function(){if(null===this._timestamp)return this.tween();if(this._isPlaying)return this._promise;var n=t.now();return this._pausedAtTime&&(this._timestamp+=n-this._pausedAtTime,this._pausedAtTime=null),this._isPlaying=!0,null===h?(h=this,p=this,function t(){h&&(s.call(f,t,1e3/60),_())}()):(this._previous=p,p._next=this,p=this),this._promise}},{key:"seek",value:function(n){n=Math.max(n,0);var e=t.now();return this._timestamp+n===0?this:(this._timestamp=e-n,this._isPlaying||y(this,e),this)}},{key:"stop",value:function(){var t=arguments.length>0&&void 0!==arguments[0]&&arguments[0],n=this._attachment,e=this._currentState,r=this._easing,i=this._originalState,u=this._targetState;if(this._isPlaying)return this._isPlaying=!1,g(this),t?(this._applyFilter("beforeTween"),v(1,e,i,u,1,0,r),this._applyFilter("afterTween"),this._applyFilter("afterTweenEnd"),this._resolve(e,n)):this._reject(e,n),this}},{key:"isPlaying",value:function(){return this._isPlaying}},{key:"setScheduleFunction",value:function(n){t.setScheduleFunction(n)}},{key:"dispose",value:function(){for(var t in this)delete this[t]}}])&&i(n.prototype,e),r&&i(n,r),t}();function w(){var t=arguments.length>0&&void 0!==arguments[0]?arguments[0]:{},n=new b,e=n.tween(t);return e.tweenable=n,e}b.setScheduleFunction=function(t){return s=t},b.formulas=d,b.filters={},b.now=Date.now||function(){return+new Date}}).call(this,e(2))},function(t,n,e){"use strict";e.r(n),e.d(n,"linear",(function(){return r})),e.d(n,"easeInQuad",(function(){return i})),e.d(n,"easeOutQuad",(function(){return u})),e.d(n,"easeInOutQuad",(function(){return o})),e.d(n,"easeInCubic",(function(){return a})),e.d(n,"easeOutCubic",(function(){return c})),e.d(n,"easeInOutCubic",(function(){return f})),e.d(n,"easeInQuart",(function(){return s})),e.d(n,"easeOutQuart",(function(){return l})),e.d(n,"easeInOutQuart",(function(){return h})),e.d(n,"easeInQuint",(function(){return p})),e.d(n,"easeOutQuint",(function(){return d})),e.d(n,"easeInOutQuint",(function(){return v})),e.d(n,"easeInSine",(function(){return y})),e.d(n,"easeOutSine",(function(){return _})),e.d(n,"easeInOutSine",(function(){return m})),e.d(n,"easeInExpo",(function(){return g})),e.d(n,"easeOutExpo",(function(){return b})),e.d(n,"easeInOutExpo",(function(){return w})),e.d(n,"easeInCirc",(function(){return O})),e.d(n,"easeOutCirc",(function(){return S})),e.d(n,"easeInOutCirc",(function(){return j})),e.d(n,"easeOutBounce",(function(){return M})),e.d(n,"easeInBack",(function(){return k})),e.d(n,"easeOutBack",(function(){return P})),e.d(n,"easeInOutBack",(function(){return x})),e.d(n,"elastic",(function(){return T})),e.d(n,"swingFromTo",(function(){return E})),e.d(n,"swingFrom",(function(){return F})),e.d(n,"swingTo",(function(){return A})),e.d(n,"bounce",(function(){return I})),e.d(n,"bouncePast",(function(){return C})),e.d(n,"easeFromTo",(function(){return D})),e.d(n,"easeFrom",(function(){return q})),e.d(n,"easeTo",(function(){return Q}));
/*!
 * All equations are adapted from Thomas Fuchs'
 * [Scripty2](https://github.com/madrobby/scripty2/blob/master/src/effects/transitions/penner.js).
 *
 * Based on Easing Equations (c) 2003 [Robert
 * Penner](http://www.robertpenner.com/), all rights reserved. This work is
 * [subject to terms](http://www.robertpenner.com/easing_terms_of_use.html).
 */
/*!
 *  TERMS OF USE - EASING EQUATIONS
 *  Open source under the BSD License.
 *  Easing Equations (c) 2003 Robert Penner, all rights reserved.
 */
var r=function(t){return t},i=function(t){return Math.pow(t,2)},u=function(t){return-(Math.pow(t-1,2)-1)},o=function(t){return(t/=.5)<1?.5*Math.pow(t,2):-.5*((t-=2)*t-2)},a=function(t){return Math.pow(t,3)},c=function(t){return Math.pow(t-1,3)+1},f=function(t){return(t/=.5)<1?.5*Math.pow(t,3):.5*(Math.pow(t-2,3)+2)},s=function(t){return Math.pow(t,4)},l=function(t){return-(Math.pow(t-1,4)-1)},h=function(t){return(t/=.5)<1?.5*Math.pow(t,4):-.5*((t-=2)*Math.pow(t,3)-2)},p=function(t){return Math.pow(t,5)},d=function(t){return Math.pow(t-1,5)+1},v=function(t){return(t/=.5)<1?.5*Math.pow(t,5):.5*(Math.pow(t-2,5)+2)},y=function(t){return 1-Math.cos(t*(Math.PI/2))},_=function(t){return Math.sin(t*(Math.PI/2))},m=function(t){return-.5*(Math.cos(Math.PI*t)-1)},g=function(t){return 0===t?0:Math.pow(2,10*(t-1))},b=function(t){return 1===t?1:1-Math.pow(2,-10*t)},w=function(t){return 0===t?0:1===t?1:(t/=.5)<1?.5*Math.pow(2,10*(t-1)):.5*(2-Math.pow(2,-10*--t))},O=function(t){return-(Math.sqrt(1-t*t)-1)},S=function(t){return Math.sqrt(1-Math.pow(t-1,2))},j=function(t){return(t/=.5)<1?-.5*(Math.sqrt(1-t*t)-1):.5*(Math.sqrt(1-(t-=2)*t)+1)},M=function(t){return t<1/2.75?7.5625*t*t:t<2/2.75?7.5625*(t-=1.5/2.75)*t+.75:t<2.5/2.75?7.5625*(t-=2.25/2.75)*t+.9375:7.5625*(t-=2.625/2.75)*t+.984375},k=function(t){var n=1.70158;return t*t*((n+1)*t-n)},P=function(t){var n=1.70158;return(t-=1)*t*((n+1)*t+n)+1},x=function(t){var n=1.70158;return(t/=.5)<1?t*t*((1+(n*=1.525))*t-n)*.5:.5*((t-=2)*t*((1+(n*=1.525))*t+n)+2)},T=function(t){return-1*Math.pow(4,-8*t)*Math.sin((6*t-1)*(2*Math.PI)/2)+1},E=function(t){var n=1.70158;return(t/=.5)<1?t*t*((1+(n*=1.525))*t-n)*.5:.5*((t-=2)*t*((1+(n*=1.525))*t+n)+2)},F=function(t){var n=1.70158;return t*t*((n+1)*t-n)},A=function(t){var n=1.70158;return(t-=1)*t*((n+1)*t+n)+1},I=function(t){return t<1/2.75?7.5625*t*t:t<2/2.75?7.5625*(t-=1.5/2.75)*t+.75:t<2.5/2.75?7.5625*(t-=2.25/2.75)*t+.9375:7.5625*(t-=2.625/2.75)*t+.984375},C=function(t){return t<1/2.75?7.5625*t*t:t<2/2.75?2-(7.5625*(t-=1.5/2.75)*t+.75):t<2.5/2.75?2-(7.5625*(t-=2.25/2.75)*t+.9375):2-(7.5625*(t-=2.625/2.75)*t+.984375)},D=function(t){return(t/=.5)<1?.5*Math.pow(t,4):-.5*((t-=2)*Math.pow(t,3)-2)},q=function(t){return Math.pow(t,4)},Q=function(t){return Math.pow(t,.25)}},function(t,n){var e;e=function(){return this}();try{e=e||new Function("return this")()}catch(t){"object"==typeof window&&(e=window)}t.exports=e},function(t,n,e){"use strict";e.r(n);var r={};e.r(r),e.d(r,"doesApply",(function(){return x})),e.d(r,"tweenCreated",(function(){return T})),e.d(r,"beforeTween",(function(){return E})),e.d(r,"afterTween",(function(){return F}));var i,u,o=e(0),a=/(\d|-|\.)/,c=/([^\-0-9.]+)/g,f=/[0-9.-]+/g,s=(i=f.source,u=/,\s*/.source,new RegExp("rgb\\(".concat(i).concat(u).concat(i).concat(u).concat(i,"\\)"),"g")),l=/^.*\(/,h=/#([0-9]|[a-f]){3,6}/gi,p=function(t,n){return t.map((function(t,e){return"_".concat(n,"_").concat(e)}))};function d(t){return parseInt(t,16)}var v=function(t){return"rgb(".concat((n=t,3===(n=n.replace(/#/,"")).length&&(n=(n=n.split(""))[0]+n[0]+n[1]+n[1]+n[2]+n[2]),[d(n.substr(0,2)),d(n.substr(2,2)),d(n.substr(4,2))]).join(","),")");var n},y=function(t,n,e){var r=n.match(t),i=n.replace(t,"VAL");return r&&r.forEach((function(t){return i=i.replace("VAL",e(t))})),i},_=function(t){for(var n in t){var e=t[n];"string"==typeof e&&e.match(h)&&(t[n]=y(h,e,v))}},m=function(t){var n=t.match(f).map(Math.floor),e=t.match(l)[0];return"".concat(e).concat(n.join(","),")")},g=function(t){return t.match(f)},b=function(t){var n,e,r={};for(var i in t){var u=t[i];"string"==typeof u&&(r[i]={formatString:(n=u,e=void 0,e=n.match(c),e?(1===e.length||n.charAt(0).match(a))&&e.unshift(""):e=["",""],e.join("VAL")),chunkNames:p(g(u),i)})}return r},w=function(t,n){var e=function(e){g(t[e]).forEach((function(r,i){return t[n[e].chunkNames[i]]=+r})),delete t[e]};for(var r in n)e(r)},O=function(t,n){var e={};return n.forEach((function(n){e[n]=t[n],delete t[n]})),e},S=function(t,n){return n.map((function(n){return t[n]}))},j=function(t,n){return n.forEach((function(n){return t=t.replace("VAL",+n.toFixed(4))})),t},M=function(t,n){for(var e in n){var r=n[e],i=r.chunkNames,u=r.formatString,o=j(u,S(O(t,i),i));t[e]=y(s,o,m)}},k=function(t,n){var e=function(e){var r=n[e].chunkNames,i=t[e];if("string"==typeof i){var u=i.split(" "),o=u[u.length-1];r.forEach((function(n,e){return t[n]=u[e]||o}))}else r.forEach((function(n){return t[n]=i}));delete t[e]};for(var r in n)e(r)},P=function(t,n){for(var e in n){var r=n[e].chunkNames,i=t[r[0]];t[e]="string"==typeof i?r.map((function(n){var e=t[n];return delete t[n],e})).join(" "):i}},x=function(t){var n=t._currentState;return Object.keys(n).some((function(t){return"string"==typeof n[t]}))};function T(t){var n=t._currentState;[n,t._originalState,t._targetState].forEach(_),t._tokenData=b(n)}function E(t){var n=t._currentState,e=t._originalState,r=t._targetState,i=t._easing,u=t._tokenData;k(i,u),[n,e,r].forEach((function(t){return w(t,u)}))}function F(t){var n=t._currentState,e=t._originalState,r=t._targetState,i=t._easing,u=t._tokenData;[n,e,r].forEach((function(t){return M(t,u)})),P(i,u)}function A(t,n){var e=Object.keys(t);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(t);n&&(r=r.filter((function(n){return Object.getOwnPropertyDescriptor(t,n).enumerable}))),e.push.apply(e,r)}return e}function I(t){for(var n=1;n<arguments.length;n++){var e=null!=arguments[n]?arguments[n]:{};n%2?A(Object(e),!0).forEach((function(n){C(t,n,e[n])})):Object.getOwnPropertyDescriptors?Object.defineProperties(t,Object.getOwnPropertyDescriptors(e)):A(Object(e)).forEach((function(n){Object.defineProperty(t,n,Object.getOwnPropertyDescriptor(e,n))}))}return t}function C(t,n,e){return n in t?Object.defineProperty(t,n,{value:e,enumerable:!0,configurable:!0,writable:!0}):t[n]=e,t}var D=new o.a,q=o.a.filters,Q=function(t,n,e,r){var i=arguments.length>4&&void 0!==arguments[4]?arguments[4]:0,u=I({},t),a=Object(o.b)(t,r);for(var c in D._filters.length=0,D.set({}),D._currentState=u,D._originalState=t,D._targetState=n,D._easing=a,q)q[c].doesApply(D)&&D._filters.push(q[c]);D._applyFilter("tweenCreated"),D._applyFilter("beforeTween");var f=Object(o.e)(e,u,t,n,1,i,a);return D._applyFilter("afterTween"),f};function B(t){return function(t){if(Array.isArray(t)){for(var n=0,e=new Array(t.length);n<t.length;n++)e[n]=t[n];return e}}(t)||function(t){if(Symbol.iterator in Object(t)||"[object Arguments]"===Object.prototype.toString.call(t))return Array.from(t)}(t)||function(){throw new TypeError("Invalid attempt to spread non-iterable instance")}()}function N(t,n){for(var e=0;e<n.length;e++){var r=n[e];r.enumerable=r.enumerable||!1,r.configurable=!0,"value"in r&&(r.writable=!0),Object.defineProperty(t,r.key,r)}}function R(t,n){var e=n.get(t);if(!e)throw new TypeError("attempted to get private field on non-instance");return e.get?e.get.call(t):e.value}var z=function(){function t(){!function(t,n){if(!(t instanceof n))throw new TypeError("Cannot call a class as a function")}(this,t),L.set(this,{writable:!0,value:[]});for(var n=arguments.length,e=new Array(n),r=0;r<n;r++)e[r]=arguments[r];e.forEach(this.add.bind(this))}var n,e,r;return n=t,(e=[{key:"add",value:function(t){return R(this,L).push(t),t}},{key:"remove",value:function(t){var n=R(this,L).indexOf(t);return~n&&R(this,L).splice(n,1),t}},{key:"empty",value:function(){return this.tweenables.map(this.remove.bind(this))}},{key:"isPlaying",value:function(){return R(this,L).some((function(t){return t.isPlaying()}))}},{key:"play",value:function(){return R(this,L).forEach((function(t){return t.tween()})),this}},{key:"pause",value:function(){return R(this,L).forEach((function(t){return t.pause()})),this}},{key:"resume",value:function(){return R(this,L).forEach((function(t){return t.resume()})),this}},{key:"stop",value:function(t){return R(this,L).forEach((function(n){return n.stop(t)})),this}},{key:"tweenables",get:function(){return B(R(this,L))}},{key:"promises",get:function(){return R(this,L).map((function(t){return t._promise}))}}])&&N(n.prototype,e),r&&N(n,r),t}(),L=new WeakMap;function V(t,n,e,r,i,u){var o,a,c=0,f=0,s=0,l=0,h=0,p=0,d=function(t){return((c*t+f)*t+s)*t},v=function(t){return(3*c*t+2*f)*t+s},y=function(t){return t>=0?t:0-t};return c=1-(s=3*n)-(f=3*(r-n)-s),l=1-(p=3*e)-(h=3*(i-e)-p),o=t,a=function(t){return 1/(200*t)}(u),function(t){return((l*t+h)*t+p)*t}(function(t,n){var e,r,i,u,o,a;for(i=t,a=0;a<8;a++){if(u=d(i)-t,y(u)<n)return i;if(o=v(i),y(o)<1e-6)break;i-=u/o}if((i=t)<(e=0))return e;if(i>(r=1))return r;for(;e<r;){if(u=d(i),y(u-t)<n)return i;t>u?e=i:r=i,i=.5*(r-e)+e}return i}(o,a))}var W=function(t,n,e,r,i){var u=function(t,n,e,r){return function(i){return V(i,t,n,e,r,1)}}(n,e,r,i);return u.displayName=t,u.x1=n,u.y1=e,u.x2=r,u.y2=i,o.a.formulas[t]=u},G=function(t){return delete o.a.formulas[t]};e.d(n,"processTweens",(function(){return o.c})),e.d(n,"Tweenable",(function(){return o.a})),e.d(n,"tween",(function(){return o.d})),e.d(n,"interpolate",(function(){return Q})),e.d(n,"Scene",(function(){return z})),e.d(n,"setBezierFunction",(function(){return W})),e.d(n,"unsetBezierFunction",(function(){return G})),o.a.filters.token=r}])}));

},{}],2:[function(require,module,exports){
// Circle shaped progress bar

var Shape = require('./shape');
var utils = require('./utils');

var Circle = function Circle(container, options) {
    // Use two arcs to form a circle
    // See this answer http://stackoverflow.com/a/10477334/1446092
    this._pathTemplate =
        'M 50,50 m 0,-{radius}' +
        ' a {radius},{radius} 0 1 1 0,{2radius}' +
        ' a {radius},{radius} 0 1 1 0,-{2radius}';

    this.containerAspectRatio = 1;

    Shape.apply(this, arguments);
};

Circle.prototype = new Shape();
Circle.prototype.constructor = Circle;

Circle.prototype._pathString = function _pathString(opts) {
    var widthOfWider = opts.strokeWidth;
    if (opts.trailWidth && opts.trailWidth > opts.strokeWidth) {
        widthOfWider = opts.trailWidth;
    }

    var r = 50 - widthOfWider / 2;

    return utils.render(this._pathTemplate, {
        radius: r,
        '2radius': r * 2
    });
};

Circle.prototype._trailString = function _trailString(opts) {
    return this._pathString(opts);
};

module.exports = Circle;

},{"./shape":7,"./utils":9}],3:[function(require,module,exports){
// Line shaped progress bar

var Shape = require('./shape');
var utils = require('./utils');

var Line = function Line(container, options) {
    this._pathTemplate = options.vertical
        ? 'M {center},100 L {center},0'
        : 'M 0,{center} L 100,{center}';
    Shape.apply(this, arguments);
};

Line.prototype = new Shape();
Line.prototype.constructor = Line;

Line.prototype._initializeSvg = function _initializeSvg(svg, opts) {
    var viewBoxStr = opts.vertical
        ? '0 0 ' + opts.strokeWidth + ' 100'
        : '0 0 100 ' + opts.strokeWidth;
    svg.setAttribute('viewBox', viewBoxStr);
    svg.setAttribute('preserveAspectRatio', 'none');
};

Line.prototype._pathString = function _pathString(opts) {
    return utils.render(this._pathTemplate, {
        center: opts.strokeWidth / 2
    });
};

Line.prototype._trailString = function _trailString(opts) {
    return this._pathString(opts);
};

module.exports = Line;

},{"./shape":7,"./utils":9}],4:[function(require,module,exports){
module.exports = {
    // Higher level API, different shaped progress bars
    Line: require('./line'),
    Circle: require('./circle'),
    SemiCircle: require('./semicircle'),
    Square: require('./square'),

    // Lower level API to use any SVG path
    Path: require('./path'),

    // Base-class for creating new custom shapes
    // to be in line with the API of built-in shapes
    // Undocumented.
    Shape: require('./shape'),

    // Internal utils, undocumented.
    utils: require('./utils')
};

},{"./circle":2,"./line":3,"./path":5,"./semicircle":6,"./shape":7,"./square":8,"./utils":9}],5:[function(require,module,exports){
// Lower level API to animate any kind of svg path

var shifty = require('shifty');
var utils = require('./utils');

var Tweenable = shifty.Tweenable;

var EASING_ALIASES = {
    easeIn: 'easeInCubic',
    easeOut: 'easeOutCubic',
    easeInOut: 'easeInOutCubic'
};

var Path = function Path(path, opts) {
    // Throw a better error if not initialized with `new` keyword
    if (!(this instanceof Path)) {
        throw new Error('Constructor was called without new keyword');
    }

    // Default parameters for animation
    opts = utils.extend({
        delay: 0,
        duration: 800,
        easing: 'linear',
        from: {},
        to: {},
        step: function() {}
    }, opts);

    var element;
    if (utils.isString(path)) {
        element = document.querySelector(path);
    } else {
        element = path;
    }

    // Reveal .path as public attribute
    this.path = element;
    this._opts = opts;
    this._tweenable = null;

    // Set up the starting positions
    var length = this.path.getTotalLength();
    this.path.style.strokeDasharray = length + ' ' + length;
    this.set(0);
};

Path.prototype.value = function value() {
    var offset = this._getComputedDashOffset();
    var length = this.path.getTotalLength();

    var progress = 1 - offset / length;
    // Round number to prevent returning very small number like 1e-30, which
    // is practically 0
    return parseFloat(progress.toFixed(6), 10);
};

Path.prototype.set = function set(progress) {
    this.stop();

    this.path.style.strokeDashoffset = this._progressToOffset(progress);

    var step = this._opts.step;
    if (utils.isFunction(step)) {
        var easing = this._easing(this._opts.easing);
        var values = this._calculateTo(progress, easing);
        var reference = this._opts.shape || this;
        step(values, reference, this._opts.attachment);
    }
};

Path.prototype.stop = function stop() {
    this._stopTween();
    this.path.style.strokeDashoffset = this._getComputedDashOffset();
};

// Method introduced here:
// http://jakearchibald.com/2013/animated-line-drawing-svg/
Path.prototype.animate = function animate(progress, opts, cb) {
    opts = opts || {};

    if (utils.isFunction(opts)) {
        cb = opts;
        opts = {};
    }

    var passedOpts = utils.extend({}, opts);

    // Copy default opts to new object so defaults are not modified
    var defaultOpts = utils.extend({}, this._opts);
    opts = utils.extend(defaultOpts, opts);

    var shiftyEasing = this._easing(opts.easing);
    var values = this._resolveFromAndTo(progress, shiftyEasing, passedOpts);

    this.stop();

    // Trigger a layout so styles are calculated & the browser
    // picks up the starting position before animating
    this.path.getBoundingClientRect();

    var offset = this._getComputedDashOffset();
    var newOffset = this._progressToOffset(progress);

    var self = this;
    this._tweenable = new Tweenable();
    this._tweenable.tween({
        from: utils.extend({ offset: offset }, values.from),
        to: utils.extend({ offset: newOffset }, values.to),
        duration: opts.duration,
        delay: opts.delay,
        easing: shiftyEasing,
        step: function(state) {
            self.path.style.strokeDashoffset = state.offset;
            var reference = opts.shape || self;
            opts.step(state, reference, opts.attachment);
        }
    }).then(function(state) {
        if (utils.isFunction(cb)) {
            cb();
        }
    }).catch(function(err) {
        console.error('Error in tweening:', err);
        throw err;
    });
};

Path.prototype._getComputedDashOffset = function _getComputedDashOffset() {
    var computedStyle = window.getComputedStyle(this.path, null);
    return parseFloat(computedStyle.getPropertyValue('stroke-dashoffset'), 10);
};

Path.prototype._progressToOffset = function _progressToOffset(progress) {
    var length = this.path.getTotalLength();
    return length - progress * length;
};

// Resolves from and to values for animation.
Path.prototype._resolveFromAndTo = function _resolveFromAndTo(progress, easing, opts) {
    if (opts.from && opts.to) {
        return {
            from: opts.from,
            to: opts.to
        };
    }

    return {
        from: this._calculateFrom(easing),
        to: this._calculateTo(progress, easing)
    };
};

// Calculate `from` values from options passed at initialization
Path.prototype._calculateFrom = function _calculateFrom(easing) {
    return shifty.interpolate(this._opts.from, this._opts.to, this.value(), easing);
};

// Calculate `to` values from options passed at initialization
Path.prototype._calculateTo = function _calculateTo(progress, easing) {
    return shifty.interpolate(this._opts.from, this._opts.to, progress, easing);
};

Path.prototype._stopTween = function _stopTween() {
    if (this._tweenable !== null) {
        this._tweenable.stop(true);
        this._tweenable = null;
    }
};

Path.prototype._easing = function _easing(easing) {
    if (EASING_ALIASES.hasOwnProperty(easing)) {
        return EASING_ALIASES[easing];
    }

    return easing;
};

module.exports = Path;

},{"./utils":9,"shifty":1}],6:[function(require,module,exports){
// Semi-SemiCircle shaped progress bar

var Shape = require('./shape');
var Circle = require('./circle');
var utils = require('./utils');

var SemiCircle = function SemiCircle(container, options) {
    // Use one arc to form a SemiCircle
    // See this answer http://stackoverflow.com/a/10477334/1446092
    this._pathTemplate =
        'M 50,50 m -{radius},0' +
        ' a {radius},{radius} 0 1 1 {2radius},0';

    this.containerAspectRatio = 2;

    Shape.apply(this, arguments);
};

SemiCircle.prototype = new Shape();
SemiCircle.prototype.constructor = SemiCircle;

SemiCircle.prototype._initializeSvg = function _initializeSvg(svg, opts) {
    svg.setAttribute('viewBox', '0 0 100 50');
};

SemiCircle.prototype._initializeTextContainer = function _initializeTextContainer(
    opts,
    container,
    textContainer
) {
    if (opts.text.style) {
        // Reset top style
        textContainer.style.top = 'auto';
        textContainer.style.bottom = '0';

        if (opts.text.alignToBottom) {
            utils.setStyle(textContainer, 'transform', 'translate(-50%, 0)');
        } else {
            utils.setStyle(textContainer, 'transform', 'translate(-50%, 50%)');
        }
    }
};

// Share functionality with Circle, just have different path
SemiCircle.prototype._pathString = Circle.prototype._pathString;
SemiCircle.prototype._trailString = Circle.prototype._trailString;

module.exports = SemiCircle;

},{"./circle":2,"./shape":7,"./utils":9}],7:[function(require,module,exports){
// Base object for different progress bar shapes

var Path = require('./path');
var utils = require('./utils');

var DESTROYED_ERROR = 'Object is destroyed';

var Shape = function Shape(container, opts) {
    // Throw a better error if progress bars are not initialized with `new`
    // keyword
    if (!(this instanceof Shape)) {
        throw new Error('Constructor was called without new keyword');
    }

    // Prevent calling constructor without parameters so inheritance
    // works correctly. To understand, this is how Shape is inherited:
    //
    //   Line.prototype = new Shape();
    //
    // We just want to set the prototype for Line.
    if (arguments.length === 0) {
        return;
    }

    // Default parameters for progress bar creation
    this._opts = utils.extend({
        color: '#555',
        strokeWidth: 1.0,
        trailColor: null,
        trailWidth: null,
        fill: null,
        text: {
            style: {
                color: null,
                position: 'absolute',
                left: '50%',
                top: '50%',
                padding: 0,
                margin: 0,
                transform: {
                    prefix: true,
                    value: 'translate(-50%, -50%)'
                }
            },
            autoStyleContainer: true,
            alignToBottom: true,
            value: null,
            className: 'progressbar-text'
        },
        svgStyle: {
            display: 'block',
            width: '100%'
        },
        warnings: false
    }, opts, true);  // Use recursive extend

    // If user specifies e.g. svgStyle or text style, the whole object
    // should replace the defaults to make working with styles easier
    if (utils.isObject(opts) && opts.svgStyle !== undefined) {
        this._opts.svgStyle = opts.svgStyle;
    }
    if (utils.isObject(opts) && utils.isObject(opts.text) && opts.text.style !== undefined) {
        this._opts.text.style = opts.text.style;
    }

    var svgView = this._createSvgView(this._opts);

    var element;
    if (utils.isString(container)) {
        element = document.querySelector(container);
    } else {
        element = container;
    }

    if (!element) {
        throw new Error('Container does not exist: ' + container);
    }

    this._container = element;
    this._container.appendChild(svgView.svg);
    if (this._opts.warnings) {
        this._warnContainerAspectRatio(this._container);
    }

    if (this._opts.svgStyle) {
        utils.setStyles(svgView.svg, this._opts.svgStyle);
    }

    // Expose public attributes before Path initialization
    this.svg = svgView.svg;
    this.path = svgView.path;
    this.trail = svgView.trail;
    this.text = null;

    var newOpts = utils.extend({
        attachment: undefined,
        shape: this
    }, this._opts);
    this._progressPath = new Path(svgView.path, newOpts);

    if (utils.isObject(this._opts.text) && this._opts.text.value !== null) {
        this.setText(this._opts.text.value);
    }
};

Shape.prototype.animate = function animate(progress, opts, cb) {
    if (this._progressPath === null) {
        throw new Error(DESTROYED_ERROR);
    }

    this._progressPath.animate(progress, opts, cb);
};

Shape.prototype.stop = function stop() {
    if (this._progressPath === null) {
        throw new Error(DESTROYED_ERROR);
    }

    // Don't crash if stop is called inside step function
    if (this._progressPath === undefined) {
        return;
    }

    this._progressPath.stop();
};

Shape.prototype.pause = function pause() {
    if (this._progressPath === null) {
        throw new Error(DESTROYED_ERROR);
    }

    if (this._progressPath === undefined) {
        return;
    }

    if (!this._progressPath._tweenable) {
        // It seems that we can't pause this
        return;
    }

    this._progressPath._tweenable.pause();
};

Shape.prototype.resume = function resume() {
    if (this._progressPath === null) {
        throw new Error(DESTROYED_ERROR);
    }

    if (this._progressPath === undefined) {
        return;
    }

    if (!this._progressPath._tweenable) {
        // It seems that we can't resume this
        return;
    }

    this._progressPath._tweenable.resume();
};

Shape.prototype.destroy = function destroy() {
    if (this._progressPath === null) {
        throw new Error(DESTROYED_ERROR);
    }

    this.stop();
    this.svg.parentNode.removeChild(this.svg);
    this.svg = null;
    this.path = null;
    this.trail = null;
    this._progressPath = null;

    if (this.text !== null) {
        this.text.parentNode.removeChild(this.text);
        this.text = null;
    }
};

Shape.prototype.set = function set(progress) {
    if (this._progressPath === null) {
        throw new Error(DESTROYED_ERROR);
    }

    this._progressPath.set(progress);
};

Shape.prototype.value = function value() {
    if (this._progressPath === null) {
        throw new Error(DESTROYED_ERROR);
    }

    if (this._progressPath === undefined) {
        return 0;
    }

    return this._progressPath.value();
};

Shape.prototype.setText = function setText(newText) {
    if (this._progressPath === null) {
        throw new Error(DESTROYED_ERROR);
    }

    if (this.text === null) {
        // Create new text node
        this.text = this._createTextContainer(this._opts, this._container);
        this._container.appendChild(this.text);
    }

    // Remove previous text and add new
    if (utils.isObject(newText)) {
        utils.removeChildren(this.text);
        this.text.appendChild(newText);
    } else {
        this.text.innerHTML = newText;
    }
};

Shape.prototype._createSvgView = function _createSvgView(opts) {
    var svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
    this._initializeSvg(svg, opts);

    var trailPath = null;
    // Each option listed in the if condition are 'triggers' for creating
    // the trail path
    if (opts.trailColor || opts.trailWidth) {
        trailPath = this._createTrail(opts);
        svg.appendChild(trailPath);
    }

    var path = this._createPath(opts);
    svg.appendChild(path);

    return {
        svg: svg,
        path: path,
        trail: trailPath
    };
};

Shape.prototype._initializeSvg = function _initializeSvg(svg, opts) {
    svg.setAttribute('viewBox', '0 0 100 100');
};

Shape.prototype._createPath = function _createPath(opts) {
    var pathString = this._pathString(opts);
    return this._createPathElement(pathString, opts);
};

Shape.prototype._createTrail = function _createTrail(opts) {
    // Create path string with original passed options
    var pathString = this._trailString(opts);

    // Prevent modifying original
    var newOpts = utils.extend({}, opts);

    // Defaults for parameters which modify trail path
    if (!newOpts.trailColor) {
        newOpts.trailColor = '#eee';
    }
    if (!newOpts.trailWidth) {
        newOpts.trailWidth = newOpts.strokeWidth;
    }

    newOpts.color = newOpts.trailColor;
    newOpts.strokeWidth = newOpts.trailWidth;

    // When trail path is set, fill must be set for it instead of the
    // actual path to prevent trail stroke from clipping
    newOpts.fill = null;

    return this._createPathElement(pathString, newOpts);
};

Shape.prototype._createPathElement = function _createPathElement(pathString, opts) {
    var path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
    path.setAttribute('d', pathString);
    path.setAttribute('stroke', opts.color);
    path.setAttribute('stroke-width', opts.strokeWidth);

    if (opts.fill) {
        path.setAttribute('fill', opts.fill);
    } else {
        path.setAttribute('fill-opacity', '0');
    }

    return path;
};

Shape.prototype._createTextContainer = function _createTextContainer(opts, container) {
    var textContainer = document.createElement('div');
    textContainer.className = opts.text.className;

    var textStyle = opts.text.style;
    if (textStyle) {
        if (opts.text.autoStyleContainer) {
            container.style.position = 'relative';
        }

        utils.setStyles(textContainer, textStyle);
        // Default text color to progress bar's color
        if (!textStyle.color) {
            textContainer.style.color = opts.color;
        }
    }

    this._initializeTextContainer(opts, container, textContainer);
    return textContainer;
};

// Give custom shapes possibility to modify text element
Shape.prototype._initializeTextContainer = function(opts, container, element) {
    // By default, no-op
    // Custom shapes should respect API options, such as text.style
};

Shape.prototype._pathString = function _pathString(opts) {
    throw new Error('Override this function for each progress bar');
};

Shape.prototype._trailString = function _trailString(opts) {
    throw new Error('Override this function for each progress bar');
};

Shape.prototype._warnContainerAspectRatio = function _warnContainerAspectRatio(container) {
    if (!this.containerAspectRatio) {
        return;
    }

    var computedStyle = window.getComputedStyle(container, null);
    var width = parseFloat(computedStyle.getPropertyValue('width'), 10);
    var height = parseFloat(computedStyle.getPropertyValue('height'), 10);
    if (!utils.floatEquals(this.containerAspectRatio, width / height)) {
        console.warn(
            'Incorrect aspect ratio of container',
            '#' + container.id,
            'detected:',
            computedStyle.getPropertyValue('width') + '(width)',
            '/',
            computedStyle.getPropertyValue('height') + '(height)',
            '=',
            width / height
        );

        console.warn(
            'Aspect ratio of should be',
            this.containerAspectRatio
        );
    }
};

module.exports = Shape;

},{"./path":5,"./utils":9}],8:[function(require,module,exports){
// Square shaped progress bar
// Note: Square is not core part of API anymore. It's left here
//       for reference. square is not included to the progressbar
//       build anymore

var Shape = require('./shape');
var utils = require('./utils');

var Square = function Square(container, options) {
    this._pathTemplate =
        'M 0,{halfOfStrokeWidth}' +
        ' L {width},{halfOfStrokeWidth}' +
        ' L {width},{width}' +
        ' L {halfOfStrokeWidth},{width}' +
        ' L {halfOfStrokeWidth},{strokeWidth}';

    this._trailTemplate =
        'M {startMargin},{halfOfStrokeWidth}' +
        ' L {width},{halfOfStrokeWidth}' +
        ' L {width},{width}' +
        ' L {halfOfStrokeWidth},{width}' +
        ' L {halfOfStrokeWidth},{halfOfStrokeWidth}';

    Shape.apply(this, arguments);
};

Square.prototype = new Shape();
Square.prototype.constructor = Square;

Square.prototype._pathString = function _pathString(opts) {
    var w = 100 - opts.strokeWidth / 2;

    return utils.render(this._pathTemplate, {
        width: w,
        strokeWidth: opts.strokeWidth,
        halfOfStrokeWidth: opts.strokeWidth / 2
    });
};

Square.prototype._trailString = function _trailString(opts) {
    var w = 100 - opts.strokeWidth / 2;

    return utils.render(this._trailTemplate, {
        width: w,
        strokeWidth: opts.strokeWidth,
        halfOfStrokeWidth: opts.strokeWidth / 2,
        startMargin: opts.strokeWidth / 2 - opts.trailWidth / 2
    });
};

module.exports = Square;

},{"./shape":7,"./utils":9}],9:[function(require,module,exports){
// Utility functions

var PREFIXES = 'Webkit Moz O ms'.split(' ');
var FLOAT_COMPARISON_EPSILON = 0.001;

// Copy all attributes from source object to destination object.
// destination object is mutated.
function extend(destination, source, recursive) {
    destination = destination || {};
    source = source || {};
    recursive = recursive || false;

    for (var attrName in source) {
        if (source.hasOwnProperty(attrName)) {
            var destVal = destination[attrName];
            var sourceVal = source[attrName];
            if (recursive && isObject(destVal) && isObject(sourceVal)) {
                destination[attrName] = extend(destVal, sourceVal, recursive);
            } else {
                destination[attrName] = sourceVal;
            }
        }
    }

    return destination;
}

// Renders templates with given variables. Variables must be surrounded with
// braces without any spaces, e.g. {variable}
// All instances of variable placeholders will be replaced with given content
// Example:
// render('Hello, {message}!', {message: 'world'})
function render(template, vars) {
    var rendered = template;

    for (var key in vars) {
        if (vars.hasOwnProperty(key)) {
            var val = vars[key];
            var regExpString = '\\{' + key + '\\}';
            var regExp = new RegExp(regExpString, 'g');

            rendered = rendered.replace(regExp, val);
        }
    }

    return rendered;
}

function setStyle(element, style, value) {
    var elStyle = element.style;  // cache for performance

    for (var i = 0; i < PREFIXES.length; ++i) {
        var prefix = PREFIXES[i];
        elStyle[prefix + capitalize(style)] = value;
    }

    elStyle[style] = value;
}

function setStyles(element, styles) {
    forEachObject(styles, function(styleValue, styleName) {
        // Allow disabling some individual styles by setting them
        // to null or undefined
        if (styleValue === null || styleValue === undefined) {
            return;
        }

        // If style's value is {prefix: true, value: '50%'},
        // Set also browser prefixed styles
        if (isObject(styleValue) && styleValue.prefix === true) {
            setStyle(element, styleName, styleValue.value);
        } else {
            element.style[styleName] = styleValue;
        }
    });
}

function capitalize(text) {
    return text.charAt(0).toUpperCase() + text.slice(1);
}

function isString(obj) {
    return typeof obj === 'string' || obj instanceof String;
}

function isFunction(obj) {
    return typeof obj === 'function';
}

function isArray(obj) {
    return Object.prototype.toString.call(obj) === '[object Array]';
}

// Returns true if `obj` is object as in {a: 1, b: 2}, not if it's function or
// array
function isObject(obj) {
    if (isArray(obj)) {
        return false;
    }

    var type = typeof obj;
    return type === 'object' && !!obj;
}

function forEachObject(object, callback) {
    for (var key in object) {
        if (object.hasOwnProperty(key)) {
            var val = object[key];
            callback(val, key);
        }
    }
}

function floatEquals(a, b) {
    return Math.abs(a - b) < FLOAT_COMPARISON_EPSILON;
}

// https://coderwall.com/p/nygghw/don-t-use-innerhtml-to-empty-dom-elements
function removeChildren(el) {
    while (el.firstChild) {
        el.removeChild(el.firstChild);
    }
}

module.exports = {
    extend: extend,
    render: render,
    setStyle: setStyle,
    setStyles: setStyles,
    capitalize: capitalize,
    isString: isString,
    isFunction: isFunction,
    isObject: isObject,
    forEachObject: forEachObject,
    floatEquals: floatEquals,
    removeChildren: removeChildren
};

},{}]},{},[4])(4)
});


export const {ProgressBar} = root;
"###;


        use std::collections::HashMap;
        // use std::sync::Arc;
        use std::sync::Mutex;

        pub type ModuleId = u64;
        pub struct Module {
            url : Mutex<Option<String>>,
            ident : &'static str,
            content: &'static str,
            imports: &'static [(&'static str, ModuleId)],
            exports: &'static [(&'static str, ModuleId)],
        }

        impl Module {

            pub fn content(&self, modules: &ModuleMap) -> String {
                let text = String::new();

                let imports = self.imports.iter().map(|(what, id)| {
                    let url = modules
                        .get(id)
                        .export(&format("unable to lookup module `{}`",self.ident))
                        .url.as_str();
                    format!("import {} from \"{}\";\n", what, url)
                }).collect::<Vec<_>>().join("\n");

                let exports = self.exports.iter().map(|(what, id)| {
                    let url = modules
                        .get(id)
                        .export(&format("unable to lookup module `{}`",self.ident))
                        .url.as_str();
                    format!("export {} from \"{}\";\n", what, url)
                }).collect::<Vec<_>>().join("\n");

                text += &imports;
                text += &self.content;
                text += &exports;

                text
            }

            pub async fn load(&self, modules: &ModuleMap) {

                let content = self.content(modules);
                let args = Array::new_with_length(1);
                args.set(0, unsafe { Uint8Array::view(content.as_bytes()).into() });
                let mut options = web_sys::BlobPropertyBag::new();
                options.type_("application/javascript");
                let blob = Blob::new_with_u8_array_sequence_and_options(&args, &options)?;
                let url = Url::create_object_url_with_blob(&blob)?;
                self.url.lock().unwrap().replace(url.clone());

                // TODO load
            }
        }

        pub type ModuleMap = HashMap<ModuleId, &Module>;

        pub async fn load() -> Result<Context> {

            // let modules : HashMap<ModuleId, &Module> = MODULES.into_iter().collect();
            let modules : ModuleMap = MODULES.into_iter().collect();

            for ids in DEPENDENCIES.iter() {
                let futures = ids
                    .iter()
                    .map(|id| {
                        let module = modules.get(id).unwrap();
                        module.load(&modules);
                    })
                    .collect::<Vec<_>>();
                
                for future in futures {
                    future.await?;
                }
            }

        }
        
			const MODULE_434: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_flow_ux_434",
				content : ASPECTRON_FLOW_UX_FLOW_UX_434,
				imports : &[],
				exports : &[("*",414),("*",362),("*",396),("*",399),("*",425),("*",364),("*",424),("*",394),("*",356),("*",373),("*",372),("*",412),("*",397),("*",429),("*",433),("*",347),("*",421),("*",393),("*",354),("*",363),("*",376),("*",390),("*",428),("*",375),("*",377),("*",384),("*",420),("*",380),("*",403),("*",405),("*",349),("*",418),("*",386),("*",366),("*",360),("*",416),("*",379),("*",408),("*",423),("*",350),("*",432),("*",391),("*",351),("*",401),("*",410),("*",417),("*",400),("*",359),("*",381),("*",368),("*",352),("*",374),("*",413),("*",387),("*",419),("*",395),("*",378),("*",353),("*",404),("*",369),("*",383),("*",426),("*",370),("*",355),("*",385),("*",415),("*",430),("*",382),("*",427),("*",388),("*",371),("*",402),("*",407)],
			};

			const MODULE_431: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_terminal_431",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_TERMINAL_431,
				imports : &[("{BaseElement, html, css, ScrollbarStyle}",410)],
				exports : &[],
			};

			const MODULE_414: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_svg_414",
				content : ASPECTRON_FLOW_UX_SRC_SVG_414,
				imports : &[],
				exports : &[],
			};

			const MODULE_362: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_anchor_362",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_ANCHOR_362,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_396: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_async_396",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_ASYNC_396,
				imports : &[("{ dpc, UID }",409)],
				exports : &[],
			};

			const MODULE_399: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_caption_bar_399",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_CAPTION_BAR_399,
				imports : &[("{BaseElement, html, css}",410),("{FlowI18nDialog}",379)],
				exports : &[],
			};

			const MODULE_425: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_checkbox_styled_425",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_CHECKBOX_STYLED_425,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_364: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_checkbox_364",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_CHECKBOX_364,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_424: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_radio_424",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_RADIO_424,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_394: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_radio_btn_394",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_RADIO_BTN_394,
				imports : &[("{FlowToggleBtn}",372)],
				exports : &[],
			};

			const MODULE_356: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_radio_btns_356",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_RADIO_BTNS_356,
				imports : &[("{ FlowMenu }",374)],
				exports : &[],
			};

			const MODULE_373: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_radios_373",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_RADIOS_373,
				imports : &[("{ FlowMenu }",374)],
				exports : &[],
			};

			const MODULE_372: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_toggle_btn_372",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_TOGGLE_BTN_372,
				imports : &[("{css}",410),("{FlowBtn}",380)],
				exports : &[],
			};

			const MODULE_412: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_btn_group_412",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_BTN_GROUP_412,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_397: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_circular_shapes_397",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_CIRCULAR_SHAPES_397,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_429: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_form_region_429",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_FORM_REGION_429,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_433: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_svg_test_433",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_SVG_TEST_433,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_347: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_tab_347",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_TAB_347,
				imports : &[("{BaseElement, html, css, svg/*, parts*/}",410)],
				exports : &[],
			};

			const MODULE_421: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_tabs_421",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_TABS_421,
				imports : &[("{BaseElement, html, css}",410),("{repeat}",505),("{i18n, T}",379)],
				exports : &[],
			};

			const MODULE_393: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_test_element_393",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_TEST_ELEMENT_393,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_354: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_text_area_354",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_TEXT_AREA_354,
				imports : &[("{BaseElement, html, css}",410),("{ifDefined}",501),("{T}",379)],
				exports : &[],
			};

			const MODULE_363: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_form_control_363",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_FORM_CONTROL_363,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_376: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_form_mixin_376",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_FORM_MIXIN_376,
				imports : &[],
				exports : &[],
			};

			const MODULE_390: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_folder_input_390",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_FOLDER_INPUT_390,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_428: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_input_428",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_INPUT_428,
				imports : &[("{BaseElement, html, css}",410),("{ifDefined}",501)],
				exports : &[],
			};

			const MODULE_375: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_core_input_375",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_CORE_INPUT_375,
				imports : &[],
				exports : &[],
			};

			const MODULE_377: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_expandable_377",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_EXPANDABLE_377,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_384: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_qrcode_384",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_QRCODE_384,
				imports : &[("{BaseElement, svg, html, css, baseUrl}",410),("{ dpc }",409)],
				exports : &[],
			};

			const MODULE_420: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_qrcode_scanner_420",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_QRCODE_SCANNER_420,
				imports : &[("{BaseElement, html, css, baseUrl}",410),("{ dpc, contain}",409),("{jsQR}",265),("{T}",379)],
				exports : &[],
			};

			const MODULE_380: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_btn_380",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_BTN_380,
				imports : &[("{BaseElement, html, css}",410),("{i18nElementsMap, i18n}",379)],
				exports : &[],
			};

			const MODULE_403: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_group_btns_403",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_GROUP_BTNS_403,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_405: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_shell_link_405",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_SHELL_LINK_405,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_349: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_window_link_349",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_WINDOW_LINK_349,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_418: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_fa_icon_418",
				content : ASPECTRON_FLOW_UX_SRC_FA_ICON_418,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_386: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_data_badge_386",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_DATA_BADGE_386,
				imports : &[("{BaseElement, html, css, dpc}",410),("{Flowd3Element}",391),("{FlowSampler}",401)],
				exports : &[],
			};

			const MODULE_366: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_data_badge_graph_366",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_DATA_BADGE_GRAPH_366,
				imports : &[("{BaseElement, html, css, dpc}",410),("{Flowd3Element}",391),("{FlowSampler}",401)],
				exports : &[],
			};

			const MODULE_360: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_dialog_360",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_DIALOG_360,
				imports : &[("{BaseElement, html, css, baseUrl}",410)],
				exports : &[],
			};

			const MODULE_416: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_pages_416",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_PAGES_416,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_379: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_i18n_379",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_I18N_379,
				imports : &[("{BaseElement, html, css, /*directive,*/ LitElement/*, parts as _parts*/}",410),("{directive, AsyncDirective}",410)],
				exports : &[],
			};

			const MODULE_408: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_code_408",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_CODE_408,
				imports : &[("{ BaseElement, html, css, baseUrl }",410)],
				exports : &[],
			};

			const MODULE_423: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_color_selector_423",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_COLOR_SELECTOR_423,
				imports : &[("{BaseElement, html, css}",410),("{FlowCanvasElement}",432)],
				exports : &[],
			};

			const MODULE_350: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_reference_350",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_REFERENCE_350,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_432: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_canvas_432",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_CANVAS_432,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_391: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_d3_391",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_D3_391,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_351: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_graph_351",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_GRAPH_351,
				imports : &[("{BaseElement, html, css, flow, dpc}",410),("{Flowd3Element}",391),("{FlowSampler}",401),("{FlowFormat}",378)],
				exports : &[],
			};

			const MODULE_401: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_sampler_401",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_SAMPLER_401,
				imports : &[("{flow, dpc}",410)],
				exports : &[],
			};

			const MODULE_410: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_base_element_410",
				content : ASPECTRON_FLOW_UX_SRC_BASE_ELEMENT_410,
				imports : &[("{LitElement, html, css}",673),("{ AsyncQueueSubscriber }",396),("{ 	baseUrl, debug, FlowIconPath, FlowIcons, resolveIcon, 	FlowStates, DeferComponent, flow }",409),("{styleAppendTo, sizeClsMap}",409)],
				exports : &[("*",611),("*",550),("*",673),("*",613),("*",409),("*",389),("*",398)],
			};

			const MODULE_417: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_socket_417",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_SOCKET_417,
				imports : &[("{FlowEvents}",367),("{dpc, UID}",409)],
				exports : &[("*",367)],
			};

			const MODULE_400: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_socket_rpc_400",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_SOCKET_RPC_400,
				imports : &[("{FlowSocket}",417),("{dpc, UID}",409),("{AsyncQueueSubscriberMap}",396)],
				exports : &[],
			};

			const MODULE_359: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_socket_nats_359",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_SOCKET_NATS_359,
				imports : &[("{FlowSocket}",417),("{dpc, UID}",409),("{AsyncQueueSubscriberMap}",396)],
				exports : &[],
			};

			const MODULE_381: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_app_381",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_APP_381,
				imports : &[("{dpc, UID, setTheme, getTheme, flow}",409),("{FlowSocketRPC}",400),("{FlowSocketNATS}",359),("{BaseElement, ScrollbarStyle, html, css}",410)],
				exports : &[],
			};

			const MODULE_368: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_app_drawer_368",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_APP_DRAWER_368,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_352: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_dropdown_352",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_DROPDOWN_352,
				imports : &[("{ 	BaseElement, html, css, dpc, 	ScrollbarStyle, 	isSmallScreen }",410)],
				exports : &[],
			};

			const MODULE_374: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_menu_374",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_MENU_374,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_413: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_select_413",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_SELECT_413,
				imports : &[("{BaseElement, html, css, dpc}",410),("{FlowMenu}",374)],
				exports : &[],
			};

			const MODULE_387: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_table_mixin_387",
				content : ASPECTRON_FLOW_UX_SRC_TABLE_MIXIN_387,
				imports : &[("{buildPagination}",398)],
				exports : &[],
			};

			const MODULE_419: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_link_419",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_LINK_419,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_395: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_selector_395",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_SELECTOR_395,
				imports : &[("{FlowSelect}",413),("{css}",410)],
				exports : &[],
			};

			const MODULE_378: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_format_378",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_FORMAT_378,
				imports : &[("{flow, dpc}",410)],
				exports : &[],
			};

			const MODULE_353: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_clock_widget_353",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_CLOCK_WIDGET_353,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_404: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_theme_select_404",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_THEME_SELECT_404,
				imports : &[("{html, css, dpc}",410),("{FlowSelect}",413),("{setTheme, getTheme}",409)],
				exports : &[],
			};

			const MODULE_369: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_dropzone_field_369",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_DROPZONE_FIELD_369,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_383: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_markdown_383",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_MARKDOWN_383,
				imports : &[("{BaseElement, html, css, baseUrl, dpc}",410),("{ FlowCode }",408)],
				exports : &[],
			};

			const MODULE_426: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_toolbar_426",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_TOOLBAR_426,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[("*",406)],
			};

			const MODULE_370: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_gridstack_370",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_GRIDSTACK_370,
				imports : &[("{BaseElement, html, css, baseUrl, dpc, getLocalSetting, setLocalSetting}",410)],
				exports : &[("*",361),("*",348),("*",422)],
			};

			const MODULE_355: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_download_badge_355",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_DOWNLOAD_BADGE_355,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_385: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_color_picker_385",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_COLOR_PICKER_385,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_415: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_rss_415",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_RSS_415,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_430: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_sunburst_graph_430",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_SUNBURST_GRAPH_430,
				imports : &[("{BaseElement, ScrollbarStyle, html, css, flow, dpc, render}",410),("{Flowd3Element}",391),("{FlowSampler}",401),("{FlowFormat}",378)],
				exports : &[],
			};

			const MODULE_382: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_swipeable_382",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_SWIPEABLE_382,
				imports : &[("{css}",410)],
				exports : &[],
			};

			const MODULE_427: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_t9_427",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_T9_427,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_388: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_progressbar_388",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_PROGRESSBAR_388,
				imports : &[("{BaseElement, html, css}",410),("{ProgressBar}",341),("{colorMixer}",402)],
				exports : &[],
			};

			const MODULE_371: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_add_to_home_371",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_ADD_TO_HOME_371,
				imports : &[("{ 	BaseElement, html, css, getLocalSetting, setLocalSetting }",410)],
				exports : &[],
			};

			const MODULE_402: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_colors_402",
				content : ASPECTRON_FLOW_UX_SRC_COLORS_402,
				imports : &[("{dpc}",409)],
				exports : &[],
			};

			const MODULE_407: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_statsd_407",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_STATSD_407,
				imports : &[("{BaseElement, svg, html, css, baseUrl}",410),("{ dpc }",409)],
				exports : &[],
			};

			const MODULE_409: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_helpers_409",
				content : ASPECTRON_FLOW_UX_SRC_HELPERS_409,
				imports : &[],
				exports : &[],
			};

			const MODULE_505: &'static Module = Module {
				url : Mutex::new(None),
				ident : "lit_html_directives_repeat_505",
				content : LIT_HTML_DIRECTIVES_REPEAT_505,
				imports : &[("{noChange as e}",613),("{directive as s,Directive as t,PartType as r}",611),("{getCommittedValue as l,setChildPartValue as o,insertPart as i,removePart as n,setCommittedValue as f}",546)],
				exports : &[],
			};

			const MODULE_613: &'static Module = Module {
				url : Mutex::new(None),
				ident : "lit_html_lit_html_613",
				content : LIT_HTML_LIT_HTML_613,
				imports : &[],
				exports : &[],
			};

			const MODULE_611: &'static Module = Module {
				url : Mutex::new(None),
				ident : "lit_html_directive_611",
				content : LIT_HTML_DIRECTIVE_611,
				imports : &[],
				exports : &[],
			};

			const MODULE_546: &'static Module = Module {
				url : Mutex::new(None),
				ident : "lit_html_directive_helpers_546",
				content : LIT_HTML_DIRECTIVE_HELPERS_546,
				imports : &[("{_$LH as o}",613)],
				exports : &[],
			};

			const MODULE_501: &'static Module = Module {
				url : Mutex::new(None),
				ident : "lit_html_directives_if_defined_501",
				content : LIT_HTML_DIRECTIVES_IF_DEFINED_501,
				imports : &[("{nothing as t}",613)],
				exports : &[],
			};

			const MODULE_265: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_resources_externqrqr_265",
				content : ASPECTRON_FLOW_UX_RESOURCES_EXTERNQRQR_265,
				imports : &[],
				exports : &[],
			};

			const MODULE_673: &'static Module = Module {
				url : Mutex::new(None),
				ident : "lit_element_lit_element_673",
				content : LIT_ELEMENT_LIT_ELEMENT_673,
				imports : &[("{ReactiveElement as t}",89),("{render as e,noChange as i}",613)],
				exports : &[("*",89),("*",613)],
			};

			const MODULE_550: &'static Module = Module {
				url : Mutex::new(None),
				ident : "lit_html_async_directive_550",
				content : LIT_HTML_ASYNC_DIRECTIVE_550,
				imports : &[("{isSingleExpression as i}",546),("{Directive as t,PartType as e}",611)],
				exports : &[("{Directive,PartType,directive}",611)],
			};

			const MODULE_389: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_html_389",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_HTML_389,
				imports : &[("{html}",674)],
				exports : &[],
			};

			const MODULE_398: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_pagination_398",
				content : ASPECTRON_FLOW_UX_SRC_PAGINATION_398,
				imports : &[("{html, css}",674),("{repeat}",505),("{isSmallScreen}",409)],
				exports : &[],
			};

			const MODULE_89: &'static Module = Module {
				url : Mutex::new(None),
				ident : "lit_reactive_element_reactive_element_89",
				content : LIT_REACTIVE_ELEMENT_REACTIVE_ELEMENT_89,
				imports : &[("{getCompatibleStyle as t,adoptStyles as i}",31)],
				exports : &[("{CSSResult,adoptStyles,css,getCompatibleStyle,supportsAdoptingStyleSheets,unsafeCSS}",31)],
			};

			const MODULE_31: &'static Module = Module {
				url : Mutex::new(None),
				ident : "lit_reactive_element_css_tag_31",
				content : LIT_REACTIVE_ELEMENT_CSS_TAG_31,
				imports : &[],
				exports : &[],
			};

			const MODULE_674: &'static Module = Module {
				url : Mutex::new(None),
				ident : "lit_element_index_674",
				content : LIT_ELEMENT_INDEX_674,
				imports : &[],
				exports : &[("*",89),("*",613),("{LitElement,UpdatingElement,_$LE}",673),("*",38),("*",40),("*",33),("*",37),("*",32),("*",34),("*",36),("*",35),("*",39),("*",41)],
			};

			const MODULE_38: &'static Module = Module {
				url : Mutex::new(None),
				ident : "lit_reactive_element_decorators_base_38",
				content : LIT_REACTIVE_ELEMENT_DECORATORS_BASE_38,
				imports : &[],
				exports : &[],
			};

			const MODULE_40: &'static Module = Module {
				url : Mutex::new(None),
				ident : "lit_reactive_element_decorators_custom_element_40",
				content : LIT_REACTIVE_ELEMENT_DECORATORS_CUSTOM_ELEMENT_40,
				imports : &[],
				exports : &[],
			};

			const MODULE_33: &'static Module = Module {
				url : Mutex::new(None),
				ident : "lit_reactive_element_decorators_property_33",
				content : LIT_REACTIVE_ELEMENT_DECORATORS_PROPERTY_33,
				imports : &[],
				exports : &[],
			};

			const MODULE_37: &'static Module = Module {
				url : Mutex::new(None),
				ident : "lit_reactive_element_decorators_state_37",
				content : LIT_REACTIVE_ELEMENT_DECORATORS_STATE_37,
				imports : &[("{property as r}",33)],
				exports : &[],
			};

			const MODULE_32: &'static Module = Module {
				url : Mutex::new(None),
				ident : "lit_reactive_element_decorators_event_options_32",
				content : LIT_REACTIVE_ELEMENT_DECORATORS_EVENT_OPTIONS_32,
				imports : &[("{decorateProperty as r}",38)],
				exports : &[],
			};

			const MODULE_34: &'static Module = Module {
				url : Mutex::new(None),
				ident : "lit_reactive_element_decorators_query_34",
				content : LIT_REACTIVE_ELEMENT_DECORATORS_QUERY_34,
				imports : &[("{decorateProperty as o}",38)],
				exports : &[],
			};

			const MODULE_36: &'static Module = Module {
				url : Mutex::new(None),
				ident : "lit_reactive_element_decorators_query_all_36",
				content : LIT_REACTIVE_ELEMENT_DECORATORS_QUERY_ALL_36,
				imports : &[("{decorateProperty as r}",38)],
				exports : &[],
			};

			const MODULE_35: &'static Module = Module {
				url : Mutex::new(None),
				ident : "lit_reactive_element_decorators_query_async_35",
				content : LIT_REACTIVE_ELEMENT_DECORATORS_QUERY_ASYNC_35,
				imports : &[("{decorateProperty as r}",38)],
				exports : &[],
			};

			const MODULE_39: &'static Module = Module {
				url : Mutex::new(None),
				ident : "lit_reactive_element_decorators_query_assigned_elements_39",
				content : LIT_REACTIVE_ELEMENT_DECORATORS_QUERY_ASSIGNED_ELEMENTS_39,
				imports : &[("{decorateProperty as o}",38)],
				exports : &[],
			};

			const MODULE_41: &'static Module = Module {
				url : Mutex::new(None),
				ident : "lit_reactive_element_decorators_query_assigned_nodes_41",
				content : LIT_REACTIVE_ELEMENT_DECORATORS_QUERY_ASSIGNED_NODES_41,
				imports : &[("{decorateProperty as e}",38),("{queryAssignedElements as t}",39)],
				exports : &[],
			};

			const MODULE_367: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_events_367",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_EVENTS_367,
				imports : &[("{dpc, UID}",409)],
				exports : &[],
			};

			const MODULE_406: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_toolbar_item_406",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_TOOLBAR_ITEM_406,
				imports : &[("{BaseElement, html, css}",410)],
				exports : &[],
			};

			const MODULE_361: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_gridstack_panel_361",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_GRIDSTACK_PANEL_361,
				imports : &[("{BaseElement, html, css}",410),("{FlowContextListenerMixin}",422)],
				exports : &[],
			};

			const MODULE_348: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_context_test_348",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_CONTEXT_TEST_348,
				imports : &[("{BaseElement, html, css}",410),("{FlowContextWorkspace, FlowContext}",422),("{FlowContextElement}",422)],
				exports : &[],
			};

			const MODULE_422: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_src_flow_context_422",
				content : ASPECTRON_FLOW_UX_SRC_FLOW_CONTEXT_422,
				imports : &[("{BaseElement, html, flowHtml, css, ScrollbarStyle, deepClone, render}",410),("{baseUrl, dpc, utils, trigger}",410)],
				exports : &[],
			};

			const MODULE_341: &'static Module = Module {
				url : Mutex::new(None),
				ident : "aspectron_flow_ux_resources_extern_progressbar_progressbar_341",
				content : ASPECTRON_FLOW_UX_RESOURCES_EXTERN_PROGRESSBAR_PROGRESSBAR_341,
				imports : &[],
				exports : &[],
			};
const MODULES : &[(ModuleId,&'static Module)] = &[
(434,&MODULE_434),
(431,&MODULE_431),
(414,&MODULE_414),
(362,&MODULE_362),
(396,&MODULE_396),
(399,&MODULE_399),
(425,&MODULE_425),
(364,&MODULE_364),
(424,&MODULE_424),
(394,&MODULE_394),
(356,&MODULE_356),
(373,&MODULE_373),
(372,&MODULE_372),
(412,&MODULE_412),
(397,&MODULE_397),
(429,&MODULE_429),
(433,&MODULE_433),
(347,&MODULE_347),
(421,&MODULE_421),
(393,&MODULE_393),
(354,&MODULE_354),
(363,&MODULE_363),
(376,&MODULE_376),
(390,&MODULE_390),
(428,&MODULE_428),
(375,&MODULE_375),
(377,&MODULE_377),
(384,&MODULE_384),
(420,&MODULE_420),
(380,&MODULE_380),
(403,&MODULE_403),
(405,&MODULE_405),
(349,&MODULE_349),
(418,&MODULE_418),
(386,&MODULE_386),
(366,&MODULE_366),
(360,&MODULE_360),
(416,&MODULE_416),
(379,&MODULE_379),
(408,&MODULE_408),
(423,&MODULE_423),
(350,&MODULE_350),
(432,&MODULE_432),
(391,&MODULE_391),
(351,&MODULE_351),
(401,&MODULE_401),
(410,&MODULE_410),
(417,&MODULE_417),
(400,&MODULE_400),
(359,&MODULE_359),
(381,&MODULE_381),
(368,&MODULE_368),
(352,&MODULE_352),
(374,&MODULE_374),
(413,&MODULE_413),
(387,&MODULE_387),
(419,&MODULE_419),
(395,&MODULE_395),
(378,&MODULE_378),
(353,&MODULE_353),
(404,&MODULE_404),
(369,&MODULE_369),
(383,&MODULE_383),
(426,&MODULE_426),
(370,&MODULE_370),
(355,&MODULE_355),
(385,&MODULE_385),
(415,&MODULE_415),
(430,&MODULE_430),
(382,&MODULE_382),
(427,&MODULE_427),
(388,&MODULE_388),
(371,&MODULE_371),
(402,&MODULE_402),
(407,&MODULE_407),
(409,&MODULE_409),
(505,&MODULE_505),
(613,&MODULE_613),
(611,&MODULE_611),
(546,&MODULE_546),
(501,&MODULE_501),
(265,&MODULE_265),
(673,&MODULE_673),
(550,&MODULE_550),
(389,&MODULE_389),
(398,&MODULE_398),
(89,&MODULE_89),
(31,&MODULE_31),
(674,&MODULE_674),
(38,&MODULE_38),
(40,&MODULE_40),
(33,&MODULE_33),
(37,&MODULE_37),
(32,&MODULE_32),
(34,&MODULE_34),
(36,&MODULE_36),
(35,&MODULE_35),
(39,&MODULE_39),
(41,&MODULE_41),
(367,&MODULE_367),
(406,&MODULE_406),
(361,&MODULE_361),
(348,&MODULE_348),
(422,&MODULE_422),
(341,&MODULE_341)
];
// ASPECTRON_FLOW_UX_SRC_HELPERS
// LIT_HTML_LIT_HTML, LIT_HTML_DIRECTIVE, LIT_HTML_DIRECTIVE_HELPERS
// LIT_HTML_DIRECTIVES_REPEAT
// LIT_HTML_DIRECTIVES_IF_DEFINED
// ASPECTRON_FLOW_UX_RESOURCES_EXTERNQRQR
// LIT_REACTIVE_ELEMENT_CSS_TAG
// LIT_REACTIVE_ELEMENT_REACTIVE_ELEMENT
// LIT_REACTIVE_ELEMENT_DECORATORS_BASE, LIT_REACTIVE_ELEMENT_DECORATORS_CUSTOM_ELEMENT, LIT_REACTIVE_ELEMENT_DECORATORS_PROPERTY, LIT_REACTIVE_ELEMENT_DECORATORS_STATE, LIT_REACTIVE_ELEMENT_DECORATORS_EVENT_OPTIONS, LIT_REACTIVE_ELEMENT_DECORATORS_QUERY, LIT_REACTIVE_ELEMENT_DECORATORS_QUERY_ALL, LIT_REACTIVE_ELEMENT_DECORATORS_QUERY_ASYNC, LIT_REACTIVE_ELEMENT_DECORATORS_QUERY_ASSIGNED_ELEMENTS, LIT_REACTIVE_ELEMENT_DECORATORS_QUERY_ASSIGNED_NODES
// LIT_ELEMENT_INDEX
// LIT_ELEMENT_LIT_ELEMENT, LIT_HTML_ASYNC_DIRECTIVE, ASPECTRON_FLOW_UX_SRC_FLOW_HTML, ASPECTRON_FLOW_UX_SRC_PAGINATION
// ASPECTRON_FLOW_UX_SRC_FLOW_EVENTS
// ASPECTRON_FLOW_UX_SRC_FLOW_TOOLBAR_ITEM
// ASPECTRON_FLOW_UX_SRC_FLOW_GRIDSTACK_PANEL, ASPECTRON_FLOW_UX_SRC_FLOW_CONTEXT_TEST, ASPECTRON_FLOW_UX_SRC_FLOW_CONTEXT
// ASPECTRON_FLOW_UX_RESOURCES_EXTERN_PROGRESSBAR_PROGRESSBAR
// ASPECTRON_FLOW_UX_SRC_SVG, ASPECTRON_FLOW_UX_SRC_FLOW_ANCHOR, ASPECTRON_FLOW_UX_SRC_FLOW_ASYNC, ASPECTRON_FLOW_UX_SRC_FLOW_CAPTION_BAR, ASPECTRON_FLOW_UX_SRC_FLOW_CHECKBOX_STYLED, ASPECTRON_FLOW_UX_SRC_FLOW_CHECKBOX, ASPECTRON_FLOW_UX_SRC_FLOW_RADIO, ASPECTRON_FLOW_UX_SRC_FLOW_RADIO_BTN, ASPECTRON_FLOW_UX_SRC_FLOW_RADIO_BTNS, ASPECTRON_FLOW_UX_SRC_FLOW_RADIOS, ASPECTRON_FLOW_UX_SRC_FLOW_TOGGLE_BTN, ASPECTRON_FLOW_UX_SRC_FLOW_BTN_GROUP, ASPECTRON_FLOW_UX_SRC_FLOW_CIRCULAR_SHAPES, ASPECTRON_FLOW_UX_SRC_FLOW_FORM_REGION, ASPECTRON_FLOW_UX_SRC_FLOW_SVG_TEST, ASPECTRON_FLOW_UX_SRC_FLOW_TAB, ASPECTRON_FLOW_UX_SRC_FLOW_TABS, ASPECTRON_FLOW_UX_SRC_FLOW_TEST_ELEMENT, ASPECTRON_FLOW_UX_SRC_FLOW_TEXT_AREA, ASPECTRON_FLOW_UX_SRC_FLOW_FORM_CONTROL, ASPECTRON_FLOW_UX_SRC_FLOW_FORM_MIXIN, ASPECTRON_FLOW_UX_SRC_FLOW_FOLDER_INPUT, ASPECTRON_FLOW_UX_SRC_FLOW_INPUT, ASPECTRON_FLOW_UX_SRC_FLOW_CORE_INPUT, ASPECTRON_FLOW_UX_SRC_FLOW_EXPANDABLE, ASPECTRON_FLOW_UX_SRC_FLOW_QRCODE, ASPECTRON_FLOW_UX_SRC_FLOW_QRCODE_SCANNER, ASPECTRON_FLOW_UX_SRC_FLOW_BTN, ASPECTRON_FLOW_UX_SRC_FLOW_GROUP_BTNS, ASPECTRON_FLOW_UX_SRC_FLOW_SHELL_LINK, ASPECTRON_FLOW_UX_SRC_FLOW_WINDOW_LINK, ASPECTRON_FLOW_UX_SRC_FA_ICON, ASPECTRON_FLOW_UX_SRC_FLOW_DATA_BADGE, ASPECTRON_FLOW_UX_SRC_FLOW_DATA_BADGE_GRAPH, ASPECTRON_FLOW_UX_SRC_FLOW_DIALOG, ASPECTRON_FLOW_UX_SRC_FLOW_PAGES, ASPECTRON_FLOW_UX_SRC_FLOW_I18N, ASPECTRON_FLOW_UX_SRC_FLOW_CODE, ASPECTRON_FLOW_UX_SRC_FLOW_COLOR_SELECTOR, ASPECTRON_FLOW_UX_SRC_FLOW_REFERENCE, ASPECTRON_FLOW_UX_SRC_FLOW_CANVAS, ASPECTRON_FLOW_UX_SRC_FLOW_D3, ASPECTRON_FLOW_UX_SRC_FLOW_GRAPH, ASPECTRON_FLOW_UX_SRC_FLOW_SAMPLER, ASPECTRON_FLOW_UX_SRC_BASE_ELEMENT, ASPECTRON_FLOW_UX_SRC_FLOW_SOCKET, ASPECTRON_FLOW_UX_SRC_FLOW_SOCKET_RPC, ASPECTRON_FLOW_UX_SRC_FLOW_SOCKET_NATS, ASPECTRON_FLOW_UX_SRC_FLOW_APP, ASPECTRON_FLOW_UX_SRC_FLOW_APP_DRAWER, ASPECTRON_FLOW_UX_SRC_FLOW_DROPDOWN, ASPECTRON_FLOW_UX_SRC_FLOW_MENU, ASPECTRON_FLOW_UX_SRC_FLOW_SELECT, ASPECTRON_FLOW_UX_SRC_TABLE_MIXIN, ASPECTRON_FLOW_UX_SRC_FLOW_LINK, ASPECTRON_FLOW_UX_SRC_FLOW_SELECTOR, ASPECTRON_FLOW_UX_SRC_FLOW_FORMAT, ASPECTRON_FLOW_UX_SRC_FLOW_CLOCK_WIDGET, ASPECTRON_FLOW_UX_SRC_FLOW_THEME_SELECT, ASPECTRON_FLOW_UX_SRC_FLOW_DROPZONE_FIELD, ASPECTRON_FLOW_UX_SRC_FLOW_MARKDOWN, ASPECTRON_FLOW_UX_SRC_FLOW_TOOLBAR, ASPECTRON_FLOW_UX_SRC_FLOW_GRIDSTACK, ASPECTRON_FLOW_UX_SRC_FLOW_DOWNLOAD_BADGE, ASPECTRON_FLOW_UX_SRC_FLOW_COLOR_PICKER, ASPECTRON_FLOW_UX_SRC_FLOW_RSS, ASPECTRON_FLOW_UX_SRC_FLOW_SUNBURST_GRAPH, ASPECTRON_FLOW_UX_SRC_FLOW_SWIPEABLE, ASPECTRON_FLOW_UX_SRC_FLOW_T9, ASPECTRON_FLOW_UX_SRC_FLOW_PROGRESSBAR, ASPECTRON_FLOW_UX_SRC_FLOW_ADD_TO_HOME, ASPECTRON_FLOW_UX_SRC_COLORS, ASPECTRON_FLOW_UX_SRC_FLOW_STATSD
// ASPECTRON_FLOW_UX_FLOW_UX, ASPECTRON_FLOW_UX_SRC_FLOW_TERMINAL
// FLOW_UX
const DEPENDENCIES : &[&[ModuleId]] = &[
	&[409],
	&[613, 611, 546],
	&[505],
	&[501],
	&[265],
	&[31],
	&[89],
	&[38, 40, 33, 37, 32, 34, 36, 35, 39, 41],
	&[674],
	&[673, 550, 389, 398],
	&[367],
	&[406],
	&[361, 348, 422],
	&[341],
	&[414, 362, 396, 399, 425, 364, 424, 394, 356, 373, 372, 412, 397, 429, 433, 347, 421, 393, 354, 363, 376, 390, 428, 375, 377, 384, 420, 380, 403, 405, 349, 418, 386, 366, 360, 416, 379, 408, 423, 350, 432, 391, 351, 401, 410, 417, 400, 359, 381, 368, 352, 374, 413, 387, 419, 395, 378, 353, 404, 369, 383, 426, 370, 355, 385, 415, 430, 382, 427, 388, 371, 402, 407],
	&[434, 431],
	&[675]
];
