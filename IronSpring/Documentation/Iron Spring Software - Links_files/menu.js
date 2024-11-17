// These scripts handle the menu functions
function over(id) {
	var n = document.getElementById(id).className;
	if(n!='inact') {
          setStyleById(id,"backgroundColor", "LightSkyBlue" );
          setStyleById(id,"color", "White" );
	  }
	}

function out(id) {
	var n = document.getElementById(id).className;
	if(n!='inact') {
          setStyleById(id,"backgroundColor", "Blue" );
          setStyleById(id,"color", "White" );
	  }
	}

function setStyleById(i, p, v) {
	var n = document.getElementById(i);
	n.style[p] = v;
	}

function link(id,page) {
	var n = document.getElementById(id).className;
	if(n!='inact') {
	  window.location = "http://www.iron-spring.com/"+page+".html";
	  }
	}

function inact(id) {
	document.getElementById(id).className='inact';
	}
