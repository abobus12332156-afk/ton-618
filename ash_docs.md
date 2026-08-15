Skip to main content

## Crate ash

## [ash](../ash/index.html)0.38.0+1.3.281

  * [All Items](all.html)

### Sections

  * Vulkan API
    * Examples
    * Getting started
    * Crate features

### Crate Items

  * Modules
  * Macros
  * Structs
  * Enums
  * Traits

# Crate ash Copy item path

[Source](../src/ash/lib.rs.html#1-216)

Expand description

## §Vulkan API

<https://registry.khronos.org/vulkan/specs/1.3-extensions/html/index.html>

### §Examples

    
    
    use ash::{vk, Entry};
    let entry = Entry::linked();
    let app_info = vk::ApplicationInfo {
        api_version: vk::make_api_version(0, 1, 0, 0),
        ..Default::default()
    };
    let create_info = vk::InstanceCreateInfo {
        p_application_info: &app_info,
        ..Default::default()
    };
    let instance = unsafe { entry.create_instance(&create_info, None)? };

### §Getting started

Load the Vulkan library linked at compile time using [`Entry::linked()`], or
load it at runtime using [`Entry::load()`](struct.Entry.html#method.load
"associated function ash::Entry::load"), which uses `libloading`. If you want
to perform entry point loading yourself, call
[`Entry::from_static_fn()`](struct.Entry.html#method.from_static_fn
"associated function ash::Entry::from_static_fn").

### §Crate features

  * **debug** (default): Whether Vulkan structs should implement `Debug`.
  * **loaded** (default): Support searching for the Vulkan loader manually at runtime.
  * **linked** : Link the Vulkan loader at compile time.
  * **std** (default): Whether ash depends on the standard library (otherwise `alloc` is required)

## Modules§

[amd](amd/index.html "mod ash::amd")

    Extensions tagged AMD
[amdx](amdx/index.html "mod ash::amdx")

    Extensions tagged AMDX
[android](android/index.html "mod ash::android")

    Extensions tagged ANDROID
[arm](arm/index.html "mod ash::arm")

    Extensions tagged ARM
[ext](ext/index.html "mod ash::ext")

    Extensions tagged EXT
[fuchsia](fuchsia/index.html "mod ash::fuchsia")

    Extensions tagged FUCHSIA
[ggp](ggp/index.html "mod ash::ggp")

    Extensions tagged GGP
[google](google/index.html "mod ash::google")

    Extensions tagged GOOGLE
[huawei](huawei/index.html "mod ash::huawei")

    Extensions tagged HUAWEI
[img](img/index.html "mod ash::img")

    Extensions tagged IMG
[intel](intel/index.html "mod ash::intel")

    Extensions tagged INTEL
[khr](khr/index.html "mod ash::khr")

    Extensions tagged KHR
[lunarg](lunarg/index.html "mod ash::lunarg")

    Extensions tagged LUNARG
[msft](msft/index.html "mod ash::msft")

    Extensions tagged MSFT
[mvk](mvk/index.html "mod ash::mvk")

    Extensions tagged MVK
[nn](nn/index.html "mod ash::nn")

    Extensions tagged NN
[nv](nv/index.html "mod ash::nv")

    Extensions tagged NV
[nvx](nvx/index.html "mod ash::nvx")

    Extensions tagged NVX
[prelude](prelude/index.html "mod ash::prelude")

[qcom](qcom/index.html "mod ash::qcom")

    Extensions tagged QCOM
[qnx](qnx/index.html "mod ash::qnx")

    Extensions tagged QNX
[sec](sec/index.html "mod ash::sec")

    Extensions tagged SEC
[util](util/index.html "mod ash::util")

[valve](valve/index.html "mod ash::valve")

    Extensions tagged VALVE
[vk](vk/index.html "mod ash::vk")

    Raw Vulkan bindings and types, generated from `vk.xml`

## Macros§

[define_handle](macro.define_handle.html "macro ash::define_handle")

[handle_nondispatchable](macro.handle_nondispatchable.html "macro
ash::handle_nondispatchable")

[match_in_struct](macro.match_in_struct.html "macro ash::match_in_struct")

    Given an immutable raw pointer to a type with an `s_type` member such as [`vk::BaseInStructure`](vk/struct.BaseInStructure.html "struct ash::vk::BaseInStructure"), match on a set of Vulkan structures. The struct will be rebound to the given variable of the type of the given Vulkan structure.
[match_out_struct](macro.match_out_struct.html "macro ash::match_out_struct")

    Given a mutable raw pointer to a type with an `s_type` member such as [`vk::BaseOutStructure`](vk/struct.BaseOutStructure.html "struct ash::vk::BaseOutStructure"), match on a set of Vulkan structures. The struct will be rebound to the given variable of the type of the given Vulkan structure.
[vk_bitflags_wrapped](macro.vk_bitflags_wrapped.html "macro
ash::vk_bitflags_wrapped")

## Structs§

[Device](struct.Device.html "struct ash::Device")

    <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDevice.html>
[DeviceFnV1_0](struct.DeviceFnV1_0.html "struct ash::DeviceFnV1_0")

    Raw Vulkan 1 device-level function pointers
[DeviceFnV1_1](struct.DeviceFnV1_1.html "struct ash::DeviceFnV1_1")

    Raw Vulkan 1.1 device-level function pointers
[DeviceFnV1_2](struct.DeviceFnV1_2.html "struct ash::DeviceFnV1_2")

    Raw Vulkan 1.2 device-level function pointers
[DeviceFnV1_3](struct.DeviceFnV1_3.html "struct ash::DeviceFnV1_3")

    Raw Vulkan 1.3 device-level function pointers
[Entry](struct.Entry.html "struct ash::Entry")

    Holds the Vulkan functions independent of a particular instance
[EntryFnV1_0](struct.EntryFnV1_0.html "struct ash::EntryFnV1_0")

    Raw Vulkan 1 entry point function pointers
[EntryFnV1_1](struct.EntryFnV1_1.html "struct ash::EntryFnV1_1")

    Raw Vulkan 1.1 entry point function pointers
[EntryFnV1_2](struct.EntryFnV1_2.html "struct ash::EntryFnV1_2")

    Raw Vulkan 1.2 entry point function pointers
[EntryFnV1_3](struct.EntryFnV1_3.html "struct ash::EntryFnV1_3")

    Raw Vulkan 1.3 entry point function pointers
[Instance](struct.Instance.html "struct ash::Instance")

    <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInstance.html>
[InstanceFnV1_0](struct.InstanceFnV1_0.html "struct ash::InstanceFnV1_0")

    Raw Vulkan 1 instance-level function pointers
[InstanceFnV1_1](struct.InstanceFnV1_1.html "struct ash::InstanceFnV1_1")

    Raw Vulkan 1.1 instance-level function pointers
[InstanceFnV1_2](struct.InstanceFnV1_2.html "struct ash::InstanceFnV1_2")

    Raw Vulkan 1.2 instance-level function pointers
[InstanceFnV1_3](struct.InstanceFnV1_3.html "struct ash::InstanceFnV1_3")

    Raw Vulkan 1.3 instance-level function pointers
[StaticFn](struct.StaticFn.html "struct ash::StaticFn")

    Raw Vulkan 1 static function pointers

## Enums§

[LoadingError](enum.LoadingError.html "enum ash::LoadingError")

## Traits§

[RawPtr](trait.RawPtr.html "trait ash::RawPtr")

