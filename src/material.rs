/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   material.rs                                        :+:    :+:            */
/*                                                     +:+                    */
/*   By: nmartins <nmartins@student.codam.nl>         +#+                     */
/*                                                   +#+                      */
/*   Created: 2019/07/20 19:44:22 by nmartins       #+#    #+#                */
/*   Updated: 2019/07/20 20:09:05 by nmartins      ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

use crate::shape::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Material {
	pub color: Vec3,
}