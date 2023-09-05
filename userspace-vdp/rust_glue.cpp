#include "fabutils.h"
#include "fake_fabgl.h"
#include "fabgl.h"
#include "video.h"
#include "dispdrivers/vga16controller.h"
#include "dispdrivers/vgabasecontroller.h"
#include "ps2controller.h"

extern fabgl::VGABaseController *	_VGAController;		// Pointer to the current VGA controller class (one of the above)

/* ps2scancode is the set2 'make' code */
extern "C" void sendHostKbEventToFabgl(uint16_t ps2scancode, bool isDown)
{
	fabgl::PS2Controller::keyboard()->injectScancode(ps2scancode, isDown);
}

/* Buffer must be big enough for any screen resolution - up to 1024x768x3 bytes :) */
extern "C" void copyVgaFramebuffer(int *outWidth, int *outHeight, void *buffer)
{
	auto lock = _VGAController->lock();
	const int w = _VGAController->getScreenWidth();
	const int h = _VGAController->getScreenHeight();
	*outHeight = h;
	*outWidth = w;
	// rect is inclusive range
	Rect rect(0, 0, w-1, h-1);
	_VGAController->readScreen(rect, (fabgl::RGB888*)buffer);
}

extern "C" void vdp_setup() {
	setup();
}

extern "C" void vdp_loop() {
	loop();
}