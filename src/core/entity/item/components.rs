/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

use bevy::prelude::*;

#[allow(dead_code)]
#[derive(Component, Debug)]
pub struct InventoryItem;

#[derive(Component, Debug)]
pub struct InventoryName;

#[derive(Component, Debug)]
pub struct InventoryDescription;

#[derive(Component, Debug)]
pub struct InventorySlot;

#[derive(Component, Debug)]
pub struct InventoryStack;

#[derive(Component, Debug)]
pub struct InventoryMaxStack;

#[derive(Component, Debug)]
pub struct IsUsed;

#[derive(Component, Debug)]
pub struct IsDrop;

#[derive(Component, Debug)]
pub struct Untradeable;

