; ModuleID = 'probe1.56ebf6f4-cgu.0'
source_filename = "probe1.56ebf6f4-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

%"std::fmt::Formatter" = type { [0 x i64], { i64, i64 }, [0 x i64], { i64, i64 }, [0 x i64], { {}*, [3 x i64]* }, [0 x i32], i32, [0 x i32], i32, [0 x i8], i8, [7 x i8] }
%"core::fmt::Opaque" = type {}
%"std::fmt::Arguments" = type { [0 x i64], { [0 x { [0 x i8]*, i64 }]*, i64 }, [0 x i64], { i64*, i64 }, [0 x i64], { [0 x { i8*, i64* }]*, i64 }, [0 x i64] }
%"std::string::String" = type { [0 x i64], %"std::vec::Vec<u8>", [0 x i64] }
%"std::vec::Vec<u8>" = type { [0 x i64], { i8*, i64 }, [0 x i64], i64, [0 x i64] }
%"std::marker::PhantomData<u8>" = type {}
%"std::ptr::metadata::PtrRepr<[u8]>" = type { [2 x i64] }
%"std::option::Option<(std::ptr::NonNull<u8>, std::alloc::Layout)>" = type { [0 x i64], {}*, [2 x i64] }
%"std::option::Option<(std::ptr::NonNull<u8>, std::alloc::Layout)>::Some" = type { [0 x i64], { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }, [0 x i64] }
%"std::alloc::Global" = type {}

@alloc2 = private unnamed_addr constant <{ [0 x i8] }> zeroinitializer, align 1
@alloc3 = private unnamed_addr constant <{ i8*, [8 x i8] }> <{ i8* getelementptr inbounds (<{ [0 x i8] }>, <{ [0 x i8] }>* @alloc2, i32 0, i32 0, i32 0), [8 x i8] zeroinitializer }>, align 8
@alloc5 = private unnamed_addr constant <{ [8 x i8] }> zeroinitializer, align 8

; <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from
; Function Attrs: inlinehint uwtable
define nonnull i8* @"_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17hcf51b9cff811e285E"(i8* nonnull %unique) unnamed_addr #0 {
start:
; call core::ptr::unique::Unique<T>::as_ptr
  %_2 = call i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17hd7fa4ebeb4b5553aE"(i8* nonnull %unique)
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::non_null::NonNull<T>::new_unchecked
  %0 = call nonnull i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17hd57d0268af51bb34E"(i8* %_2)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i8* %0
}

; core::fmt::ArgumentV1::new
; Function Attrs: uwtable
define { i8*, i64* } @_ZN4core3fmt10ArgumentV13new17h9f64c2cca8ecd4d2E(i64* align 8 dereferenceable(8) %x, i1 (i64*, %"std::fmt::Formatter"*)* nonnull %f) unnamed_addr #1 {
start:
  %0 = alloca %"core::fmt::Opaque"*, align 8
  %1 = alloca i1 (%"core::fmt::Opaque"*, %"std::fmt::Formatter"*)*, align 8
  %2 = alloca { i8*, i64* }, align 8
  %3 = bitcast i1 (i64*, %"std::fmt::Formatter"*)* %f to i1 (%"core::fmt::Opaque"*, %"std::fmt::Formatter"*)*
  store i1 (%"core::fmt::Opaque"*, %"std::fmt::Formatter"*)* %3, i1 (%"core::fmt::Opaque"*, %"std::fmt::Formatter"*)** %1, align 8
  %_3 = load i1 (%"core::fmt::Opaque"*, %"std::fmt::Formatter"*)*, i1 (%"core::fmt::Opaque"*, %"std::fmt::Formatter"*)** %1, align 8, !nonnull !1
  br label %bb1

bb1:                                              ; preds = %start
  %4 = bitcast i64* %x to %"core::fmt::Opaque"*
  store %"core::fmt::Opaque"* %4, %"core::fmt::Opaque"** %0, align 8
  %_5 = load %"core::fmt::Opaque"*, %"core::fmt::Opaque"** %0, align 8, !nonnull !1
  br label %bb2

bb2:                                              ; preds = %bb1
  %5 = bitcast { i8*, i64* }* %2 to %"core::fmt::Opaque"**
  store %"core::fmt::Opaque"* %_5, %"core::fmt::Opaque"** %5, align 8
  %6 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %2, i32 0, i32 1
  %7 = bitcast i64** %6 to i1 (%"core::fmt::Opaque"*, %"std::fmt::Formatter"*)**
  store i1 (%"core::fmt::Opaque"*, %"std::fmt::Formatter"*)* %_3, i1 (%"core::fmt::Opaque"*, %"std::fmt::Formatter"*)** %7, align 8
  %8 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %2, i32 0, i32 0
  %9 = load i8*, i8** %8, align 8, !nonnull !1
  %10 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %2, i32 0, i32 1
  %11 = load i64*, i64** %10, align 8, !nonnull !1
  %12 = insertvalue { i8*, i64* } undef, i8* %9, 0
  %13 = insertvalue { i8*, i64* } %12, i64* %11, 1
  ret { i8*, i64* } %13
}

; core::fmt::Arguments::new_v1
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3fmt9Arguments6new_v117he4a9a1f377defa3cE(%"std::fmt::Arguments"* noalias nocapture sret(%"std::fmt::Arguments") dereferenceable(48) %0, [0 x { [0 x i8]*, i64 }]* nonnull align 8 %pieces.0, i64 %pieces.1, [0 x { i8*, i64* }]* nonnull align 8 %args.0, i64 %args.1) unnamed_addr #0 {
start:
  %_4 = alloca { i64*, i64 }, align 8
  %1 = bitcast { i64*, i64 }* %_4 to {}**
  store {}* null, {}** %1, align 8
  %2 = bitcast %"std::fmt::Arguments"* %0 to { [0 x { [0 x i8]*, i64 }]*, i64 }*
  %3 = getelementptr inbounds { [0 x { [0 x i8]*, i64 }]*, i64 }, { [0 x { [0 x i8]*, i64 }]*, i64 }* %2, i32 0, i32 0
  store [0 x { [0 x i8]*, i64 }]* %pieces.0, [0 x { [0 x i8]*, i64 }]** %3, align 8
  %4 = getelementptr inbounds { [0 x { [0 x i8]*, i64 }]*, i64 }, { [0 x { [0 x i8]*, i64 }]*, i64 }* %2, i32 0, i32 1
  store i64 %pieces.1, i64* %4, align 8
  %5 = getelementptr inbounds %"std::fmt::Arguments", %"std::fmt::Arguments"* %0, i32 0, i32 3
  %6 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %_4, i32 0, i32 0
  %7 = load i64*, i64** %6, align 8
  %8 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %_4, i32 0, i32 1
  %9 = load i64, i64* %8, align 8
  %10 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %5, i32 0, i32 0
  store i64* %7, i64** %10, align 8
  %11 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %5, i32 0, i32 1
  store i64 %9, i64* %11, align 8
  %12 = getelementptr inbounds %"std::fmt::Arguments", %"std::fmt::Arguments"* %0, i32 0, i32 5
  %13 = getelementptr inbounds { [0 x { i8*, i64* }]*, i64 }, { [0 x { i8*, i64* }]*, i64 }* %12, i32 0, i32 0
  store [0 x { i8*, i64* }]* %args.0, [0 x { i8*, i64* }]** %13, align 8
  %14 = getelementptr inbounds { [0 x { i8*, i64* }]*, i64 }, { [0 x { i8*, i64* }]*, i64 }* %12, i32 0, i32 1
  store i64 %args.1, i64* %14, align 8
  ret void
}

; core::num::nonzero::NonZeroUsize::new_unchecked
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core3num7nonzero12NonZeroUsize13new_unchecked17h2dd7bb8c175ff335E(i64 %n) unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  store i64 %n, i64* %0, align 8
  %1 = load i64, i64* %0, align 8, !range !2
  ret i64 %1
}

; core::num::nonzero::NonZeroUsize::get
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core3num7nonzero12NonZeroUsize3get17h82ccd89a31e7231eE(i64 %self) unnamed_addr #0 {
start:
  ret i64 %self
}

; core::ptr::slice_from_raw_parts_mut
; Function Attrs: inlinehint uwtable
define { [0 x i8]*, i64 } @_ZN4core3ptr24slice_from_raw_parts_mut17h03de4e1e292c4e6bE(i8* %data, i64 %len) unnamed_addr #0 {
start:
  %0 = bitcast i8* %data to {}*
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::metadata::from_raw_parts_mut
  %1 = call { [0 x i8]*, i64 } @_ZN4core3ptr8metadata18from_raw_parts_mut17ha28cdfd7ab6c33dfE({}* %0, i64 %len)
  %2 = extractvalue { [0 x i8]*, i64 } %1, 0
  %3 = extractvalue { [0 x i8]*, i64 } %1, 1
  br label %bb2

bb2:                                              ; preds = %bb1
  %4 = insertvalue { [0 x i8]*, i64 } undef, [0 x i8]* %2, 0
  %5 = insertvalue { [0 x i8]*, i64 } %4, i64 %3, 1
  ret { [0 x i8]*, i64 } %5
}

; core::ptr::drop_in_place<alloc::string::String>
; Function Attrs: uwtable
define void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h71ff5be47284ad5aE"(%"std::string::String"* %_1) unnamed_addr #1 {
start:
  %0 = bitcast %"std::string::String"* %_1 to %"std::vec::Vec<u8>"*
; call core::ptr::drop_in_place<alloc::vec::Vec<u8>>
  call void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h160ea191c64b3515E"(%"std::vec::Vec<u8>"* %0)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::drop_in_place<alloc::vec::Vec<u8>>
; Function Attrs: uwtable
define void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h160ea191c64b3515E"(%"std::vec::Vec<u8>"* %_1) unnamed_addr #1 personality i32 (...)* @__CxxFrameHandler3 {
start:
; invoke <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
  invoke void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hd72c1a19dfb60615E"(%"std::vec::Vec<u8>"* align 8 dereferenceable(24) %_1)
          to label %bb4 unwind label %funclet_bb3

bb4:                                              ; preds = %start
  %0 = bitcast %"std::vec::Vec<u8>"* %_1 to { i8*, i64 }*
; call core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
  call void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17had575e63cd8032f1E"({ i8*, i64 }* %0)
  br label %bb2

bb3:                                              ; preds = %funclet_bb3
  %1 = bitcast %"std::vec::Vec<u8>"* %_1 to { i8*, i64 }*
; call core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
  call void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17had575e63cd8032f1E"({ i8*, i64 }* %1) #6 [ "funclet"(token %cleanuppad) ]
  br label %bb1

funclet_bb3:                                      ; preds = %start
  %cleanuppad = cleanuppad within none []
  br label %bb3

bb1:                                              ; preds = %bb3
  cleanupret from %cleanuppad unwind to caller

bb2:                                              ; preds = %bb4
  ret void
}

; core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
; Function Attrs: uwtable
define void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17had575e63cd8032f1E"({ i8*, i64 }* %_1) unnamed_addr #1 {
start:
; call <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hf4e56d07c00dc84cE"({ i8*, i64 }* align 8 dereferenceable(16) %_1)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::unique::Unique<T>::new_unchecked
; Function Attrs: inlinehint uwtable
define nonnull i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17h0f77f261040d3ec3E"(i8* %ptr) unnamed_addr #0 {
start:
  %0 = alloca i8*, align 8
  store i8* %ptr, i8** %0, align 8
  %1 = bitcast i8** %0 to %"std::marker::PhantomData<u8>"*
  %2 = load i8*, i8** %0, align 8, !nonnull !1
  ret i8* %2
}

; core::ptr::unique::Unique<T>::cast
; Function Attrs: inlinehint uwtable
define nonnull i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$4cast17h66d13f26e05dbe0fE"(i8* nonnull %self) unnamed_addr #0 {
start:
; call core::ptr::unique::Unique<T>::as_ptr
  %_3 = call i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17hd7fa4ebeb4b5553aE"(i8* nonnull %self)
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::unique::Unique<T>::new_unchecked
  %0 = call nonnull i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17h0f77f261040d3ec3E"(i8* %_3)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i8* %0
}

; core::ptr::unique::Unique<T>::as_ptr
; Function Attrs: inlinehint uwtable
define i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17hd7fa4ebeb4b5553aE"(i8* nonnull %self) unnamed_addr #0 {
start:
  ret i8* %self
}

; core::ptr::mut_ptr::<impl *mut T>::guaranteed_eq
; Function Attrs: inlinehint uwtable
define zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$13guaranteed_eq17hb748727c37629c9eE"(i8* %self, i8* %other) unnamed_addr #0 {
start:
  %0 = alloca i8, align 1
  %1 = icmp eq i8* %self, %other
  %2 = zext i1 %1 to i8
  store i8 %2, i8* %0, align 1
  %3 = load i8, i8* %0, align 1, !range !3
  %4 = trunc i8 %3 to i1
  br label %bb1

bb1:                                              ; preds = %start
  ret i1 %4
}

; core::ptr::mut_ptr::<impl *mut T>::is_null
; Function Attrs: inlinehint uwtable
define zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17hdd37fb0f130d6bc5E"(i8* %self) unnamed_addr #0 {
start:
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::mut_ptr::<impl *mut T>::guaranteed_eq
  %0 = call zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$13guaranteed_eq17hb748727c37629c9eE"(i8* %self, i8* null)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i1 %0
}

; core::ptr::metadata::from_raw_parts_mut
; Function Attrs: inlinehint uwtable
define { [0 x i8]*, i64 } @_ZN4core3ptr8metadata18from_raw_parts_mut17ha28cdfd7ab6c33dfE({}* %data_address, i64 %metadata) unnamed_addr #0 {
start:
  %_4 = alloca { i8*, i64 }, align 8
  %_3 = alloca %"std::ptr::metadata::PtrRepr<[u8]>", align 8
  %0 = bitcast { i8*, i64 }* %_4 to {}**
  store {}* %data_address, {}** %0, align 8
  %1 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_4, i32 0, i32 1
  store i64 %metadata, i64* %1, align 8
  %2 = bitcast %"std::ptr::metadata::PtrRepr<[u8]>"* %_3 to { i8*, i64 }*
  %3 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_4, i32 0, i32 0
  %4 = load i8*, i8** %3, align 8
  %5 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_4, i32 0, i32 1
  %6 = load i64, i64* %5, align 8
  %7 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %2, i32 0, i32 0
  store i8* %4, i8** %7, align 8
  %8 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %2, i32 0, i32 1
  store i64 %6, i64* %8, align 8
  %9 = bitcast %"std::ptr::metadata::PtrRepr<[u8]>"* %_3 to { [0 x i8]*, i64 }*
  %10 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %9, i32 0, i32 0
  %11 = load [0 x i8]*, [0 x i8]** %10, align 8
  %12 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %9, i32 0, i32 1
  %13 = load i64, i64* %12, align 8
  %14 = insertvalue { [0 x i8]*, i64 } undef, [0 x i8]* %11, 0
  %15 = insertvalue { [0 x i8]*, i64 } %14, i64 %13, 1
  ret { [0 x i8]*, i64 } %15
}

; core::ptr::non_null::NonNull<T>::new_unchecked
; Function Attrs: inlinehint uwtable
define nonnull i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17hd57d0268af51bb34E"(i8* %ptr) unnamed_addr #0 {
start:
  %0 = alloca i8*, align 8
  store i8* %ptr, i8** %0, align 8
  %1 = load i8*, i8** %0, align 8, !nonnull !1
  ret i8* %1
}

; core::ptr::non_null::NonNull<T>::as_ptr
; Function Attrs: inlinehint uwtable
define i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h31874a9d6f4b8da5E"(i8* nonnull %self) unnamed_addr #0 {
start:
  ret i8* %self
}

; core::alloc::layout::Layout::from_size_align_unchecked
; Function Attrs: inlinehint uwtable
define internal { i64, i64 } @_ZN4core5alloc6layout6Layout25from_size_align_unchecked17h596c91086be58cd6E(i64 %size, i64 %align) unnamed_addr #0 {
start:
  %0 = alloca { i64, i64 }, align 8
; call core::num::nonzero::NonZeroUsize::new_unchecked
  %_4 = call i64 @_ZN4core3num7nonzero12NonZeroUsize13new_unchecked17h2dd7bb8c175ff335E(i64 %align), !range !2
  br label %bb1

bb1:                                              ; preds = %start
  %1 = bitcast { i64, i64 }* %0 to i64*
  store i64 %size, i64* %1, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  store i64 %_4, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 0
  %4 = load i64, i64* %3, align 8
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  %6 = load i64, i64* %5, align 8, !range !2
  %7 = insertvalue { i64, i64 } undef, i64 %4, 0
  %8 = insertvalue { i64, i64 } %7, i64 %6, 1
  ret { i64, i64 } %8
}

; core::alloc::layout::Layout::size
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core5alloc6layout6Layout4size17h3437f097831ce9deE({ i64, i64 }* align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %0 = bitcast { i64, i64 }* %self to i64*
  %1 = load i64, i64* %0, align 8
  ret i64 %1
}

; core::alloc::layout::Layout::align
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core5alloc6layout6Layout5align17h549cbf488619a574E({ i64, i64 }* align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 1
  %_2 = load i64, i64* %0, align 8, !range !2
; call core::num::nonzero::NonZeroUsize::get
  %1 = call i64 @_ZN4core3num7nonzero12NonZeroUsize3get17h82ccd89a31e7231eE(i64 %_2)
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %1
}

; <T as core::convert::Into<U>>::into
; Function Attrs: uwtable
define nonnull i8* @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17hbdbc7cc2bc8d4627E"(i8* nonnull %self) unnamed_addr #1 {
start:
; call <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from
  %0 = call nonnull i8* @"_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17hcf51b9cff811e285E"(i8* nonnull %self)
  br label %bb1

bb1:                                              ; preds = %start
  ret i8* %0
}

; alloc::vec::Vec<T,A>::as_mut_ptr
; Function Attrs: inlinehint uwtable
define i8* @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$10as_mut_ptr17h7d99e6d8829c4a5eE"(%"std::vec::Vec<u8>"* align 8 dereferenceable(24) %self) unnamed_addr #0 {
start:
  %_2 = bitcast %"std::vec::Vec<u8>"* %self to { i8*, i64 }*
; call alloc::raw_vec::RawVec<T,A>::ptr
  %ptr = call i8* @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$3ptr17h34fc76d75613ca64E"({ i8*, i64 }* align 8 dereferenceable(16) %_2)
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::mut_ptr::<impl *mut T>::is_null
  %_5 = call zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17hdd37fb0f130d6bc5E"(i8* %ptr)
  br label %bb2

bb2:                                              ; preds = %bb1
  %_4 = xor i1 %_5, true
  call void @llvm.assume(i1 %_4)
  br label %bb3

bb3:                                              ; preds = %bb2
  ret i8* %ptr
}

; alloc::alloc::dealloc
; Function Attrs: inlinehint uwtable
define internal void @_ZN5alloc5alloc7dealloc17hc7c4f1bc30ad983fE(i8* %ptr, i64 %0, i64 %1) unnamed_addr #0 {
start:
  %layout = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %3, align 8
; call core::alloc::layout::Layout::size
  %_4 = call i64 @_ZN4core5alloc6layout6Layout4size17h3437f097831ce9deE({ i64, i64 }* align 8 dereferenceable(16) %layout)
  br label %bb1

bb1:                                              ; preds = %start
; call core::alloc::layout::Layout::align
  %_6 = call i64 @_ZN4core5alloc6layout6Layout5align17h549cbf488619a574E({ i64, i64 }* align 8 dereferenceable(16) %layout)
  br label %bb2

bb2:                                              ; preds = %bb1
  call void @__rust_dealloc(i8* %ptr, i64 %_4, i64 %_6)
  br label %bb3

bb3:                                              ; preds = %bb2
  ret void
}

; alloc::raw_vec::RawVec<T,A>::current_memory
; Function Attrs: uwtable
define void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h7eb8ec5ed1f6acf7E"(%"std::option::Option<(std::ptr::NonNull<u8>, std::alloc::Layout)>"* noalias nocapture sret(%"std::option::Option<(std::ptr::NonNull<u8>, std::alloc::Layout)>") dereferenceable(24) %0, { i8*, i64 }* align 8 dereferenceable(16) %self) unnamed_addr #1 {
start:
  %1 = alloca i64, align 8
  %_13 = alloca { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }, align 8
  %_2 = alloca i8, align 1
  br label %bb4

bb4:                                              ; preds = %start
  %2 = icmp eq i64 1, 0
  br i1 %2, label %bb1, label %bb2

bb1:                                              ; preds = %bb4
  store i8 1, i8* %_2, align 1
  br label %bb3

bb2:                                              ; preds = %bb4
  %3 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %self, i32 0, i32 1
  %_5 = load i64, i64* %3, align 8
  %_4 = icmp eq i64 %_5, 0
  %4 = zext i1 %_4 to i8
  store i8 %4, i8* %_2, align 1
  br label %bb3

bb3:                                              ; preds = %bb1, %bb2
  %5 = load i8, i8* %_2, align 1, !range !3
  %6 = trunc i8 %5 to i1
  br i1 %6, label %bb5, label %bb6

bb6:                                              ; preds = %bb3
  store i64 1, i64* %1, align 8
  %7 = load i64, i64* %1, align 8
  br label %bb7

bb5:                                              ; preds = %bb3
  %8 = bitcast %"std::option::Option<(std::ptr::NonNull<u8>, std::alloc::Layout)>"* %0 to {}**
  store {}* null, {}** %8, align 8
  br label %bb12

bb12:                                             ; preds = %bb11, %bb5
  ret void

bb7:                                              ; preds = %bb6
  br label %bb8

bb8:                                              ; preds = %bb7
  %9 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %self, i32 0, i32 1
  %_9 = load i64, i64* %9, align 8
  %size = mul i64 1, %_9
; call core::alloc::layout::Layout::from_size_align_unchecked
  %10 = call { i64, i64 } @_ZN4core5alloc6layout6Layout25from_size_align_unchecked17h596c91086be58cd6E(i64 %size, i64 %7)
  %layout.0 = extractvalue { i64, i64 } %10, 0
  %layout.1 = extractvalue { i64, i64 } %10, 1
  br label %bb9

bb9:                                              ; preds = %bb8
  %11 = bitcast { i8*, i64 }* %self to i8**
  %_16 = load i8*, i8** %11, align 8, !nonnull !1
; call core::ptr::unique::Unique<T>::cast
  %_15 = call nonnull i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$4cast17h66d13f26e05dbe0fE"(i8* nonnull %_16)
  br label %bb10

bb10:                                             ; preds = %bb9
; call <T as core::convert::Into<U>>::into
  %_14 = call nonnull i8* @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17hbdbc7cc2bc8d4627E"(i8* nonnull %_15)
  br label %bb11

bb11:                                             ; preds = %bb10
  %12 = bitcast { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }* %_13 to i8**
  store i8* %_14, i8** %12, align 8
  %13 = getelementptr inbounds { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }, { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }* %_13, i32 0, i32 3
  %14 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %13, i32 0, i32 0
  store i64 %layout.0, i64* %14, align 8
  %15 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %13, i32 0, i32 1
  store i64 %layout.1, i64* %15, align 8
  %16 = bitcast %"std::option::Option<(std::ptr::NonNull<u8>, std::alloc::Layout)>"* %0 to %"std::option::Option<(std::ptr::NonNull<u8>, std::alloc::Layout)>::Some"*
  %17 = bitcast %"std::option::Option<(std::ptr::NonNull<u8>, std::alloc::Layout)>::Some"* %16 to { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }*
  %18 = bitcast { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }* %17 to i8*
  %19 = bitcast { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }* %_13 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %18, i8* align 8 %19, i64 24, i1 false)
  br label %bb12
}

; alloc::raw_vec::RawVec<T,A>::ptr
; Function Attrs: inlinehint uwtable
define i8* @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$3ptr17h34fc76d75613ca64E"({ i8*, i64 }* align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %0 = bitcast { i8*, i64 }* %self to i8**
  %_2 = load i8*, i8** %0, align 8, !nonnull !1
; call core::ptr::unique::Unique<T>::as_ptr
  %1 = call i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17hd7fa4ebeb4b5553aE"(i8* nonnull %_2)
  br label %bb1

bb1:                                              ; preds = %start
  ret i8* %1
}

; <alloc::alloc::Global as core::alloc::Allocator>::deallocate
; Function Attrs: inlinehint uwtable
define internal void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h833114f2a40c1909E"(%"std::alloc::Global"* nonnull align 1 %self, i8* nonnull %ptr, i64 %0, i64 %1) unnamed_addr #0 {
start:
  %layout = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %3, align 8
; call core::alloc::layout::Layout::size
  %_4 = call i64 @_ZN4core5alloc6layout6Layout4size17h3437f097831ce9deE({ i64, i64 }* align 8 dereferenceable(16) %layout)
  br label %bb1

bb1:                                              ; preds = %start
  %4 = icmp eq i64 %_4, 0
  br i1 %4, label %bb3, label %bb2

bb3:                                              ; preds = %bb1
  br label %bb6

bb2:                                              ; preds = %bb1
; call core::ptr::non_null::NonNull<T>::as_ptr
  %_6 = call i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h31874a9d6f4b8da5E"(i8* nonnull %ptr)
  br label %bb4

bb4:                                              ; preds = %bb2
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  %_8.0 = load i64, i64* %5, align 8
  %6 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  %_8.1 = load i64, i64* %6, align 8, !range !2
; call alloc::alloc::dealloc
  call void @_ZN5alloc5alloc7dealloc17hc7c4f1bc30ad983fE(i8* %_6, i64 %_8.0, i64 %_8.1)
  br label %bb5

bb5:                                              ; preds = %bb4
  br label %bb6

bb6:                                              ; preds = %bb3, %bb5
  ret void
}

; <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hd72c1a19dfb60615E"(%"std::vec::Vec<u8>"* align 8 dereferenceable(24) %self) unnamed_addr #1 {
start:
; call alloc::vec::Vec<T,A>::as_mut_ptr
  %_3 = call i8* @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$10as_mut_ptr17h7d99e6d8829c4a5eE"(%"std::vec::Vec<u8>"* align 8 dereferenceable(24) %self)
  br label %bb1

bb1:                                              ; preds = %start
  %0 = getelementptr inbounds %"std::vec::Vec<u8>", %"std::vec::Vec<u8>"* %self, i32 0, i32 3
  %_5 = load i64, i64* %0, align 8
; call core::ptr::slice_from_raw_parts_mut
  %1 = call { [0 x i8]*, i64 } @_ZN4core3ptr24slice_from_raw_parts_mut17h03de4e1e292c4e6bE(i8* %_3, i64 %_5)
  %_2.0 = extractvalue { [0 x i8]*, i64 } %1, 0
  %_2.1 = extractvalue { [0 x i8]*, i64 } %1, 1
  br label %bb2

bb2:                                              ; preds = %bb1
  br label %bb3

bb3:                                              ; preds = %bb2
  ret void
}

; <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hf4e56d07c00dc84cE"({ i8*, i64 }* align 8 dereferenceable(16) %self) unnamed_addr #1 {
start:
  %_2 = alloca %"std::option::Option<(std::ptr::NonNull<u8>, std::alloc::Layout)>", align 8
; call alloc::raw_vec::RawVec<T,A>::current_memory
  call void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h7eb8ec5ed1f6acf7E"(%"std::option::Option<(std::ptr::NonNull<u8>, std::alloc::Layout)>"* noalias nocapture sret(%"std::option::Option<(std::ptr::NonNull<u8>, std::alloc::Layout)>") dereferenceable(24) %_2, { i8*, i64 }* align 8 dereferenceable(16) %self)
  br label %bb1

bb1:                                              ; preds = %start
  %0 = bitcast %"std::option::Option<(std::ptr::NonNull<u8>, std::alloc::Layout)>"* %_2 to {}**
  %1 = load {}*, {}** %0, align 8
  %2 = icmp eq {}* %1, null
  %_4 = select i1 %2, i64 0, i64 1
  %3 = icmp eq i64 %_4, 1
  br i1 %3, label %bb3, label %bb2

bb3:                                              ; preds = %bb1
  %4 = bitcast %"std::option::Option<(std::ptr::NonNull<u8>, std::alloc::Layout)>"* %_2 to %"std::option::Option<(std::ptr::NonNull<u8>, std::alloc::Layout)>::Some"*
  %5 = bitcast %"std::option::Option<(std::ptr::NonNull<u8>, std::alloc::Layout)>::Some"* %4 to { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }*
  %6 = bitcast { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }* %5 to i8**
  %ptr = load i8*, i8** %6, align 8, !nonnull !1
  %7 = bitcast %"std::option::Option<(std::ptr::NonNull<u8>, std::alloc::Layout)>"* %_2 to %"std::option::Option<(std::ptr::NonNull<u8>, std::alloc::Layout)>::Some"*
  %8 = bitcast %"std::option::Option<(std::ptr::NonNull<u8>, std::alloc::Layout)>::Some"* %7 to { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }*
  %9 = getelementptr inbounds { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }, { [0 x i64], i8*, [0 x i64], { i64, i64 }, [0 x i64] }* %8, i32 0, i32 3
  %10 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %9, i32 0, i32 0
  %layout.0 = load i64, i64* %10, align 8
  %11 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %9, i32 0, i32 1
  %layout.1 = load i64, i64* %11, align 8, !range !2
  %_7 = bitcast { i8*, i64 }* %self to %"std::alloc::Global"*
; call <alloc::alloc::Global as core::alloc::Allocator>::deallocate
  call void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h833114f2a40c1909E"(%"std::alloc::Global"* nonnull align 1 %_7, i8* nonnull %ptr, i64 %layout.0, i64 %layout.1)
  br label %bb4

bb2:                                              ; preds = %bb1
  br label %bb5

bb5:                                              ; preds = %bb4, %bb2
  ret void

bb4:                                              ; preds = %bb3
  br label %bb5
}

; probe1::probe
; Function Attrs: uwtable
define void @_ZN6probe15probe17h4c635ac760564483E() unnamed_addr #1 {
start:
  %_11 = alloca i64*, align 8
  %_10 = alloca [1 x { i8*, i64* }], align 8
  %_3 = alloca %"std::fmt::Arguments", align 8
  %res = alloca %"std::string::String", align 8
  %_1 = alloca %"std::string::String", align 8
  store i64* bitcast (<{ [8 x i8] }>* @alloc5 to i64*), i64** %_11, align 8
  %arg0 = load i64*, i64** %_11, align 8, !nonnull !1
; call core::fmt::ArgumentV1::new
  %0 = call { i8*, i64* } @_ZN4core3fmt10ArgumentV13new17h9f64c2cca8ecd4d2E(i64* align 8 dereferenceable(8) %arg0, i1 (i64*, %"std::fmt::Formatter"*)* nonnull @"_ZN4core3fmt3num3imp55_$LT$impl$u20$core..fmt..LowerExp$u20$for$u20$isize$GT$3fmt17hb82b37371cdeb573E")
  %_14.0 = extractvalue { i8*, i64* } %0, 0
  %_14.1 = extractvalue { i8*, i64* } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
  %1 = bitcast [1 x { i8*, i64* }]* %_10 to { i8*, i64* }*
  %2 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %1, i32 0, i32 0
  store i8* %_14.0, i8** %2, align 8
  %3 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %1, i32 0, i32 1
  store i64* %_14.1, i64** %3, align 8
  %_7.0 = bitcast [1 x { i8*, i64* }]* %_10 to [0 x { i8*, i64* }]*
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117he4a9a1f377defa3cE(%"std::fmt::Arguments"* noalias nocapture sret(%"std::fmt::Arguments") dereferenceable(48) %_3, [0 x { [0 x i8]*, i64 }]* nonnull align 8 bitcast (<{ i8*, [8 x i8] }>* @alloc3 to [0 x { [0 x i8]*, i64 }]*), i64 1, [0 x { i8*, i64* }]* nonnull align 8 %_7.0, i64 1)
  br label %bb2

bb2:                                              ; preds = %bb1
; call alloc::fmt::format
  call void @_ZN5alloc3fmt6format17ha36d54910f7a7aa8E(%"std::string::String"* noalias nocapture sret(%"std::string::String") dereferenceable(24) %res, %"std::fmt::Arguments"* noalias nocapture dereferenceable(48) %_3)
  br label %bb3

bb3:                                              ; preds = %bb2
  %4 = bitcast %"std::string::String"* %_1 to i8*
  %5 = bitcast %"std::string::String"* %res to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %4, i8* align 8 %5, i64 24, i1 false)
; call core::ptr::drop_in_place<alloc::string::String>
  call void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h71ff5be47284ad5aE"(%"std::string::String"* %_1)
  br label %bb4

bb4:                                              ; preds = %bb3
  ret void
}

declare i32 @__CxxFrameHandler3(...) unnamed_addr #2

; Function Attrs: nofree nosync nounwind willreturn
declare void @llvm.assume(i1 noundef) #3

; Function Attrs: nounwind uwtable
declare void @__rust_dealloc(i8*, i64, i64) unnamed_addr #4

; Function Attrs: argmemonly nofree nosync nounwind willreturn
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #5

; core::fmt::num::imp::<impl core::fmt::LowerExp for isize>::fmt
; Function Attrs: uwtable
declare zeroext i1 @"_ZN4core3fmt3num3imp55_$LT$impl$u20$core..fmt..LowerExp$u20$for$u20$isize$GT$3fmt17hb82b37371cdeb573E"(i64* align 8 dereferenceable(8), %"std::fmt::Formatter"* align 8 dereferenceable(64)) unnamed_addr #1

; alloc::fmt::format
; Function Attrs: uwtable
declare void @_ZN5alloc3fmt6format17ha36d54910f7a7aa8E(%"std::string::String"* noalias nocapture sret(%"std::string::String") dereferenceable(24), %"std::fmt::Arguments"* noalias nocapture dereferenceable(48)) unnamed_addr #1

attributes #0 = { inlinehint uwtable "target-cpu"="x86-64" }
attributes #1 = { uwtable "target-cpu"="x86-64" }
attributes #2 = { "target-cpu"="x86-64" }
attributes #3 = { nofree nosync nounwind willreturn }
attributes #4 = { nounwind uwtable "target-cpu"="x86-64" }
attributes #5 = { argmemonly nofree nosync nounwind willreturn }
attributes #6 = { noinline }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{}
!2 = !{i64 1, i64 0}
!3 = !{i8 0, i8 2}
