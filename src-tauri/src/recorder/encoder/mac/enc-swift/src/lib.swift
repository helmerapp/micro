import SwiftRs
import Foundation
import AVFoundation
import VideoToolbox
import CoreVideo
import CoreImage

// TODO: handle errors better
class Encoder: NSObject {
    var width: Int
    var height: Int
    var assetWriter: AVAssetWriter
    var assetWriterInput: AVAssetWriterInput
    var pixelBufferAdaptor: AVAssetWriterInputPixelBufferAdaptor

    init(_ width: Int, _ height: Int, _ outFile: URL) {
        self.width = width;
        self.height = height;

        // Setup AVAssetWriter
        // Create AVAssetWriter for a mp4 file
        self.assetWriter = try! AVAssetWriter(url: outFile, fileType: .mp4)
        
        // Prepare the AVAssetWriterInputPixelBufferAdaptor
        let outputSettings: [String: Any] = [
            AVVideoCodecKey: AVVideoCodecType.h264,
            AVVideoWidthKey: width,
            AVVideoHeightKey: height
        ]

        self.assetWriterInput = AVAssetWriterInput(mediaType: .video, outputSettings: outputSettings)
        self.assetWriterInput.expectsMediaDataInRealTime = true

        let sourcePixelBufferAttributes: [String: Any] = [
            kCVPixelBufferPixelFormatTypeKey as String: kCVPixelFormatType_420YpCbCr8BiPlanarFullRange
        ]

        self.pixelBufferAdaptor = AVAssetWriterInputPixelBufferAdaptor(
            assetWriterInput: self.assetWriterInput,
            sourcePixelBufferAttributes: sourcePixelBufferAttributes
        )
        
        if self.assetWriter.canAdd(self.assetWriterInput) {
            self.assetWriter.add(self.assetWriterInput)
        }

        self.assetWriter.startWriting()
        self.assetWriter.startSession(atSourceTime: CMTime.zero)
    }  
}


@_cdecl("encoder_init")
func encoderInit(_ width: Int, _ height: Int, _ outFile: SRString) -> Encoder {
    return Encoder(
        width,
        height,
        URL(fileURLWithPath: outFile.toString())
    )
}

func createCvPixelBufferFromYuvFrameData(
    _ width: Int,
    _ height: Int,
    _ displayTime: Int,
    _ luminanceStride: Int,
    _ luminanceBytes: [UInt8],
    _ chrominanceStride: Int,
    _ chrominanceBytes: [UInt8]
) -> CVPixelBuffer? {

    let pixelBufferAttributes: CFDictionary = [
        kCVPixelBufferIOSurfacePropertiesKey: [:] as CFDictionary,
        kCVPixelBufferPixelFormatTypeKey: kCVPixelFormatType_420YpCbCr8BiPlanarFullRange
    ] as CFDictionary

    var pixelBuffer: CVPixelBuffer?

    let status = CVPixelBufferCreate(
        kCFAllocatorDefault,
        width,
        height,
        kCVPixelFormatType_420YpCbCr8BiPlanarFullRange,
        pixelBufferAttributes,
        &pixelBuffer
    )

    if status != kCVReturnSuccess {
        print("Failed to create CVPixelBuffer")
        return nil
    }

    // Get the base addresses of the Y and UV planes
    CVPixelBufferLockBaseAddress(pixelBuffer!, CVPixelBufferLockFlags(rawValue: 0))
    let yPlaneAddress = CVPixelBufferGetBaseAddressOfPlane(pixelBuffer!, 0)
    let uvPlaneAddress = CVPixelBufferGetBaseAddressOfPlane(pixelBuffer!, 1)

    // Copy the luminance (Y) data to the Y plane
    let yDestPointer = yPlaneAddress?.assumingMemoryBound(to: UInt8.self)
    yDestPointer?.update(from: luminanceBytes, count: luminanceBytes.count)

    // Copy the chrominance (UV) data to the UV plane
    let uvDestPointer = uvPlaneAddress?.assumingMemoryBound(to: UInt8.self)
    uvDestPointer?.update(from: chrominanceBytes, count: chrominanceBytes.count)

    CVPixelBufferUnlockBaseAddress(pixelBuffer!, CVPixelBufferLockFlags(rawValue: 0))

    return pixelBuffer
}

// NOTE: make any timestamp adjustments in Rust before passing here

@_cdecl("encoder_ingest_yuv_frame")
func encoderIngestYuvFrame(
    _ enc: Encoder,
    _ width: Int,
    _ height: Int,
    _ displayTime: Int,
    _ luminanceStride: Int,
    _ luminanceBytesRaw: SRData,
    _ chrominanceStride: Int,
    _ chrominanceBytesRaw: SRData
) {
    let luminanceBytes = luminanceBytesRaw.toArray()
    let chrominanceBytes = chrominanceBytesRaw.toArray()

    // Create a CVPixelBuffer from YUV data
    var pixelBuffer = createCvPixelBufferFromYuvFrameData(
        width,
        height,
        displayTime,
        luminanceStride,
        luminanceBytes,
        chrominanceStride,
        chrominanceBytes
    )
            
    // Append the CVPixelBuffer to the AVAssetWriter
    if enc.assetWriterInput.isReadyForMoreMediaData {
        let frameTime = CMTimeMake(value: Int64(displayTime), timescale: 1000000000)
        let success = enc.pixelBufferAdaptor.append(pixelBuffer!, withPresentationTime: frameTime)
        if !success {
            print("Asset writer error: \(enc.assetWriter.error?.localizedDescription ?? "Unknown error")")
        } else {
            print("frame appended successfully")
        }
    } else {
        print("Asset writer input is not ready for more media data")
    }
}


func createCvPixelBufferFromBgraFrameData(
    _ width: Int,
    _ height: Int,
    _ displayTime: Int,
    _ bytesPerRow: Int,
    _ bgraBytes: [UInt8]
) -> CVPixelBuffer? {
    let pixelBufferAttributes: CFDictionary = [
        kCVPixelBufferIOSurfacePropertiesKey: [:] as CFDictionary,
        kCVPixelBufferPixelFormatTypeKey: kCVPixelFormatType_32BGRA
    ] as CFDictionary

    var pixelBuffer: CVPixelBuffer?

    let status = CVPixelBufferCreate(
        kCFAllocatorDefault,
        width,
        height,
        kCVPixelFormatType_32BGRA,
        pixelBufferAttributes,
        &pixelBuffer
    )

    if status != kCVReturnSuccess {
        print("Failed to create CVPixelBuffer")
        return nil
    }

    // Get the base address of the pixel buffer
    CVPixelBufferLockBaseAddress(pixelBuffer!, CVPixelBufferLockFlags(rawValue: 0))
    let baseAddress = CVPixelBufferGetBaseAddress(pixelBuffer!)

    // Copy the BGRA data to the pixel buffer
    let destPointer = baseAddress?.assumingMemoryBound(to: UInt8.self)
    destPointer?.update(from: bgraBytes, count: bgraBytes.count)

    CVPixelBufferUnlockBaseAddress(pixelBuffer!, CVPixelBufferLockFlags(rawValue: 0))

    return pixelBuffer
}


@_cdecl("encoder_ingest_bgra_frame")
func encoderIngestBgraFrame(
    _ enc: Encoder,
    _ width: Int,
    _ height: Int,
    _ displayTime: Int,
    _ bytesPerRow: Int,
    _ bgraBytesRaw: SRData
) {
    let bgraBytes = bgraBytesRaw.toArray()

    // Create a CVPixelBuffer from BGRA data
    var pixelBuffer = createCvPixelBufferFromBgraFrameData(
        width,
        height,
        displayTime,
        bytesPerRow,
        bgraBytes
    )
            
    // Append the CVPixelBuffer to the AVAssetWriter
    if enc.assetWriterInput.isReadyForMoreMediaData {
        let frameTime = CMTimeMake(value: Int64(displayTime), timescale: 1000000000)
        let success = enc.pixelBufferAdaptor.append(pixelBuffer!, withPresentationTime: frameTime)
        if !success {
            print("Asset writer error: \(enc.assetWriter.error?.localizedDescription ?? "Unknown error")")
        } else {
            print("frame appended successfully")
        }
    } else {
        print("Asset writer input is not ready for more media data")
    }
}

@_cdecl("encoder_finish")
func encoderFinish(_ enc: Encoder) {

    // TODO: figure out how to gracefully end session
    // enc.assetWriter.endSession(atSourceTime: CMTime)
    
    enc.assetWriterInput.markAsFinished()
    enc.assetWriter.finishWriting{}

    while enc.assetWriter.status == .writing {
        print("Waiting for asset writer to finish writing...")
        usleep(500000)
    }

    print("Asset writer finished writing")
}
