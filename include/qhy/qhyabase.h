/*
 QHYCCD SDK
 
 Copyright (c) 2014 QHYCCD.
 All Rights Reserved.
 
 This program is free software; you can redistribute it and/or modify it
 under the terms of the GNU General Public License as published by the Free
 Software Foundation; either version 2 of the License, or (at your option)
 any later version.
 
 This program is distributed in the hope that it will be useful, but WITHOUT
 ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
 more details.
 
 You should have received a copy of the GNU General Public License along with
 this program; if not, write to the Free Software Foundation, Inc., 59
 Temple Place - Suite 330, Boston, MA  02111-1307, USA.
 
 The full GNU General Public License is included in this distribution in the
 file called LICENSE.
 */

/*!
 * @file qhyabase.h
 * @brief QHYABASE class define
 */

#ifndef QHYABASE_CLASS
#define QHYABASE_CLASS

#include "qhybase.h"

/**
 * @brief QHYABASE class define
 *
 * include all functions for QHYABASE
 */
class QHYABASE:public QHYBASE
{
public:
    QHYABASE();
    ~QHYABASE();

    uint32_t BeginSingleExposure(qhyccd_handle *h);
    uint32_t CancelExposing(qhyccd_handle *handle);
    uint32_t CancelExposingAndReadout(qhyccd_handle *h);
    uint32_t BeginLiveExposure(qhyccd_handle *h);
    uint32_t StopLiveExposure(qhyccd_handle *h);
    uint32_t GetSingleFrame(qhyccd_handle *h,uint32_t *pW,uint32_t *pH,uint32_t * pBpp,uint32_t *pChannels,uint8_t *ImgData);
    uint32_t GetLiveFrame(qhyccd_handle *h,uint32_t *pW,uint32_t *pH,uint32_t * pBpp,uint32_t *pChannels,uint8_t *ImgData);
    
	static void ThreadCancelExposingAndReadout(void *p);
	void ThreadCancelExposingAndReadoutStart(qhyccd_handle *h);

	uint32_t DisConnectCamera(qhyccd_handle *h);
    /**
     @fn uint32_t InitChipRegs(qhyccd_handle *h)
     @brief Init the registers and some other things
     @param h camera control handle
     @return
     success return QHYCCD_SUCCESS \n
     another QHYCCD_ERROR code on other failures
     */
    uint32_t InitChipRegs(qhyccd_handle *h);

    /**
     @fn uint32_t IsChipHasFunction(CONTROL_ID id)
     @brief check the camera has the function or not
     @param id function id
     @return
     HAVE return QHYCCD_HAVE \n
     NOT HAVE return QHYCCD_NOTHAVE
     */
    uint32_t IsChipHasFunction(CONTROL_ID id);
          
	/**
     @fn virtual uint32_t GetChipMemoryLength()
     @brief get the image cost memory length
     @return
     success return memory length \n
     another QHYCCD_ERROR code on other failures
     */
     uint32_t GetChipMemoryLength();

    /**
     @fn uint32_t SetChipGain(qhyccd_handle *h,double gain)
     @brief set the gain to camera
     @param h camera control handle
     @param gain gain value
     @return
     success return QHYCCD_SUCCESS \n
     another QHYCCD_ERROR code on other failures
     */
    uint32_t SetChipGain(qhyccd_handle *h,double gain);
    
    /**
     @fn uint32_t SetChipExposeTime(qhyccd_handle *h,double i)
     @brief set the expose time to camera
     @param h camera control handle
     @param i expose time value
     @return
     success return QHYCCD_SUCCESS \n
     another QHYCCD_ERROR code on other failures
     */
    uint32_t SetChipExposeTime(qhyccd_handle *h,double i);
    
    /**
     @fn uint32_t SetChipSpeed(qhyccd_handle *h,uint32_t i)
     @brief set the transfer speed to camera
     @param h camera control handle
     @param i speed level
     @return
     success return QHYCCD_SUCCESS \n
     another QHYCCD_ERROR code on other failures
     */
    uint32_t SetChipSpeed(qhyccd_handle *h,uint32_t i);
  
    /**
     @fn uint32_t SetChipOffset(qhyccd_handle *h,double offset)
     @brief set the camera offset
     @param h camera control handle
     @param offset offset value
     @return
     success return QHYCCD_SUCCESS \n
     another QHYCCD_ERROR code on other failures
     */
    uint32_t SetChipOffset(qhyccd_handle *h,double offset);
    
    /**
     @fn uint32_t SetChipBinMode(qhyccd_handle *h,uint32_t wbin,uint32_t hbin)
     @brief set the camera offset
     @param h camera control handle
     @param wbin width bin
     @param hbin height bin
     @return
     success return QHYCCD_SUCCESS \n
     another QHYCCD_ERROR code on other failures
     */
    uint32_t SetChipBinMode(qhyccd_handle *h,uint32_t wbin,uint32_t hbin);
    
    /**
     @fn uint32_t GetControlMinMaxStepValue(CONTROL_ID controlId,double *min,double *max,double *step)
     @brief get the min,max and step value for function
     @param controlId the control id
     @param min the min value for function
     @param max the max value for function
     @param step single step value for function
     @return
     success return QHYCCD_SUCCESS \n
     another QHYCCD_ERROR code on other failures
     */
    uint32_t GetControlMinMaxStepValue(CONTROL_ID controlId,double *min,double *max,double *step);
    
    /**
     @fn double GetChipCoolTemp(qhyccd_handle *h)
     @brief get the current ccd/cmos temprature
     @param h camera control handle
     @return
     success return the current cool temprature \n
     another QHYCCD_ERROR code on other failures
     */
    double GetChipCoolTemp(qhyccd_handle *h);
    
   /** 
    @fn uint32_t SetChipResolution(qhyccd_handle *handle,uint32_t x,uint32_t y,uint32_t xsize,uint32_t ysize)
    @brief set camera ouput resolution
    @param handle camera control handle
    @param x the top left position x
    @param y the top left position y
    @param xsize the image width
    @param ysize the image height
    @return
    on success,return QHYCCD_SUCCESS\n
    another QHYCCD_ERROR code on other failures
    */
    uint32_t SetChipResolution(qhyccd_handle *handle,uint32_t x,uint32_t y,uint32_t xsize,uint32_t ysize);
    
	static void ThreadCountExposureTime(void *p);

	void ThreadCountExposureTimeStart(void *p);

    /**
     @fn uint32_t AutoTempControl(qhyccd_handle *h,double ttemp)
     @brief auto temprature control
     @param h camera control handle
     @param ttemp target temprature(degree Celsius)
     @return
     success return QHYCCD_SUCCESS \n
     another QHYCCD_ERROR code on other failures
     */
    uint32_t AutoTempControl(qhyccd_handle *h,double ttemp);
    
    /**
     @fn uint32_t SetChipCoolPWM(qhyccd_handle *h,double PWM)
     @brief set cool power
     @param h camera control handle
     @param PWM power(0-255)
     @return
     success return QHYCCD_SUCCESS \n
     another QHYCCD_ERROR code on other failures
     */
    uint32_t SetChipCoolPWM(qhyccd_handle *h,double PWM);
    
    /**
     @fn void ConvertDataBIN11(uint8_t * Data,uint32_t x, uint32_t y, uint16_t PixShift)
     @brief move the pixel raw data to correction position,and bin if need
     @param Data raw image data
     @param x image width
     @param y image height
     @param PixShift this is a way to fix the bad pixel data by the usb transfer
     */
    void ConvertDataBIN11(uint8_t * Data,uint32_t x, uint32_t y, uint16_t PixShift);
    
    /**
     @fn void ConvertDataBIN22(uint8_t * Data,uint32_t x, uint32_t y, uint16_t PixShift)
     @brief move the pixel raw data to correction position,and bin if need
     @param Data raw image data
     @param x image width
     @param y image height
     @param PixShift this is a way to fix the bad pixel data by the usb transfer
     */
    void ConvertDataBIN22(uint8_t * Data,uint32_t x, uint32_t y, uint16_t PixShift);
 
   /**
     @fn void ConvertDataBIN44(uint8_t * Data,uint32_t x, uint32_t y, uint16_t PixShift)
     @brief move the pixel raw data to correction position,and bin if need
     @param Data raw image data
     @param x image width
     @param y image height
     @param PixShift this is a way to fix the bad pixel data by the usb transfer
     */
    void ConvertDataBIN44(uint8_t * Data,uint32_t x, uint32_t y, uint16_t PixShift);
	 
    /**
      @fn uint32_t SetFocusSetting(qhyccd_handle *h,uint32_t focusCenterX, uint32_t focusCenterY)
      @brief Set the camera on focus mode
      @param h camera control handle
      @param focusCenterX
      @param focusCenterY
      @return
	  on success,return QHYCCD_SUCCESS \n
	  another QHYCCD_ERROR code on other failures
     */
     uint32_t SetFocusSetting(qhyccd_handle *h,uint32_t focusCenterX, uint32_t focusCenterY);
    /** @fn virtual uint32_t SetInterCamSerialParam(qhyccd_handle *h,uint32_t opt)
      @brief Set InterCam serial2 params
      @param h camera control handle
	  @param opt the param
      @return
	  on success,return QHYCCD_SUCCESS \n

	  another QHYCCD_ERROR code on other failures
    */
     uint32_t SetInterCamSerialParam(qhyccd_handle *h,uint32_t opt);

  /** @fn virtual uint32_t InterCamSerialTX(qhyccd_handle *h,char *buf,uint32_t length)
      @brief Send data to InterCam serial2
      @param h camera control handle
	  @param buf buffer for data
	  @param length the length to send
      @return
	  on success,return QHYCCD_SUCCESS \n

	  another QHYCCD_ERROR code on other failures
    */
     uint32_t InterCamSerialTX(qhyccd_handle *h,char *buf,uint32_t length);


  /** @fn virtual uint32_t InterCamSerialRX(qhyccd_handle *h,char *buf)
      @brief Get data from InterCam serial2
      @param h camera control handle
	  @param buf buffer for data
      @return
	  on success,return the data number \n

	  another QHYCCD_ERROR code on other failures
    */
	 uint32_t InterCamSerialRX(qhyccd_handle *h,char *buf);

   /** @fn virtual uint32_t Send2OledFast(qhyccd_handle *h,char *buffer)
      @brief send data to show on InterCam's OLED
      @param h camera control handle
	  @param buffer buffer for data
      @return
	  on success,return QHYCCD_SUCCESS \n
	  another QHYCCD_ERROR code on other failures
    */
     uint32_t Send2OledFast(qhyccd_handle *h,uint8_t *buffer);

	/** @fn uint32_t InterCamOledOnOff(qhyccd_handle *handle,uint8_t onoff)
      @brief turn off or turn on the InterCam's Oled
      @param handle camera control handle
	  @param onoff on or off the oled \n
	  1:on \n
	  0:off \n
      @return
	  on success,return QHYCCD_SUCCESS \n
	  another QHYCCD_ERROR code on other failures
    */
	 uint32_t InterCamOledOnOff(qhyccd_handle *handle,uint8_t onoff);

	/** @fn uint32_t SetInterCamOledBrightness(qhyccd_handle *handle,uint8_t brightness)
      @brief send data to show on InterCam's OLED
      @param handle camera control handle
	  @param brightness the oled's brightness
      @return
	  on success,return QHYCCD_SUCCESS \n
	  another QHYCCD_ERROR code on other failures
    */
	 uint32_t SetInterCamOledBrightness(qhyccd_handle *handle,uint8_t brightness);

     uint32_t SendFourLine2InterCamOled(qhyccd_handle *handle,char *messagetemp,char *messageinfo,char *messagetime,char *messagemode);

	/** @fn uint32_t SendTwoLine2InterCamOled(qhyccd_handle *handle,char *messageTop,char *messageBottom)
      @brief spilit the message to two line,send to camera
      @param handle camera control handle
	  @param messageTop message for the oled's 1st line
	  @param messageBottom message for the oled's 2nd line
      @return
	  on success,return QHYCCD_SUCCESS \n
	  another QHYCCD_ERROR code on other failures
    */
	 uint32_t SendTwoLine2InterCamOled(qhyccd_handle *handle,char *messageTop,char *messageBottom);

      /** 
      @fn uint32_t SendOneLine2InterCamOled(qhyccd_handle *handle,char *messageTop)
      @brief spilit the message to two line,send to camera
      @param handle camera control handle
      @param messageTop message for all the oled
      @return
      on success,return QHYCCD_SUCCESS \n
      another QHYCCD_ERROR code on other failures
      */  
      uint32_t SendOneLine2InterCamOled(qhyccd_handle *handle,char *messageTop);

      /** 
      @fn uint32_t GetCameraStatus(qhyccd_handle *h,uint8_t *buf)
      @brief Get camera status
      @param h camera control handle
      @param buf camera's status save space
      @return
      on success,return the camera statu \n
      another QHYCCD_ERROR code on other failures
      */
      uint32_t GetCameraStatus(qhyccd_handle *h,uint8_t *buf);

	 /** 
	  @fn uint32_t SendOrder2CFW(qhyccd_handle *handle,char *order,uint32_t length)
      @brief control color filter wheel 
      @param handle camera control handle
	  @param order order send to color filter wheel
	  @param length the order string length
	  @return
	  on success,return QHYCCD_SUCCESS \n
	  another QHYCCD_ERROR code on other failures
    */
	 uint32_t SendOrder2CFW(qhyccd_handle *handle,char *order,uint32_t length);

	 /** 
	  @fn uint32_t GetCFWStatus(qhyccd_handle *handle,char *status)
      @brief get the color filter wheel status
      @param handle camera control handle
	  @param status the color filter wheel position status
	  @return
	  on success,return QHYCCD_SUCCESS \n
	  another QHYCCD_ERROR code on other failures
    */
	uint32_t GetCFWStatus(qhyccd_handle *handle,char *status);

	 /** 
	  @fn uint32_t ControlShutter(qhyccd_handle *handle,uint8_t status)
      @brief control camera's shutter
      @param handle camera control handle
	  @param status the shutter status
	  @return
	  on success,return QHYCCD_SUCCESS \n
	  another QHYCCD_ERROR code on other failures
     */
	 uint32_t ControlShutter(qhyccd_handle *handle,uint8_t status);

	 /** 
	  @fn uint32_t GetShutterStatus(qhyccd_handle *handle)
      @brief get the camera's shutter status 
      @param handle camera control handle
	  @return
	  on success,return status \n
	  another QHYCCD_ERROR code on other failures
     */
	 uint32_t GetShutterStatus(qhyccd_handle *handle);

	 uint32_t GetHumidity(qhyccd_handle *handle,double *hd);

	 uint32_t SetTrigerFunction(qhyccd_handle *handle,bool value);

	 double GetReadingProgress(qhyccd_handle *handle);

	 uint32_t GetPreProcessInfoFromEEPROM(qhyccd_handle *h);

	 uint32_t widthmax;
	 uint32_t heightmax;

	 uint32_t curOverScanX;
	 uint32_t curOverScanY;
	 uint32_t curOverScanSizeX;
	 uint32_t curOverScanSizeY;
#ifdef WIN32
	 HANDLE hCountExpTimeThread;
	 HANDLE hCancelExposingAndReadoutThread;
#endif
	 uint32_t isReadoutTemp;

	 uint8_t flagquitgetsingleframe;

	 bool isbin22Tobin33;
	 uint32_t wantedWidth;
	 uint32_t wantedHeight;

	 uint8_t badlinenum;
	 uint16_t badlinex[32];
	 uint16_t badliney[32];
	 uint8_t badlinewidth[32];
	 uint16_t badlineendy[32];
};
#endif

